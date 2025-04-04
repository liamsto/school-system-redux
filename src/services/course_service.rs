use uuid::Uuid;

use crate::models::{course::Course, course_meeting_time::CourseMeetingTime};

pub async fn get_course_by_id(
    id: Uuid,
    pool: &sqlx::PgPool,
) -> Result<Option<Course>, sqlx::Error> {
    let course = sqlx::query_as!(
        Course,
        r#"
        SELECT id, department_id, course_number, title, description, credits FROM courses WHERE id=$1
        "#,
        id
    ).fetch_optional(pool).await?;
    Ok(course)
}

pub async fn get_meeting_times_by_id(
    course_id: Uuid,
    pool: &sqlx::PgPool,
) -> Result<Vec<CourseMeetingTime>, sqlx::Error> {
    Ok(sqlx::query_as!(
        CourseMeetingTime,
        r#"
        SELECT * FROM course_meeting_times WHERE offering_id=$1
        "#,
        course_id
    )
    .fetch_all(pool)
    .await?)
}
