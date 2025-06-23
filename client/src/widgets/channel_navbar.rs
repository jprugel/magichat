use iced::Element;
use iced::Length;
use iced::widget::{button, column, container, text};
use protocol::Channel;

#[derive(Debug, Clone, Default)]
pub struct ChannelNavbar {
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChannelSelected(String),
}

pub fn view(state: &ChannelNavbar) -> Element<Message> {
    state
        .clone()
        .channels
        .into_iter()
        .fold(column![], |col, channel| {
            col.push(container(
                button(text(format!("# {}", channel.name.clone())))
                    .style(button::secondary)
                    .width(Length::Fill)
                    .on_press(Message::ChannelSelected(channel.name.clone())),
            ))
        })
        .into()
}
