use iced::{
    Alignment, Element, Padding,
    widget::{Button, Column, Text, TextInput},
};

/// Messages for the login view.
#[derive(Debug, Clone)]
pub enum Message {
    EmailChanged(String),
    PasswordChanged(String),
    LoginClicked,
    RegisterClicked,
    LoginResult(Result<bool, String>),
}

/// Login state holding email, password, and login status.
#[derive(Default)]
pub struct LoginState {
    pub email: String,
    pub password: String,
    pub login_status: Option<String>,
}

impl LoginState {
    /// Builds the login UI.
    pub fn view(&self) -> Element<Message> {
        let email_input = TextInput::new("Email", &self.email)
            .on_input(Message::EmailChanged)
            .padding(10);

        let password_input = TextInput::new("Password", &self.password)
            .on_input(Message::PasswordChanged)
            .secure(true) // Masks the input.
            .padding(10);

        let login_button = Button::new(Text::new("Login"))
            .on_press(Message::LoginClicked)
            .padding(10);

        let register_button = Button::new(Text::new("Register"))
            .on_press(Message::RegisterClicked)
            .padding(10);

        let mut content = Column::new()
            .spacing(20)
            .align_x(Alignment::Center)
            .padding(Padding::from([250, 100]))
            .push(email_input)
            .push(password_input)
            .push(login_button)
            .push(register_button);

        if let Some(ref status) = self.login_status {
            content = content.push(Text::new(status));
        }

        content.into()
    }
}
