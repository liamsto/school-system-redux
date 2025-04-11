mod models;
mod security;
mod services;
mod tests;
mod ui;

use sqlx::PgPool;
use ui::app::{App, title_fn, update_fn, view_fn};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");

    // Create the database pool.
    let pool = PgPool::connect(&database_url).await?;
    let handle = tokio::runtime::Handle::current();
    // Launch the application using the builder API.
    iced::application(title_fn, update_fn, view_fn)
        .run_with(move || App::new(pool, handle))
        .expect("Failed to launch the app.");

    Ok(())
}
