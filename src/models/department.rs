use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, FromRow)]
pub struct Department {
    pub serial: i32,
    pub code: String,
    pub name: String,
}

impl Department {
    pub async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

pub fn create_department(serial: i32, code: String, name: String) -> Department {
    Department { serial, code, name }
}
