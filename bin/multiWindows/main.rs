mod modal;
pub use modal::{Modal, ModalMessage};
use iced::widget::{
    button, center, stack, column, container, horizontal_space, row, text,mouse_area,opaque};
use iced::{Bottom, Element,Fill,Task};

#[derive(Default)]
struct App {
    modal: Modal,
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal,
    HideModal,
    ModalMessage(ModalMessage),
}

impl App {

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowModal => {
                self.modal.toggle();
                Task::none()
            }
            Message::HideModal => {
                self.modal.toggle();
                Task::none()
            }
            Message::ModalMessage(modal_message) => {
                self.modal.update(modal_message);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = container(
            column![
                row![text("Top Left"), horizontal_space(), text("Top Right")].height(Fill),
                center(button(text("Show Modal")).on_press(Message::ShowModal)),
                row![text("Bottom Left"), horizontal_space(), text("Bottom Right")]
                    .align_y(Bottom)
                    .height(Fill),
            ]
            .height(Fill),
        )
        .padding(10);

        let modal_view = self.modal.view().map(|modal_msg| Message::ModalMessage(modal_msg));
        if self.modal.is_visible {
            modal(content, modal_view, Message::HideModal)
        } else {
            content.into()
        }
    }
}


fn main() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .run()
}


fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(content.into()))
            .on_press(on_blur)
        )
    ]
    .into()
}