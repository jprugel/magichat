use crate::server_info::*;
use crate::widgets::chat;
use crate::widgets::navbar;
use iced::Element;
use iced::widget::{column, container, text, text_input};
use iced_dialog::dialog;
use iced_split::{Split, Strategy};

pub struct State {
    pub split_at_sc: f32,
    pub split_at_cc: f32,
    pub navbar: navbar::Navbar,
    pub chat: chat::Chat,
    pub open_dialog: bool,
    pub server_address: String,
    pub server_addresses: Vec<ServerInfo>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ResizeSC(f32),
    ResizeCC(f32),
    Navbar(navbar::Message),
    Chat(chat::Message),
    ServerAddressUpdate(String),
    ServerAddressSubmit,
    CloseDialog,
}

pub fn view(state: &State) -> Element<'_, Message> {
    let server_channel_split = Split::new(
        navbar::view(&state.navbar).map(|msg| Message::Navbar(msg)),
        text("B"),
        state.split_at_sc,
        Message::ResizeSC,
    )
    .strategy(Strategy::Start);

    let channel_chat_split = Split::new(
        server_channel_split,
        chat::view(&state.chat).map(|msg| Message::Chat(msg)),
        state.split_at_cc,
        Message::ResizeCC,
    )
    .strategy(Strategy::Start);

    let container = container(channel_chat_split);

    let dialog_content = column![
        text_input("Enter server address...", &state.server_address)
            .on_input(Message::ServerAddressUpdate)
            .on_submit(Message::ServerAddressSubmit)
    ];

    dialog(state.open_dialog, container, dialog_content)
        .title("Add Server")
        .push_button(iced_dialog::button("Add", Message::ServerAddressSubmit))
        .push_button(iced_dialog::button("Cancel", Message::CloseDialog))
        .width(350)
        .height(234)
        .into()
}
