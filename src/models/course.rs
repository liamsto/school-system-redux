use std::fmt::Display;

use sqlx::FromRow;
use uuid::Uuid;

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
    pub async fn get_prerequisites(&self, pool: &sqlx::PgPool) -> Result<Vec<Course>, sqlx::Error> {
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

    pub async fn add_prerequisites(
        &self,
        pool: &sqlx::PgPool,
        prerequisites: Vec<CoursePrerequisite>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for prereq in prerequisites {
            sqlx::query!(
                r#"
                INSERT INTO course_prerequisites (course_id, prerequisite_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                prereq.course_id,
                prereq.prerequisite_id
            )
            .execute(&mut *tx)
            .await?;
        }
        Ok(())
    }

    pub async fn delete(
        &self,
        pool: &sqlx::PgPool
    ) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            DELETE FROM courses WHERE id = $1
            "#,
            self.id
        ).execute(pool).await?;
        Ok(())
    }
}


impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\nUUID:{}\nDepartment: {}\nTitle: {}\nDescription: {:#?}\nCredits: {}", self.course_number, self.id, self.department_id, self.title, self.description, self.credits)?;

        Ok(())
    }
}

#[derive(Debug, FromRow)]
pub struct CoursePrerequisite {
    course_id: Uuid,
    prerequisite_id: Uuid,
}

pub fn create_prerequisite(course_id: Uuid, prerequisite_id: Uuid) -> CoursePrerequisite {
    CoursePrerequisite {
        course_id,
        prerequisite_id,
    }
}
