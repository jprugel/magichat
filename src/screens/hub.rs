use crate::server_info::*;
use crate::widgets::chat;
use crate::widgets::server_navbar;
use crate::widgets::channel_navbar;
use iced::Element;
use iced::widget::{column, container, text, text_input, svg};
use iced_dialog::dialog;
use iced_split::{Split, Strategy};

pub struct Hub {
    pub split_at_sc: f32,
    pub split_at_cc: f32,
    pub navbar: server_navbar::Navbar,
    pub chat: chat::Chat,
    pub open_dialog: bool,
    pub server_address: String,
    pub server_addresses: Vec<ServerInfo>,
    pub channel_navbar: channel_navbar::ChannelNavbar,
}

#[derive(Debug, Clone)]
pub enum Message {
    ResizeSC(f32),
    ResizeCC(f32),
    Navbar(server_navbar::Message),
    ChannelNavbar(channel_navbar::Message),
    Chat(chat::Message),
    ServerAddressUpdate(String),
    ServerAddressSubmit,
    CloseDialog,
}

const SVG_LOADING: &str = "client/assets/loading.svg";

impl Hub {
    pub fn view(&self) -> Element<Message> {
        let server_channel_split = Split::new(
            server_navbar::view(&self.navbar).map(|msg| Message::Navbar(msg)),
            channel_navbar::view(&self.channel_navbar).map(|msg| Message::ChannelNavbar(msg)),
            self.split_at_sc,
            Message::ResizeSC,
        )
            .strategy(Strategy::Start);

        let channel_chat_split = Split::new(
            server_channel_split,
            chat::view(&self.chat).map(|msg| Message::Chat(msg)),
            self.split_at_cc,
            Message::ResizeCC,
        )
            .strategy(Strategy::Start);

        let container = container(channel_chat_split);

        let dialog_content = column![
            text_input("Enter server address...", &self.server_address)
                .on_input(Message::ServerAddressUpdate)
                .on_submit(Message::ServerAddressSubmit)
        ];

        dialog(self.open_dialog, container, dialog_content)
            .title("Add Server")
            .push_button(iced_dialog::button("Add", Message::ServerAddressSubmit))
            .push_button(iced_dialog::button("Cancel", Message::CloseDialog))
            .width(350)
            .height(234)
            .into()
    }
}
