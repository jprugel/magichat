use iced::{Alignment, Element, Length};
use iced::widget::{row, column, container, text, svg};
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

pub fn view(state: &UserMessage) -> Element<Message> {
    let icon = svg("client/assets/user_icon.svg")
        .height(45)
        .width(45);
    
    container(
        row![
            container(icon)
                .align_y(Alignment::Center),
            container(
                column![
                    container(
                        text(&state.user.username).shaping(Shaping::Advanced).style(text::success),
                    ), 
                    container(
                        text(&state.content).shaping(Shaping::Advanced)
                    )
                ]
            )
        ].height(50)
    ).into()
}