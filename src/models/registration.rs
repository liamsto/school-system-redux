use std::fmt;

use sqlx::{types::chrono::{DateTime, Utc}, PgPool};
use uuid::Uuid;



pub struct Registration {
    pub id: Uuid,
    pub student_id: Option<Uuid>,
    pub offering_id: Option<Uuid>,
    pub registered_at: Option<DateTime<Utc>>,
    pub status: RegistrationStatus,
    pub grade: Option<Grade>
}

impl Registration {
    fn new(student_id: Option<Uuid>, offering_id: Option<Uuid>, status: RegistrationStatus) -> Self {
        Registration { id: Uuid::new_v4(), student_id, offering_id, registered_at: None, status, grade: None }
    }

    //pain in the ass
    async fn insert(self, pool: &PgPool) -> Result<Registration, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO registrations (id, student_id, offering_id, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, student_id, offering_id, registered_at, status, grade
            "#,
            self.id,
            self.student_id,
            self.offering_id,
            self.status.to_string()
        )
        .fetch_one(pool)
        .await?;
        
        // Convert the raw row into our Registration, mapping the Option<String> grade into Option<Grade>. Because Rust doesn't want to implement From<Option<String>> for Option<Grade>
        let registration = Registration {
            id: row.id,
            student_id: row.student_id,
            offering_id: row.offering_id,
            registered_at: row.registered_at,
            status: row.status.into(), // String -> RegistrationStatus
            grade: row.grade.map(|g| g.into()), // Option<String> -> Option<Grade>
        };
        Ok(registration)
    }

    pub async fn create(student_id: Option<Uuid>, offering_id: Option<Uuid>, status: RegistrationStatus, pool: &PgPool) -> Result<Registration, sqlx::Error> {
        Ok(Self::new(student_id, offering_id, status).insert(pool).await?)
    }
}


pub enum RegistrationStatus {
    Registered,
    Dropped,
    Waitlisted
}

impl From<String> for RegistrationStatus {
    fn from(value: String) -> Self {
        match value.trim() {
            "registered" => RegistrationStatus::Registered,
            "dropped" => RegistrationStatus::Dropped,
            "waitlisted" => RegistrationStatus::Waitlisted,
            _ => panic!("Invalid registration status in database!")
        }
    }
}

impl fmt::Display for RegistrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg_string = match self {
            RegistrationStatus::Registered => "Registered",
            RegistrationStatus::Dropped => "Dropped",
            RegistrationStatus::Waitlisted => "Waitlisted"
        };
        write!(f, "{}", reg_string)
    }
}

pub enum Grade {
    A,
    B,
    C,
    D,
    F
}

impl From<String> for Grade {
    fn from(value: String) -> Self {
        match value.trim() {
            "A" => Grade::A,
            "B" => Grade::B,
            "C" => Grade::C,
            "D" => Grade::D,
            "F" => Grade::F,
            _ => panic!("Invalid grade found in database!")
        }
    }
}


impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grade_str = match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::F => "F"
        };
        write!(f,  "{}", grade_str)
    }
}