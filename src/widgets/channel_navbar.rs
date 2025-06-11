use iced::Element;
use iced::widget::{
    container, 
    column, 
    text,
    button
};
use iced::Length;
use crate::server_info::Channel;

#[derive(Debug, Clone, Default)]
pub struct ChannelNavbar {
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChannelSelected(String)
}

pub fn view(state: &ChannelNavbar) -> Element<Message> {
    state
        .clone()
        .channels
        .into_iter()
        .fold(column![], |col, channel| col.push(container(button(text(channel.name.clone())).width(Length::Fill).on_press(Message::ChannelSelected(channel.name.clone())))))
        .into()
}