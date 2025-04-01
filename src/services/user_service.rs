use sqlx::PgPool;

use crate::models::user::User;

pub async fn insert_user(user: &User, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, hashed_password, first_name, last_name, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user.id,
        user.email,
        user.hashed_password,
        user.first_name,
        user.last_name,
        user.role
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_user(user: &User, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM users WHERE id=$1        
        "#,
        user.id
    )
    .execute(pool)
    .await?;

    Ok(())
}
