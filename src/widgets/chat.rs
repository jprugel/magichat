use iced::Element;
use iced::widget::text::Shaping;
use iced::widget::{column, container, horizontal_rule, scrollable, text, text_input, vertical_space, Rule, float};
use crate::widgets::user_message;
use iced::Length;
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
    let title_bar = float(text(chat.channel.name.clone()));
    let rule: Rule = horizontal_rule(1);
    let text_log = chat
        .channel
        .log
        .iter()
        .fold(column![], |col, msg| col.push(msg.view().map(Message::MessageView)));
    
    let scroll = scrollable(text_log).height(Length::Fill).anchor_bottom();
    

    let text_input = float(text_input("Enter text here..", &chat.written_text)
        .on_input(Message::UserUpdated)
        .on_submit(Message::UserSubmitted));

    container(column![title_bar, rule, scroll, text_input]).into()
}
