use rand::Rng;
use std::fmt::{self};

use sqlx::PgPool;
use uuid::Uuid;

pub struct StudentProfile {
    user_id: Uuid,
    student_id: String,
    enrollment_year: i32,
    major: StudentMajor,
}

impl StudentProfile {
    fn new(user_id: Uuid, enrollment_year: i32, major: StudentMajor) -> StudentProfile {
        StudentProfile {
            user_id,
            student_id: Self::generate_student_id(),
            enrollment_year,
            major,
        }
    }

    async fn insert(self, pool: &PgPool) -> Result<StudentProfile, sqlx::Error> {
        let student_profile = sqlx::query_as!(
            StudentProfile,
            r#"
            INSERT INTO student_profiles (user_id, student_id, enrollment_year, major)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, student_id, enrollment_year, major
            "#,
            self.user_id,
            self.student_id,
            self.enrollment_year,
            self.major.to_string()
        )
        .fetch_one(pool)
        .await?;
        Ok(student_profile)
    }

    pub async fn create(
        user_id: Uuid,
        enrollment_year: i32,
        major: StudentMajor,
        pool: &PgPool,
    ) -> Result<StudentProfile, sqlx::Error> {
        Ok(Self::new(user_id, enrollment_year, major)
            .insert(pool)
            .await?)
    }

    fn generate_student_id() -> String {
        let mut rng = rand::rng();
        rng.random_range(10_000_000..=99_999_999).to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudentMajor {
    ComputerScience,
    Engineering,
    Biology,
    Mathematics,
    Physics,
    Psychology,
    Sociology,
    Politics,
    Literature,
    Business,
    FineArts,
    Nursing,
    Education,
}

impl fmt::Display for StudentMajor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major_str = match self {
            StudentMajor::ComputerScience => "Computer Science",
            StudentMajor::Engineering => "Engineering",
            StudentMajor::Biology => "Biology",
            StudentMajor::Mathematics => "Mathematics",
            StudentMajor::Physics => "Physics",
            StudentMajor::Psychology => "Psychology",
            StudentMajor::Sociology => "Sociology",
            StudentMajor::Politics => "Politics",
            StudentMajor::Literature => "Literature",
            StudentMajor::Business => "Business",
            StudentMajor::FineArts => "Fine Arts",
            StudentMajor::Nursing => "Nursing",
            StudentMajor::Education => "Education",
        };
        write!(f, "{}", major_str)
    }
}

impl From<String> for StudentMajor {
    fn from(s: String) -> Self {
        match s.trim() {
            "Computer Science" => StudentMajor::ComputerScience,
            "Engineering" => StudentMajor::Engineering,
            "Biology" => StudentMajor::Biology,
            "Mathematics" => StudentMajor::Mathematics,
            "Physics" => StudentMajor::Physics,
            "Psychology" => StudentMajor::Psychology,
            "Sociology" => StudentMajor::Sociology,
            "Politics" => StudentMajor::Politics,
            "Literature" => StudentMajor::Literature,
            "Business" => StudentMajor::Business,
            "Fine Arts" => StudentMajor::FineArts,
            "Nursing" => StudentMajor::Nursing,
            "Education" => StudentMajor::Education,
            _ => panic!("Invalid student major string: {}", s),
        }
    }
}
