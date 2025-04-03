use std::{
    fmt::{self, Display},
    str::FromStr,
};

use sqlx::{FromRow, types::chrono::NaiveTime};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CourseMeetingTime {
    pub id: Option<i32>,
    pub offering_id: Uuid,
    pub day_of_week: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

impl CourseMeetingTime {
    fn new(
        offering_id: Uuid,
        day_of_week: Weekday,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> CourseMeetingTime {
        CourseMeetingTime {
            id: None,
            offering_id,
            day_of_week,
            start_time,
            end_time,
        }
    }

    async fn insert(self, pool: &sqlx::PgPool) -> Result<CourseMeetingTime, sqlx::Error> {
        let course_meeting_time = sqlx::query_as!(
            CourseMeetingTime,
            r#"
            INSERT INTO course_meeting_times (offering_id, day_of_week, start_time, end_time)
            VALUES($1, $2, $3, $4)
            RETURNING id, offering_id, day_of_week, start_time, end_time
            "#,
            self.offering_id,
            self.day_of_week.to_string(),
            self.start_time,
            self.end_time
        )
        .fetch_one(pool)
        .await?;

        Ok(course_meeting_time)
    }

    pub async fn create(
        offering_id: Uuid,
        day_of_week: Weekday,
        start_time: NaiveTime,
        end_time: NaiveTime,
        pool: &sqlx::PgPool,
    ) -> Result<CourseMeetingTime, sqlx::Error> {
        let course_meeting_time =
            CourseMeetingTime::new(offering_id, day_of_week, start_time, end_time);
        Ok(course_meeting_time.insert(pool).await?)
    }

    pub async fn delete(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            self,
            r#"
            DELETE FROM course_meeting_times WHERE id=$1
            "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Display for CourseMeetingTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ID: {}\nOffering ID: {}\nWeekday: {}\nStart Time: {}\nEnd Time: {}\n",
            self.id.unwrap(),
            self.offering_id,
            self.day_of_week,
            self.start_time,
            self.end_time
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Invalid,
}

impl FromStr for Weekday {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(Weekday::Monday),
            "tuesday" => Ok(Weekday::Tuesday),
            "wednesday" => Ok(Weekday::Wednesday),
            "thursday" => Ok(Weekday::Thursday),
            "friday" => Ok(Weekday::Friday),
            _ => Err(format!("Invalid weekday {}.", s)),
        }
    }
}

impl From<std::string::String> for Weekday {
    fn from(value: std::string::String) -> Self {
        match value.to_lowercase().as_str() {
            "monday" => Weekday::Monday,
            "tuesday" => Weekday::Tuesday,
            "wednesday" => Weekday::Wednesday,
            "thursday" => Weekday::Thursday,
            "friday" => Weekday::Friday,
            _ => Weekday::Invalid, //ideally we never hit this branch. sqlx insists on using From<> instead of TryFrom<>, but we only allow inserting things from the enum so we should be good
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day_str = match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Invalid => "Invalid",
        };
        write!(f, "{}", day_str)
    }
}
