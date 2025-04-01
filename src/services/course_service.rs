use uuid::Uuid;

use crate::models::course::Course;

pub async fn get_course_by_id(id: Uuid, pool: &sqlx::PgPool) -> Result<Option<Course>, sqlx::Error> {
    let course = sqlx::query_as!(
        Course,
        r#"
        SELECT id, department_id, course_number, title, description, credits FROM courses WHERE id=$1
        "#,
        id
    ).fetch_optional(pool).await?;
    Ok(course)
}