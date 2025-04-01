use crate::models::department::Department;

pub async fn get_department_by_code(code: &str, pool: &sqlx::PgPool) -> Result<Option<Department>, sqlx::Error> {
    let department = sqlx::query_as!(
        Department,
        r#"
        SELECT id, code, name FROM departments WHERE code=$1
        "#,
        code
    ).fetch_optional(pool).await?;

    Ok(department)
}