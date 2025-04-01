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
}

#[derive(Debug)]
pub struct NewDepartment {
    pub code: String,
    pub name: String,
}

impl NewDepartment {
    pub async fn insert(self, pool: &PgPool) -> Result<Department, sqlx::Error> {
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


pub fn create_department(code: String, name: String) -> NewDepartment {
    NewDepartment { code, name }
}
