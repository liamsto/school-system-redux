use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct Department {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl Department {
    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
        DELETE FROM departments WHERE id = $1
        "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            INSERT INTO departments (id, code, name)
            VALUES ($1, $2, $3)
            "#,
            self.id,
            self.code,
            self.name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub fn create_department(id: i32, code: String, name: String) -> Department {
    Department { id, code, name }
}
