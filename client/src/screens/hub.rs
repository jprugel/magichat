use crate::server_info::*;
use crate::widgets::chat;
use crate::widgets::server_navbar;
use crate::widgets::channel_navbar;
use iced::{Element, Task};
use iced::widget::{column, container, text, text_input, svg};
use iced_dialog::dialog;
use iced_split::{Split, Strategy};
use tracing::debug;
use crate::action::Action;

pub struct Hub {
    pub split_at_sc: f32,
    pub split_at_cc: f32,
    pub navbar: server_navbar::Navbar,
    pub chat: chat::Chat,
    pub open_dialog: bool,
    pub dialog_written_server_address: String,
    pub servers: Vec<ServerInfo>,
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
    ReceivedInfo(ServerInfo),
}

pub enum Instruction {
    AddServer(String),
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
            self.chat.view().map(|msg| Message::Chat(msg)),
            self.split_at_cc,
            Message::ResizeCC,
        )
            .strategy(Strategy::Start);

        let container = container(channel_chat_split);

        let dialog_content = column![
            text_input("Enter server address...", &self.dialog_written_server_address)
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
    
    //WIP
    pub fn update(&mut self, message: Message) -> Action<Instruction, Message> {
        match message {
            Message::ServerAddressUpdate(address) => {
                debug!("Server address updated: {}", address);
                self.dialog_written_server_address = address;
                Action::none()
            }
            Message::ServerAddressSubmit => {
                debug!("Server address submitted: {}", self.dialog_written_server_address);
                let instruction = Instruction::AddServer(self.dialog_written_server_address.clone());
                self.open_dialog = false;
                Action::instruction(instruction)
            }
            Message::CloseDialog => {
                self.open_dialog = false;
                Action::none()
            }
            _ => Action::none()
        }
    }
}
