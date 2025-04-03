use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;

#[derive(sqlx::FromRow)]
pub struct Term {
    pub id: Option<i32>,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Term {
    async fn insert(self, pool: &PgPool) -> Result<Term, sqlx::Error> {
        let term = sqlx::query_as!(
            Term,
            r#"
            INSERT INTO terms (name, start_date, end_date)
            VALUES ($1, $2, $3)
            RETURNING id, name, start_date, end_date
            "#,
            self.name,
            self.start_date,
            self.end_date
        )
        .fetch_one(pool)
        .await?;

        Ok(term)
    }

    fn new(name: String, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self, String> {
        if start_date >= end_date {
            return Err("start_date must be before end_date".to_string());
        }

        Ok(Term {
            id: None,
            name,
            start_date,
            end_date,
        })
    }
}

pub async fn create_term(
    name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    pool: &sqlx::PgPool,
) -> Result<Term, sqlx::Error> {
    let term = Term::new(name, start_date, end_date).map_err(|err| sqlx::Error::Protocol(err))?;
    term.insert(pool).await
}
