mod models;
mod security;
mod services;
mod tests;
mod ui;

use sqlx::PgPool;
use ui::app::{title_fn, update_fn, view_fn, App};

const DATABASE_URL: &str = "postgres://postgres:example@localhost/schooldb";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    // Create the database pool.
    let pool = PgPool::connect(DATABASE_URL).await?;

    // Launch the application using the builder API.
    iced::application(title_fn, update_fn, view_fn).run_with(move || App::new(pool)).expect("Failed to launch the app.");


    Ok(())
}
