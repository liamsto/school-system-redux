use std::fmt::Display;

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::course_prerequisite::CoursePrerequisite;

#[derive(Debug, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub department_id: i32,
    pub course_number: String,
    pub title: String,
    pub description: Option<String>,
    pub credits: i32,
}

pub fn create_course(
    id: Uuid,
    department_id: i32,
    course_number: String,
    title: String,
    description: Option<String>,
    credits: i32,
) -> Course {
    Course {
        id,
        department_id,
        course_number,
        title,
        description,
        credits,
    }
}

// Generally speaking, something should go in the impl block unless it spans multiple entites (e.g enrolling a user in a course), has nontrivial I/O, or uses transactions
impl Course {
    pub async fn prerequisites(&self, pool: &PgPool) -> Result<Vec<Course>, sqlx::Error> {
        let prerequisites = sqlx::query_as!(
            Course,
            r#"
                SELECT c.id, c.department_id, c.course_number, c.title, c.description, c.credits
                FROM course_prerequisites cp
                JOIN courses c ON cp.prerequisite_id = c.id
                WHERE cp.course_id = $1
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(prerequisites)
    }

    pub async fn add_prerequisite(
        &self,
        pool: &sqlx::PgPool,
        prerequisite: &CoursePrerequisite,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO course_prerequisites (course_id, prerequisite_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            prerequisite.course_id,
            prerequisite.prerequisite_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn remove_prerequisite(
        &self,
        pool: &sqlx::PgPool,
        prerequisite: &CoursePrerequisite,
    ) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            CoursePrerequisite,
            r#"
            DELETE FROM course_prerequisites WHERE course_id=$1 AND prerequisite_id=$2
            "#,
            prerequisite.course_id,
            prerequisite.prerequisite_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            DELETE FROM courses WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            INSERT INTO courses (id, department_id, course_number, title, description, credits)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.id,
            self.department_id,
            self.course_number,
            self.title,
            self.description,
            self.credits
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Returns the number of students currently registered for this course.
    pub async fn current_enrollment(&self, pool: &PgPool) -> Result<i64, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM registrations r
            JOIN course_offerings co ON r.offering_id = co.id
            WHERE co.course_id = $1 AND r.status = 'registered'
            "#,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(record
            .count
            .expect("Should be able to count the number of registered students."))
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\nUUID:{}\nDepartment: {}\nTitle: {}",
            self.course_number, self.id, self.department_id, self.title
        )?;

        if let Some(desc) = &self.description {
            writeln!(f, "Description: {}", desc)?;
        } else {
            writeln!(f, "Description: None")?;
        }

        writeln!(f, "Credits: {}", self.credits)
    }
}
