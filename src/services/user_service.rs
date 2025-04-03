use uuid::Uuid;

use crate::models::user::User;

pub async fn get_user_hash(user: User, pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    let user = sqlx::query!(r#"SELECT hashed_password FROM users WHERE id=$1"#, user.id)
        .fetch_one(pool)
        .await?;
    Ok(user.hashed_password)
}

pub async fn get_user_by_id(id: Uuid, pool: &sqlx::PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, hashed_password, first_name, last_name, role FROM users WHERE id=$1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
