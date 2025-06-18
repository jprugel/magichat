use iced::{Alignment, Element, Length};
use iced::advanced::text::Wrapping;
use iced::widget::{row, column, container, text, svg, horizontal_space, text_input};
use iced::widget::text::Shaping;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server_info::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub user: User,
    pub content: String,
    pub channel: String,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl UserMessage {
    pub fn view(&self) -> Element<Message> {
        let icon = svg("client/assets/user_icon.svg")
            .height(45)
            .width(45);

        container(
            row![
            container(icon)
                .align_y(Alignment::Center),
            horizontal_space().width(7),
            container(
                column![
                    container(
                        text(&self.user.username).shaping(Shaping::Advanced).style(text::success),
                    ), 
                    container(
                        text(&self.content).shaping(Shaping::Advanced),
                    ).width(Length::Fill),
                ]
            ).width(Length::Fill).style(container::secondary),
        ].width(Length::Fill).padding(3),
        ).style(container::bordered_box).into()
    }
}
