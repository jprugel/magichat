use iced::Element;
use iced::widget::{ container, text};
use iced_split::{Split, Strategy};
use crate::widgets::navbar;
use crate::widgets::chat;

pub struct State {
    pub split_at_sc: f32,
    pub split_at_cc: f32,
    pub navbar: navbar::Navbar,
    pub chat: chat::Chat,
}

#[derive(Debug, Clone)]
pub enum Message {
    ResizeSC(f32),
    ResizeCC(f32),
    Navbar(navbar::Message),
    Chat(chat::Message),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let server_channel_split = Split::new(
            navbar::view(&state.navbar).map(|msg| Message::Navbar(msg)), 
            text("B"), 
            state.split_at_sc, 
            Message::ResizeSC
        ).strategy(Strategy::Start);

    let channel_chat_split = Split::new(
        server_channel_split, 
        chat::view(&state.chat).map(|msg| Message::Chat(msg)),
        state.split_at_cc, 
        Message::ResizeCC
    ).strategy(Strategy::Start);
    let container = container(channel_chat_split);
    container.into()
}
