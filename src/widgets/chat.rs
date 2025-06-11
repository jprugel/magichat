use iced::Element;
use iced::widget::text::Shaping;
use iced::widget::{column, container, horizontal_rule, text, text_input, vertical_space, Rule};
use crate::widgets::user_message;
use crate::server_info::{
    ServerInfo, Channel,
};

#[derive(Debug, Clone)]
pub enum Message {
    UserUpdated(String),
    UserSubmitted,
    MessageView(user_message::Message)
}

#[derive(Debug, Clone, Default)]
pub struct Chat {
    pub server: ServerInfo,
    pub channel: Channel,
    pub written_text: String,
}

pub fn view(chat: &Chat) -> Element<Message> {
    let title_bar = text(chat.channel.name.clone());
    let rule: Rule = horizontal_rule(1);
    let text_log = chat
        .channel
        .log
        .iter()
        .fold(column![], |col, msg| col.push(user_message::view(msg).map(Message::MessageView)));

    let space = vertical_space();

    let text_input = text_input("Enter text here..", &chat.written_text)
        .on_input(Message::UserUpdated)
        .on_submit(Message::UserSubmitted);

    container(column![title_bar, rule, text_log, space, text_input]).into()
}
