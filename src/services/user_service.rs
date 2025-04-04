use sqlx::{PgPool, query_scalar};
use uuid::Uuid;

use crate::{
    models::{
        student_profile::{StudentMajor, StudentProfile},
        user::{self, FullName, Role, User},
    },
    security::password::{self, validate_password},
};

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
        SELECT id, email, hashed_password, first_name, last_name, role, created_at FROM users WHERE id=$1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Returns a user by searching for their email. Returns `None` if the email is not found in the database.
async fn get_user_by_email(email: String, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1 LIMIT 1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

/// The same as `get_user_by_email`, but slightly faster and returns only a String.
async fn get_user_hash_by_email(
    email: String,
    pool: &PgPool,
) -> Result<Option<String>, sqlx::Error> {
    let hash = query_scalar!(
        r#"
        SELECT hashed_password FROM users WHERE email = $1 LIMIT 1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(hash)
}
/// Registers a new user. This inserts an entry into both the `users` and `student_profiles` tables at the same time.
pub async fn register_student(
    email: String,
    plaintext_password: String,
    name: FullName,
    enrollment_year: i32,
    major: StudentMajor,
    pool: &PgPool,
) -> Result<(User, StudentProfile), sqlx::Error> {
    let hashed_password = password::hash_password(&plaintext_password)
        .expect("Password hashing function should not fail!");
    let user = User::create_user(email, hashed_password, name, Role::Student, pool).await?;
    let student_profile = StudentProfile::create(user.id, enrollment_year, major, pool).await?;
    Ok((user, student_profile))
}

/// Registers a new admin. Since admins are only referenced in the `users` table, this just hashes the plaintext and wraps the `create_user` function.
pub async fn register_admin(
    email: String,
    plaintext_password: String,
    name: FullName,
    pool: &PgPool,
) -> Result<User, sqlx::Error> {
    let hashed_password = password::hash_password(&plaintext_password)
        .expect("Password hashing function should not fail!");
    let user = User::create_user(email, hashed_password, name, Role::Admin, pool).await?;
    Ok(user)
}

/// Attempts to login a user based on email and plaintext password. Will return `false` if the user does not exist or the password was incorrect.
pub async fn try_login(
    email: &String,
    plaintext: &String,
    pool: &sqlx::PgPool,
) -> Result<bool, sqlx::Error> {
    let hash = match get_user_hash_by_email(email.to_string(), pool).await? {
        Some(hash) => hash,
        None => return Ok(false),
    };

    let valid = validate_password(&plaintext, &hash).unwrap_or(false);

    if !valid {
        return Ok(false);
    }

    Ok(true)
}
