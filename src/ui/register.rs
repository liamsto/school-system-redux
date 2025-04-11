use iced::{
    Alignment, Element, Padding,
    widget::{Button, Column, Text, TextInput},
};

// Registration view messages
#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    RegisterClicked,
    RegisterResult(Result<bool, String>),
    BackToLogin,
}

/// Registration state holding registration fields.
#[derive(Default)]
pub struct RegisterState {
    pub name: String,
    pub email: String,
    pub password: String,
    pub register_status: Option<String>,
}

impl RegisterState {
    /// Builds the registration UI.
    pub fn view(&self) -> Element<Message> {
        let name_input = TextInput::new("Name", &self.name)
            .on_input(Message::NameChanged)
            .padding(10);

        let email_input = TextInput::new("Email", &self.email)
            .on_input(Message::EmailChanged)
            .padding(10);

        let password_input = TextInput::new("Password", &self.password)
            .on_input(Message::PasswordChanged)
            .secure(true)
            .padding(10);

        let register_button = Button::new(Text::new("Register"))
            .on_press(Message::RegisterClicked)
            .padding(10);

        let back_button = Button::new(Text::new("Back to Login"))
            .on_press(Message::BackToLogin)
            .padding(10);

        let mut content = Column::new()
            .spacing(20)
            .align_x(Alignment::Center)
            .padding(Padding::from([250, 100]))
            .push(name_input)
            .push(email_input)
            .push(password_input)
            .push(register_button)
            .push(back_button);

        if let Some(ref status) = self.register_status {
            content = content.push(Text::new(status));
        }

        content.into()
    }
}
