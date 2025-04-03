use std::fmt::Display;

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CourseOffering {
    pub id: Uuid,
    pub course_id: Uuid,
    pub term_id: i32,
    pub instructor_id: Uuid,
    pub capacity: i32,
    pub location: String,
}

impl CourseOffering {
    fn new(
        course_id: Uuid,
        term_id: i32,
        instructor_id: Uuid,
        capacity: i32,
        location: String,
    ) -> Result<Self, String> {
        if capacity < 0 {
            return Err("Capacity must be greater than 0!".to_string());
        }
        Ok(CourseOffering {
            id: Uuid::new_v4(),
            course_id,
            term_id,
            instructor_id,
            capacity,
            location,
        })
    }

    async fn insert(self, pool: &PgPool) -> Result<CourseOffering, sqlx::Error> {
        let course_offering = sqlx::query_as!(
            CourseOffering,
            r#"
            INSERT INTO course_offerings (course_id, term_id, instructor_id, capacity, location)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, course_id, term_id, instructor_id, capacity, location
            "#,
            self.course_id,
            self.term_id,
            self.instructor_id,
            self.capacity,
            self.location
        )
        .fetch_one(pool)
        .await?;

        Ok(course_offering)
    }

    pub async fn create(
        course_id: Uuid,
        term_id: i32,
        instructor_id: Uuid,
        capacity: i32,
        location: String,
        pool: &PgPool,
    ) -> Result<CourseOffering, sqlx::Error> {
        let course_offering =
            CourseOffering::new(course_id, term_id, instructor_id, capacity, location)
                .map_err(|err| sqlx::Error::Protocol(err))?;
        course_offering.insert(pool).await
    }
}

impl Display for CourseOffering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID: {}\nTerm ID: {}\nInstructor ID: {}\nCapacity: {}\nLocation: {}",
            self.course_id, self.term_id, self.instructor_id, self.capacity, self.location
        )
    }
}
