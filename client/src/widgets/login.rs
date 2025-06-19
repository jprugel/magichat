use crate::action::Action;
use iced::Padding;
use iced::Theme;
use iced::border::*;
use iced::widget::{column, container, text, text_input};
use iced::{Element};
use tracing::info;

#[derive(Debug, Clone)]
pub enum Message {
    UpdatedUsername(String),
    Submitted,
}

pub enum Instruction {
    Login { username: String },
}

#[derive(Debug, Clone, Default)]
pub struct Login {
    pub username: String,
}

impl Login {
    pub fn view(&self) -> Element<Message> {
        let text = container(text("LOGIN").center()).padding(Padding {
            top: 50.,
            bottom: 50.,
            left: 80.,
            ..Default::default()
        });

        let username = text_input("Enter username...", &self.username)
            .on_input(Message::UpdatedUsername)
            .on_submit(Message::Submitted);

        let username_container = container(username).padding([10, 0]);

        let style = |theme: &Theme| container::Style {
            border: iced::Border {
                width: 2.,
                radius: Radius::new(10.),
                ..iced::Border::default()
            },
            ..container::rounded_box(theme)
        };

        container(column![text, username_container])
            .align_x(iced::alignment::Horizontal::Center)
            .style(style)
            .height(400)
            .width(300)
            .padding([0, 50])
            .into()
    }

    pub fn update(&mut self, msg: Message) -> Action<Instruction, Message> {
        match msg {
            Message::UpdatedUsername(username) => {
                self.username = username;
                Action::none()
            }
            Message::Submitted => {
                info!("Submitted username: {}", self.username);
                Action::instruction(Instruction::Login {
                    username: self.username.clone(),
                })
            }
        }
    }
}
