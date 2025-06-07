use iced::Element;
use iced::widget::{column, container, text, text_input, vertical_space};
use iced::widget::text::Shaping;

#[derive(Debug, Clone)]
pub enum Message {
    UserUpdated(String),
    UserSubmitted,
}

#[derive(Debug, Clone, Default)]
pub struct Chat {
    pub text_log: Vec<String>,
    pub written_text: String,
}

pub fn view(chat: &Chat) -> Element<Message> {
    let text_log = chat
        .text_log
        .iter()
        .fold(column![], |col, msg| col.push(text(msg).shaping(Shaping::Advanced)));
    let space = vertical_space();
    let text_input = text_input("Enter text here..", &chat.written_text)
        .on_input(Message::UserUpdated)
        .on_submit(Message::UserSubmitted);

    container(column![text_log, space, text_input]).into()
}
