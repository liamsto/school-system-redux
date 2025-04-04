use std::fmt::Display;

use sqlx::{
    FromRow,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

use super::course::Course;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
    pub created_at: Option<DateTime<Utc>>,
}

impl User {
    fn new(id: Uuid, email: String, hashed_password: String, name: FullName, role: Role) -> User {
        let first_name = name.first;
        let last_name = name.last;
        User {
            id,
            email,
            hashed_password,
            first_name,
            last_name,
            role,
            created_at: None,
        }
    }

    pub async fn create_user(
        email: String,
        hashed_password: String,
        name: FullName,
        role: Role,
        pool: &sqlx::PgPool,
    ) -> Result<User, sqlx::Error> {
        let user = Self::new(Uuid::new_v4(), email, hashed_password, name, role);
        user.insert(pool).await?;
        Ok(user)
    }

    async fn insert(&self, pool: &sqlx::PgPool) -> Result<User, sqlx::Error> {
        let record = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, hashed_password, first_name, last_name, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, hashed_password, first_name, last_name, role, created_at
            "#,
            self.id,
            self.email,
            self.hashed_password,
            self.first_name,
            self.last_name,
            self.role.to_string()
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    pub async fn delete(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            DELETE FROM users WHERE id=$1
            "#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Given a user, get all the courses the are registered in. Note that this means actually registered, not waitlisted or dropped.
    pub async fn get_registered_courses(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Course>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Course,
            r#"
            SELECT 
            c.id, 
            c.department_id, 
            c.course_number, 
            c.title, 
            c.description, 
            c.credits
            FROM registrations r
            JOIN course_offerings co ON r.offering_id = co.id
            JOIN courses c ON co.course_id = c.id
            WHERE r.student_id = $1 AND r.status = 'registered'
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?)
    }

    /// Retrieves  all the courses a user has a registration for, irrespective of whether they are `registered`, `dropped`, or `waitlisted`
    pub async fn get_all_user_courses(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<Vec<Course>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Course,
            r#"
            SELECT 
            c.id, 
            c.department_id, 
            c.course_number, 
            c.title, 
            c.description, 
            c.credits
            FROM registrations r
            JOIN course_offerings co ON r.offering_id = co.id
            JOIN courses c ON co.course_id = c.id
            WHERE r.student_id = $1
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?)
    }

    
}

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Student,
    Admin,
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.trim() {
            "student" => Role::Student,
            "admin" => Role::Admin,
            _ => panic!("Nonexistant role found in DB!"),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::Student => "student",
            Role::Admin => "admin",
        };
        write!(f, "{}", role_str)
    }
}

pub struct FullName {
    pub first: String,
    pub last: String,
}

impl FullName {
    pub fn new(first: &str, last: &str) -> FullName {
        FullName {
            first: first.to_string(),
            last: last.to_string(),
        }
    }
}
