use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

pub fn create_user(
    id: Uuid,
    email: String,
    hashed_password: String,
    first_name: String,
    last_name: String,
    role: String,
) -> User {
    User {
        id,
        email,
        hashed_password,
        first_name,
        last_name,
        role,
    }
}
