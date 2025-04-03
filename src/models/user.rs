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

impl User {
    async fn insert(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            INSERT INTO users (id, email, hashed_password, first_name, last_name, role)
            VALUES($1, $2, $3, $4, $5, $6)
            "#,
            self.id,
            self.email,
            self.hashed_password,
            self.first_name,
            self.last_name,
            self.role
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            DELETE FROM users WHERE id=$1
            "#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

fn new(
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

pub async fn create_user(
    email: String,
    hashed_password: String,
    first_name: String,
    last_name: String,
    role: String,
    pool: &sqlx::PgPool,
) -> Result<User, sqlx::Error> {
    let user = new(
        Uuid::new_v4(),
        email,
        hashed_password,
        first_name,
        last_name,
        role,
    );
    user.insert(pool).await?;
    Ok(user)
}
