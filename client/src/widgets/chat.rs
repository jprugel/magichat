use crate::widgets::user_message;
use iced::Element;
use iced::Length;
use iced::widget::{Rule, column, container, float, horizontal_rule, scrollable, text, text_input};
use protocol::{Channel, Server};

#[derive(Debug, Clone)]
pub enum Message {
    Updated(String),
    Submitted,
    View(user_message::Message),
}

#[derive(Debug, Clone, Default)]
pub struct Chat {
    pub server: Server,
    pub channel: Channel,
    pub written_text: String,
}

impl Chat {
    pub fn view(&self) -> Element<Message> {
        let title_bar = float(text(self.channel.name.clone()));
        let rule: Rule = horizontal_rule(1);
        let text_log = self.channel.log.iter().fold(column![], |col, msg| {
            col.push(user_message::view(msg).map(Message::View))
        });

        let scroll = scrollable(text_log).height(Length::Fill).anchor_bottom();

        let text_input = float(
            text_input("Enter text here..", &self.written_text)
                .on_input(Message::Updated)
                .on_submit(Message::Submitted),
        );

        container(column![title_bar, rule, scroll, text_input]).into()
    }
}
