use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct Department {
    pub id: Option<i32>,
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

    async fn insert(self, pool: &PgPool) -> Result<Department, sqlx::Error> {
        let department = sqlx::query_as!(
            Department,
            r#"
            INSERT INTO departments (code, name)
            VALUES ($1, $2)
            RETURNING id, code, name
            "#,
            self.code,
            self.name
        )
        .fetch_one(pool)
        .await?;
    
        Ok(department)
    }

}

/// Creates a new instance of `Department`. Note that the ID will be initalized to `None`, so this function is private to prevent misuse of the struct.
fn new(code:  String, name: String) -> Department {
    Department { id: None, code, name }
}


pub async fn create_department(code: String, name: String, pool: &PgPool) -> Result<Department, sqlx::Error> {
    let department = new(code, name);
    Ok(department.insert(pool).await?)
}