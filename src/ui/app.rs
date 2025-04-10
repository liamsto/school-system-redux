use iced::{Element, Task};
use sqlx::PgPool;

use crate::services::user_service;
use crate::ui::login::{LoginState, Message as LoginMessage};

/// Unified message for the application.
#[derive(Debug, Clone)]
pub enum Message {
    Login(LoginMessage),
}

impl From<LoginMessage> for Message {
    fn from(msg: LoginMessage) -> Self {
        Message::Login(msg)
    }
}

/// The application state.
pub struct App {
    login_state: LoginState,
    pool: PgPool,
    handle: tokio::runtime::Handle,
}

impl App {
    /// Creates a new instance of our application.
    pub fn new(pool: PgPool, handle: tokio::runtime::Handle) -> (Self, Task<Message>) {
        (
            Self {
                login_state: LoginState::default(),
                pool,
                handle,
            },
            Task::none(),
        )
    }
    /// Returns the window title.
    pub fn title(&self) -> String {
        String::from("Login - SchoolRedux")
    }    

    /// Processes incoming messages.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(login_message) => match login_message {
                LoginMessage::EmailChanged(email) => {
                    self.login_state.email = email;
                    Task::none()
                }
                LoginMessage::PasswordChanged(password) => {
                    self.login_state.password = password;
                    Task::none()
                }
                LoginMessage::LoginClicked => {
                    let email = self.login_state.email.clone();
                    let password = self.login_state.password.clone();
                    let pool = self.pool.clone();
                    let handle = self.handle.clone();

                    // Launch the asynchronous backend login function, converting sqlx::Error into String.
                    Task::perform(
                        async move {
                            // Spawn the login future on the Tokio runtime.
                            let join_handle = handle.spawn(async move {
                                user_service::try_login(&email, &password, &pool).await
                            });
                            // Await the result from the tokio task.
                            join_handle.await
                                .expect("Tokio task panicked")
                                .map_err(|e| e.to_string())
                        },
                        |result| LoginMessage::LoginResult(result).into(),
                    )                
                }
                LoginMessage::LoginResult(result) => {
                    // Update the login status with a String message.
                    self.login_state.login_status = match result {
                        Ok(true) => Some("Login successful".into()),
                        Ok(false) => Some("Invalid credentials".into()),
                        Err(e) => Some(format!("An error occurred: {}", e)),
                    };
                    Task::none()
                }
            },
        }
    }

    /// Constructs the view.
    pub fn view(&self) -> Element<Message> {
        self.login_state.view().map(Message::Login)
    }
}


pub fn title_fn(app: &App) -> String {
    app.title()
}

pub fn update_fn(app: &mut App, message: Message) -> Task<Message> {
    app.update(message)
}

pub fn view_fn(app: &App) -> Element<Message> {
    app.view()
}