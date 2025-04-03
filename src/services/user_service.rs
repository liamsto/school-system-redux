use crate::models::user::User;

pub async fn get_user_hash(user: User, pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    let user = sqlx::query!(r#"SELECT hashed_password FROM users WHERE id=$1"#, user.id)
        .fetch_one(pool)
        .await?;
    Ok(user.hashed_password)
}
