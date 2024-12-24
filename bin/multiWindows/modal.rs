use iced::widget::{button, column, container, text_input, text};
use iced::Element;

#[derive(Debug, Clone)]
pub enum ModalMessage {
    Submit,
    Hide,
    Email(String),
    Password(String),
}

pub struct Modal {
    pub email: String,
    pub password: String,
    pub is_visible: bool,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            is_visible: false,
        }
    }
}

impl Modal {
    pub fn view(&self) -> Element<ModalMessage> {
        if self.is_visible {
            container(
                column![
                    text("Sign Up").size(24),
                    column![
                        text("Email").size(12),
                        text_input("abc@123.com", &self.email)
                            .on_input(ModalMessage::Email)
                            .on_submit(ModalMessage::Submit)
                            .padding(5),
                    ]
                    .spacing(5),
                    column![
                        text("Password").size(12),
                        text_input("", &self.password)
                            .on_input(ModalMessage::Password)
                            .on_submit(ModalMessage::Submit)
                            .secure(true)
                            .padding(5),
                    ]
                    .spacing(5),
                    button(text("Submit")).on_press(ModalMessage::Hide),
                ]
                .spacing(10),
            )
            .width(300)
            .padding(10)
            .style(container::rounded_box)
            .into()
        } else {
            container(column![]).into()
        }
    }

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn update(&mut self, message: ModalMessage) {
        match message {
            ModalMessage::Submit => {
                if !self.email.is_empty() && !self.password.is_empty() {
                    self.toggle(); 
                }
            }
            ModalMessage::Hide => self.toggle(),
            ModalMessage::Email(email) => self.email = email,
            ModalMessage::Password(password) => self.password = password,
        }
    }
}