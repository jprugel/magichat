mod direct;
mod server;

use crate::action::Action;
use crate::widgets::channel_navbar;
use crate::widgets::chat;
use crate::widgets::server_navbar;
use iced::Element;
use iced::widget::{column, container, text_input};
use iced_dialog::dialog;
use iced_split::{Split, Strategy};
use tracing::{debug, info};

pub struct Hub {
    pub split_at_sc: f32,
    pub split_at_cc: f32,
    pub navbar: server_navbar::Navbar,
    pub chat: chat::Chat,
    pub open_dialog: bool,
    pub dialog_written_server_address: String,
    //pub servers: Vec<Server>,
    pub channel_navbar: channel_navbar::ChannelNavbar,
}

/*
pub enum Screen {
    Direct,
    Server,
}
*/

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
    //ReceivedInfo(Server),
}

pub enum Instruction {
    AddServer(String),
}

//const SVG_LOADING: &str = "client/assets/loading.svg";

impl Hub {
    pub fn view(&self) -> Element<'_, Message> {
        let server_channel_split = Split::new(
            self.navbar.view().map(Message::Navbar),
            channel_navbar::view(&self.channel_navbar).map(Message::ChannelNavbar),
            self.split_at_sc,
            Message::ResizeSC,
        )
        .strategy(Strategy::Start);

        let channel_chat_split = Split::new(
            server_channel_split,
            self.chat.view().map(Message::Chat),
            self.split_at_cc,
            Message::ResizeCC,
        )
        .strategy(Strategy::Start);

        let container = container(channel_chat_split);

        let dialog_content = column![
            text_input(
                "Enter server address...",
                &self.dialog_written_server_address
            )
            .on_input(Message::ServerAddressUpdate)
            .on_submit(Message::ServerAddressSubmit),
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
                debug!(
                    "Server address submitted: {}",
                    self.dialog_written_server_address
                );
                let instruction =
                    Instruction::AddServer(self.dialog_written_server_address.clone());
                Action::instruction(instruction)
            }
            Message::CloseDialog => {
                self.open_dialog = false;
                Action::none()
            }
            Message::Navbar(message) => {
                let Action {
                    instruction,
                    task: _task,
                } = self.navbar.update(message);
                if let Some(instruction) = instruction {
                    match instruction {
                        server_navbar::Instruction::SelectServer(server) => {
                            self.chat.server = server;
                            Action::none()
                        }
                        server_navbar::Instruction::AddServer => {
                            self.dialog_written_server_address.clear();
                            self.open_dialog = true;
                            Action::none()
                        } //_ => Action::none(),
                    }
                } else {
                    Action::none()
                }
            }
            Message::ChannelNavbar(channel_navbar::Message::ChannelSelected(channel)) => {
                info!("Selected channel: {}", channel);
                let test = self
                    .channel_navbar
                    .channels
                    .iter()
                    .find(|c| c.name == channel)
                    .unwrap()
                    .clone();
                info!("test: {:?}", test);
                self.chat.channel = self
                    .channel_navbar
                    .channels
                    .iter()
                    .find(|c| c.name == channel)
                    .unwrap()
                    .clone();
                Action::none()
            }
            _ => Action::none(),
        }
    }
}
