use iced::Border;
use iced::Element;
use iced::Length;
use iced::Padding;
use iced::Theme;
use iced::border::*;
use iced::widget::{column, container, text, text_input};

#[derive(Debug, Clone)]
pub enum Message {
    UpdatedUsername(String),
    UpdatedUrl(String),
    Submitted,
}

#[derive(Debug, Clone, Default)]
pub struct Login {
    pub username: String,
    pub url: String,
    pub submitted: bool,
}

pub fn view(login: &Login) -> Element<Message> {
    let text = container(text("LOGIN").center()).padding(Padding {
        top: 50.,
        bottom: 50.,
        left: 80.,
        ..Default::default()
    });

    let username = text_input("Enter username...", &login.username)
        .on_input(Message::UpdatedUsername)
        .on_submit(Message::Submitted);

    let username_container = container(username).padding([10, 0]);

    let url = text_input("Enter url...", &login.url)
        .on_input(Message::UpdatedUrl)
        .on_submit(Message::Submitted);

    let url_container = container(url).padding([10, 0]);

    let style = |theme: &Theme| container::Style {
        border: iced::Border {
            width: 2.,
            radius: Radius::new(10.),
            ..iced::Border::default()
        },
        ..container::rounded_box(theme)
    };

    container(column![text, username_container, url_container])
        .align_x(iced::alignment::Horizontal::Center)
        .style(style)
        .height(400)
        .width(300)
        .padding([0, 50])
        .into()
}
