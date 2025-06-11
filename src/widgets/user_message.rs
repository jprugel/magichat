use iced::Element;
use iced::widget::{row, column, container, text};
use iced::widget::text::Shaping;
use serde::{Deserialize, Serialize};
use crate::server_info::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub user: User,
    pub content: String,
    pub channel: String,
}

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view(state: &UserMessage) -> Element<Message> {
    container(
        row![
            container("Icon"), 
            container(
                column![
                    container(
                        text(&state.user.username).shaping(Shaping::Advanced)
                    ), 
                    container(
                        text(&state.content).shaping(Shaping::Advanced)
                    )
                ]
            )
        ]
    ).into()
}