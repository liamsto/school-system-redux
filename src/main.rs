use std::io::{self, Write};

use models::{
    course::create_course,
    course_meeting_time::{CourseMeetingTime, Weekday},
    course_offering::CourseOffering,
    department::create_department,
    term::Term,
    user::{FullName, Role, User},
};
use security::password::hash_password;
use services::{course_service::get_course_by_id, user_service::try_login};
use sqlx::{
    postgres::PgPoolOptions,
    types::chrono::{NaiveDate, NaiveTime},
};
use uuid::Uuid;

mod models;
mod security;
mod services;
mod tests;

const DATABASE_URL: &str = "postgres://postgres:example@localhost/schooldb";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    println!("Welcome to the user creation program.");

    let name = loop {
        let full_name = read_input("Enter your first and last name: ");
        match parse_name(&full_name) {
            Some(name) => break name,
            None => println!("Invalid name format. Enter a valid first and last name."),
        }
    };

    let email = read_input("Enter your email: ");
    let password = read_input("Enter a secure password: ");
    let hashed_password = hash_password(&password).expect("Password hashing failed");

    let role = Role::Student;

    let test_user = User::create_user(email, hashed_password, name, role, &pool).await?;

    println!("Now try logging in!");

    loop {
        let email = read_input("Enter your email: ");
        let password = read_input("Enter your password: ");
        let login_success = try_login(&email, &password, &pool).await?;
        if login_success {
            break;
        }
    }

    println!("User created");
    test_user.delete(&pool).await?;
    println!("User deleted!");

    let hashed_password = hash_password("testpassword").unwrap();
    let professor = User::create_user(
        "joe.smith@gmail.com".to_string(),
        hashed_password,
        FullName::new("Joe", "Smith"),
        Role::Admin,
        &pool,
    )
    .await?;

    let compsci =
        create_department("COSC".to_string(), "Computer Science".to_string(), &pool).await?;

    let cosc101 = create_course(
        Uuid::new_v4(),
        compsci
            .id
            .expect("A department with no ID is present in database!"),
        "COSC101".to_string(),
        "Intro to Computer Science".to_string(),
        Some("A basic intro to Java".to_string()),
        3,
    );
    cosc101.insert(&pool).await?;

    let course_search = get_course_by_id(cosc101.id, &pool).await?.unwrap();
    println!("{}", course_search);

    println!("Creating course offering.");

    let term = Term::create_term(
        NaiveDate::from_ymd_opt(2024, 9, 6).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 20).unwrap(),
        &pool,
    )
    .await?;

    let cosc101_term1 = CourseOffering::create(
        cosc101.id,
        term.id.unwrap(),
        professor.id,
        100,
        "ART103".to_string(),
        &pool,
    )
    .await?;

    println!("{}", cosc101_term1);

    let cosc101_meeting = CourseMeetingTime::create(
        cosc101_term1.id,
        Weekday::Monday,
        NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
        NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        &pool,
    )
    .await?;

    println!("{}", cosc101_meeting);

    cosc101.delete(&pool).await?;
    compsci.delete(&pool).await?;
    professor.delete(&pool).await?;
    cosc101_meeting.delete(&pool).await?;
    Ok(())
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Readline panicked!");
    buf.trim().to_string()
}

fn parse_name(input: &str) -> Option<FullName> {
    let mut parts = input.trim().splitn(2, ' ');
    let first = parts.next()?;
    let last = parts.next()?;
    Some(FullName {
        first: first.to_string(),
        last: last.to_string(),
    })
}
