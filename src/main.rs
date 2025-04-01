use std::io::{self, Write};

use models::{course::create_course, department::create_department, user};
use security::password::hash_password;
use services::{
    course_service::get_course_by_id,
    user_service::{delete_user, insert_user},
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

mod models;
mod security;
mod services;

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

    let (first_name, last_name) = loop {
        let full_name = read_input("Enter your first and last name:");
        match parse_name(&full_name) {
            Some((first, last)) => break (first, last),
            None => println!("Invalid name format. Enter a valid first and last name."),
        }
    };

    let email = read_input("Enter your email:");
    let password = read_input("Enter a secure password:");
    let hashed_password = hash_password(&password).expect("Password hashing failed");

    let role = "student".to_string();

    let test_user = user::create_user(
        Uuid::new_v4(),
        email,
        hashed_password,
        first_name,
        last_name,
        role,
    );
    insert_user(&test_user, &pool).await?;
    println!("User created");
    delete_user(&test_user, &pool).await?;
    println!("User deleted!");

    let compsci = create_department(1, "COSC".to_string(), "Computer Science".to_string());
    compsci.insert(&pool).await?;
    let cosc101 = create_course(
        Uuid::new_v4(),
        compsci.id,
        "COSC101".to_string(),
        "Intro to Computer Science".to_string(),
        Some("A basic intro to Java".to_string()),
        3,
    );
    cosc101.insert(&pool).await?;

    println!("{}", cosc101);
    let course_search = get_course_by_id(cosc101.id, &pool).await?.unwrap();
    println!("{}", course_search);
    cosc101.delete(&pool).await?;

    Ok(())
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Readline panicked!");
    buf.trim().to_string()
}

fn parse_name(input: &str) -> Option<(String, String)> {
    let mut parts = input.trim().splitn(2, ' ');
    let first = parts.next()?;
    let last = parts.next()?;
    Some((first.to_string(), last.to_string()))
}
