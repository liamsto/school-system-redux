use crate::ui::login::{LoginState, Message as LoginMessage};
use crate::ui::register::{RegisterState, Message as RegisterMessage};
use iced::Element;

#[derive(Debug, Clone)]
pub enum PageMessage {
    // A unified message that wraps both login and register messages.
    Login(LoginMessage),
    Register(RegisterMessage),
}

/// All possible pages.
pub enum Page {
    Login(LoginState),
    Register(RegisterState),
}

impl Page {
    /// Returns an Element representing the current page’s UI.
    pub fn view(&self) -> Element<PageMessage> {
        match self {
            Page::Login(login_state) => login_state.view().map(PageMessage::Login),
            Page::Register(register_state) => register_state.view().map(PageMessage::Register),
        }
    }
}