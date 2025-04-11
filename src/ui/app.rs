use iced::{Element, Task};
use sqlx::PgPool;
use tokio::runtime::Handle;

use crate::models::student_profile::StudentMajor;
use crate::models::user::FullName;
use crate::ui::login;
use crate::ui::page::{Page, PageMessage};
use crate::ui::register;

#[derive(Debug, Clone)]
pub enum Message {
    Page(PageMessage),
}

// The application state now holds a current page.
pub struct App {
    pub current_page: Page,
    pub pool: PgPool,
    pub handle: Handle,
}

impl App {
    /// Creates a new instance of our application.
    pub fn new(pool: PgPool, handle: Handle) -> (Self, Task<Message>) {
        (
            Self {
                current_page: Page::Login(login::LoginState::default()),
                pool,
                handle,
            },
            Task::none(),
        )
    }

    /// Returns the window title.
    pub fn title(&self) -> String {
        String::from("SchoolRedux")
    }

    /// Processes incoming messages.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        //this is terrible and will need a better way of doing this
        match message {
            Message::Page(page_msg) => match page_msg {
                // --- Login page messages ---
                crate::ui::page::PageMessage::Login(login_msg) => {
                    match login_msg {
                        login::Message::EmailChanged(email) => {
                            if let Page::Login(ref mut login_state) = self.current_page {
                                login_state.email = email;
                            }
                            Task::none()
                        }
                        login::Message::PasswordChanged(password) => {
                            if let Page::Login(ref mut login_state) = self.current_page {
                                login_state.password = password;
                            }
                            Task::none()
                        }
                        login::Message::LoginClicked => {
                            if let Page::Login(ref login_state) = self.current_page {
                                let email = login_state.email.clone();
                                let password = login_state.password.clone();
                                let pool = self.pool.clone();
                                let handle = self.handle.clone();
                                // Perform login asynchronously.
                                Task::perform(
                                    async move {
                                        let join_handle = handle.spawn(async move {
                                            crate::services::user_service::try_login(
                                                &email, &password, &pool,
                                            )
                                            .await
                                        });
                                        join_handle
                                            .await
                                            .expect("Tokio task panicked")
                                            .map_err(|e| e.to_string())
                                    },
                                    |result| {
                                        // Wrap the returned PageMessage into a Message
                                        Message::Page(crate::ui::page::PageMessage::Login(
                                            login::Message::LoginResult(result),
                                        ))
                                    },
                                )
                            } else {
                                Task::none()
                            }
                        }
                        login::Message::LoginResult(result) => {
                            if let Page::Login(ref mut login_state) = self.current_page {
                                login_state.login_status = match result {
                                    Ok(true) => Some("Login successful".into()),
                                    Ok(false) => Some("Invalid credentials".into()),
                                    Err(e) => Some(format!("An error occurred: {}", e)),
                                };
                            }
                            Task::none()
                        }
                        login::Message::RegisterClicked => {
                            // Transition to the registration page.
                            self.current_page = Page::Register(register::RegisterState::default());
                            Task::none()
                        }
                    }
                }
                // --- Registration page messages ---
                crate::ui::page::PageMessage::Register(register_msg) => {
                    match register_msg {
                        register::Message::BackToLogin => {
                            // back to the login page.
                            self.current_page = Page::Login(login::LoginState::default());
                            Task::none()
                        }
                        register::Message::EmailChanged(email) => {
                            if let Page::Register(ref mut register_state) = self.current_page {
                                register_state.email = email;
                            }
                            Task::none()
                        }
                        register::Message::PasswordChanged(password) => {
                            if let Page::Register(ref mut register_state) = self.current_page {
                                register_state.password = password;
                            }
                            Task::none()
                        }
                        register::Message::NameChanged(name) => {
                            if let Page::Register(ref mut register_state) = self.current_page {
                                register_state.name = name;
                            }
                            Task::none()
                        }
                        register::Message::RegisterClicked => {
                            if let Page::Register(ref register_state) = self.current_page {
                                let email = register_state.email.clone();
                                let password = register_state.password.clone();
                                let pool = self.pool.clone();
                                let handle = self.handle.clone();
                                Task::perform(
                                    async move {
                                        let join_handle = handle.spawn(async move {
                                            crate::services::user_service::register_student(
                                                email,
                                                password,
                                                FullName::new("test", "test"),
                                                1,
                                                StudentMajor::Mathematics,
                                                &pool,
                                            )
                                            .await
                                        });
                                        join_handle
                                            .await
                                            .expect("Tokio task panicked")
                                            .map(|(_user, _profile)| true) // Convert the (User, StudentProfile) to Ok(true)
                                            .map_err(|e| e.to_string())
                                    },
                                    |result| {
                                        Message::Page(crate::ui::page::PageMessage::Register(
                                            register::Message::RegisterResult(result),
                                        ))
                                    },
                                )
                            } else {
                                Task::none()
                            }
                        }

                        register::Message::RegisterResult(result) => {
                            if let Page::Register(ref mut register_state) = self.current_page {
                                register_state.register_status = match result {
                                    Ok(true) => Some("Login successful".into()),
                                    Ok(false) => Some("Failed to register.".into()),
                                    Err(e) => Some(format!("An error occurred: {}", e)),
                                };
                            }
                            Task::none()
                        }
                    }
                }
            },
        }
    }

    /// Constructs the view.
    pub fn view(&self) -> Element<Message> {
        self.current_page.view().map(Message::Page)
    }
}

// Free functions required by iced::application.

pub fn title_fn(app: &App) -> String {
    app.title()
}

pub fn update_fn(app: &mut App, message: Message) -> Task<Message> {
    app.update(message)
}

pub fn view_fn(app: &App) -> Element<Message> {
    app.view()
}
