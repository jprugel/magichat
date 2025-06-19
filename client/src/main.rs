mod action;
mod screens;
mod websocket;
mod widgets;

use crate::action::Action;
use crate::screens::hub::Hub;
use crate::widgets::channel_navbar::ChannelNavbar;
use iced::Element;
use iced::Length;
use iced::alignment::*;
use iced::task::Task;
use iced::widget::container;
use protocol::{Icon, Server, User, UserMessage};
use screens::*;
use tracing::info;
use uuid::Uuid;
use widgets::chat::*;
use widgets::login::Login;
use widgets::*;

const ICON: &str = "client/assets/magichat_icon.png";

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(App::new, App::update, App::view)
        .window(App::window())
        .settings(App::settings())
        .run()
}

struct App {
    screen: Screen,
    login: Login,
    state: State,
    user: User,
    hub: Hub,
}

enum Screen {
    Login,
    Hub,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Login(login::Message),
    Websocket(websocket::Event),
    Hub(hub::Message),
    ReceivedServerInfo(Server),
    ImageFetchFailed,
    ReceivedServerImage(Vec<u8>),
    ServerInfoFailed,
}

enum State {
    Connected(websocket::Connection),
    Disconnected,
}

impl App {
    fn window() -> iced::window::Settings {
        let icon = iced::window::icon::from_file(ICON).expect("Failed to get icon.");

        iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            ..Default::default()
        }
    }

    fn settings() -> iced::Settings {
        let font = iced::Font::with_name("JetBrainsMonoNLNerdFont-Regular");

        iced::Settings {
            default_font: font,
            ..Default::default()
        }
    }

    fn new() -> (Self, Task<Message>) {
        let load_font = |data: &'static [u8]| iced::font::load(data).map(Message::FontLoaded);

        let task = Task::batch(vec![load_font(include_bytes!(
            "../assets/fonts/JetBrainsMonoNLNerdFont-Regular.ttf"
        ))]);

        let app = Self {
            screen: Screen::Login,
            login: Login::default(),
            state: State::Disconnected,
            user: User::default(),
            hub: hub::Hub {
                split_at_sc: 80.,
                split_at_cc: 300.,
                navbar: server_navbar::Navbar::default(),
                chat: Chat::default(),
                open_dialog: false,
                dialog_written_server_address: String::default(),
                //servers: Vec::default(),
                channel_navbar: ChannelNavbar::default(),
            },
        };

        (app, task)
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Login => container(self.login.view().map(Message::Login))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            Screen::Hub => {
                self.hub.view().map(Message::Hub)
                //widgets::chat::view(&self.chat).map(|msg| Message::Chat(msg))
            }
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {
                info!("Fonts loaded.");
                Task::none()
            }
            Message::Login(msg) => {
                let Action {
                    instruction,
                    task: _,
                } = self.login.update(msg);
                if let Some(login::Instruction::Login { username }) = instruction {
                    self.user.username = username;
                    self.screen = Screen::Hub;
                }
                Task::none()
            }
            Message::Websocket(websocket::Event::Connected(connection)) => {
                info!("Websocket connected");
                self.state = State::Connected(connection);
                Task::none()
            }
            Message::Websocket(websocket::Event::MessageReceived(msg)) => {
                info!("Message received: {}", msg);
                if let websocket::Message::User(user_message) = msg.clone() {
                    self.hub.chat.channel.log.push(user_message.clone());
                    let channel = self
                        .hub
                        .channel_navbar
                        .channels
                        .iter_mut()
                        .find(|c| c.id == user_message.channel_id)
                        .unwrap();
                    channel.log.push(user_message.clone());
                };
                Task::none()
            }
            Message::Websocket(websocket::Event::Disconnected) => {
                info!("Websocket disconnected");
                Task::none()
            }
            Message::Hub(hub::Message::ResizeSC(_split_at)) => Task::none(),
            Message::Hub(hub::Message::ResizeCC(_split_at)) => Task::none(),
            Message::Hub(hub::Message::Chat(chat::Message::Updated(msg))) => {
                info!("chat::text_input updated: {}", msg);
                self.hub.chat.written_text = msg;
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::Submitted)) => {
                info!(
                    "chat::text_input submitted: {}",
                    self.hub.chat.written_text.clone()
                );
                match &mut self.state {
                    State::Connected(connection) => {
                        let msg = UserMessage {
                            user: self.user.clone(),
                            channel_id: self.hub.chat.channel.id,
                            content: self.hub.chat.written_text.clone(),
                            id: Uuid::new_v4(),
                        };
                        connection.send(websocket::Message::User(msg));
                    }
                    State::Disconnected => {
                        println!("Server is not connected");
                    }
                };
                self.hub.chat.written_text.clear();
                Task::none()
            }
            Message::Hub(msg) => {
                let Action {
                    instruction,
                    task: _task,
                } = self.hub.update(msg);
                if let Some(instruction) = instruction {
                    match instruction {
                        hub::Instruction::AddServer(_server_address) => {
                            info!(
                                "Server address submitted: {}",
                                self.hub.dialog_written_server_address.clone()
                            );

                            Task::batch(vec![
                                Task::perform(
                                    {
                                        let server_address =
                                            self.hub.dialog_written_server_address.clone();
                                        async move { Server::from_url(&server_address).await }
                                    },
                                    |output| match output {
                                        Ok(server) => Message::ReceivedServerInfo(server),
                                        Err(_) => Message::ServerInfoFailed,
                                    },
                                ),
                                Task::perform(
                                    {
                                        info!("Fetching server icon");
                                        let server_address = format!(
                                            "http://{}/images/server_icon.png",
                                            self.hub.dialog_written_server_address.clone()
                                        );
                                        async move {
                                            match reqwest::get(&server_address).await {
                                                Ok(response) if response.status().is_success() => {
                                                    let bytes =
                                                        response.bytes().await.unwrap_or_default();
                                                    Some(bytes)
                                                }
                                                _ => None,
                                            }
                                        }
                                    },
                                    |maybe_bytes| match maybe_bytes {
                                        Some(bytes) => Message::ReceivedServerImage(bytes.into()),
                                        None => Message::ImageFetchFailed,
                                    },
                                ),
                            ])
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::ReceivedServerInfo(info) => {
                info!("Received server info: {:?}", info);

                info.clone()
                    .channel_list
                    .into_iter()
                    .for_each(|channel| self.hub.channel_navbar.channels.push(channel));
                self.hub.chat.channel = self.hub.channel_navbar.channels[0].clone();
                self.hub.navbar.servers.push(info);
                let websocket_address =
                    format!("ws://{}/ws", self.hub.dialog_written_server_address.clone());
                Task::batch(vec![
                    Task::sip(
                        websocket::connect(websocket_address),
                        |event| Message::Websocket(event),
                        |_| Message::Websocket(websocket::Event::Disconnected),
                    ),
                    self.update(Message::Hub(hub::Message::CloseDialog)),
                ])
            }

            Message::ServerInfoFailed => {
                info!("Failed to get server info");
                Task::none()
            }

            Message::ReceivedServerImage(bytes) => {
                info!("Received server icon");
                self.hub.chat.server.icon = Icon::Image(bytes);
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
