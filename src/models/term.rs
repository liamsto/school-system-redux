use std::fmt;

use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;
use chrono::Datelike;

#[derive(sqlx::FromRow)]
pub struct Term {
    pub id: Option<i32>,
    pub name: TermName,
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
            self.name.to_string(),
            self.start_date,
            self.end_date
        )
        .fetch_one(pool)
        .await?;

        Ok(term)
    }

    fn new(name: TermName, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self, String> {
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
    
    /// Creates and inserts a new Term into the database. Using only `start_date` and `end_date` as parameters ensures we only ever have valid terms inserted into the database.
    pub async fn create_term(
        start_date: NaiveDate,
        end_date: NaiveDate,
        pool: &sqlx::PgPool,
    ) -> Result<Term, sqlx::Error> {
        let term = Term::new(Term::generate_name(start_date), start_date, end_date).map_err(|err| sqlx::Error::Protocol(err))?;
        term.insert(pool).await
    }

    /// Generates a TermName for the term. 
    fn generate_name(start_date: NaiveDate) -> TermName {
        let semester = if start_date.month() >= 9 {
            Semester::Winter
        } else {
            Semester::Summer
        };

        TermName {
            semester,
            year: extract_year(start_date),
        }
    }
    
}



struct TermName {
    pub semester: Semester,
    pub year: String 
}


impl fmt::Display for TermName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.semester, self.year)
    }
}

impl From<String> for TermName {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.trim().split_whitespace().collect();
        if parts.len() != 2 {
            panic!("Invalid term name format. Expected format: 'SEMESTER YEAR'");
        }

        let semester = match parts[0].to_lowercase().as_str() {
            "summer" => Semester::Summer,
            "winter" => Semester::Winter,
            _ => panic!("Invalid semester value in term name"),
        };

        TermName {
            semester,
            year: parts[1].to_string(),
        }
    }
}


enum Semester  {
    Summer,
    Winter
}

impl fmt::Display  for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let semester_str = match self {
            Semester::Summer => "Summer",
            Semester::Winter => "Winter"
        };
        write!(f, "{}", semester_str)
    }
}

impl From<String> for Semester {
    fn from(value: String) -> Self {
        match value.trim().to_lowercase().as_str() {
            "summer" => Semester::Summer,
            "winter" => Semester::Winter,
            _ => panic!("Invalid semester value: {}", value),
        }
    }
}


fn extract_year(date: NaiveDate) -> String {
    date.format("%Y").to_string()
}