mod screens;
mod server_info;
mod websocket;
mod widgets;

use std::collections::HashMap;
use dragking::DragEvent;
use iced::Element;
use iced::Length;
use iced::alignment::*;
use iced::task::Task;
use iced::widget::container;
use tracing::log::info;
use screens::*;
use server_info::*;
use widgets::chat::*;
use widgets::login::Login;
use widgets::*;
use widgets::user_message::UserMessage;
use crate::widgets::channel_navbar::ChannelNavbar;
use tracing_subscriber::*;

const ICON: &'static str = "client/assets/magichat_icon.png";

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
    hub_state: screens::hub::State,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Login(login::Message),
    Websocket(websocket::Event),
    Hub(hub::Message),
    ReceivedServerInfo(ServerInfo),
}

enum State {
    Connected(websocket::Connection),
    Disconnected,
}

impl App {
    fn window() -> iced::window::Settings {
        let icon = iced::window::icon::from_file(ICON)
            .expect("Failed to get icon.");

        let settings = iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            ..Default::default()
        };

        settings
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
            hub_state: hub::State {
                split_at_sc: 80.,
                split_at_cc: 300.,
                navbar: navbar::Navbar::default(),
                chat: Chat::default(),
                open_dialog: false,
                server_address: String::default(),
                server_addresses: Vec::default(),
                channel_navbar: ChannelNavbar::default()
            },
        };

        (app, task)
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Login => container(login::view(&self.login).map(|msg| Message::Login(msg)))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            Screen::Chat => {
                screens::hub::view(&self.hub_state).map(|msg| Message::Hub(msg))
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
            Message::Login(login::Message::UpdatedUsername(username)) => {
                info!("Username updated: {}", username);
                self.login.username = username;
                Task::none()
            }
            Message::Login(login::Message::Submitted) => {
                // Do login logic here.
                info!("Login Submitted: {}", self.login.username.clone());
                self.user.username = self.login.username.clone();
                self.screen = Screen::Chat;
                Task::none()
            }
            Message::Websocket(websocket::Event::Connected(connection)) => {
                info!("Websocket connected");
                self.state = State::Connected(connection);
                Task::none()
            }
            Message::Websocket(websocket::Event::MessageReceived(msg)) => {
                info!("Message received: {}", msg);
                self.hub_state.chat.channel.log.push(msg.clone().into());
                self.hub_state.channel_navbar.channels.iter_mut().for_each(|channel| {
                    if let websocket::Message::User(user_message ) = msg.clone() {
                        info!("user message: {:?}", user_message);
                        info!("channel: {:?}", channel.name);
                        info!("user: {:?}", &user_message.channel);
                        if channel.name == user_message.channel {
                            channel.log.push(msg.clone().into());
                        }
                    }
                });
                Task::none()
            }
            Message::Websocket(websocket::Event::Disconnected) => {
                info!("Websocket disconnected");
                Task::none()
            }
            Message::Hub(hub::Message::ResizeSC(_split_at)) => Task::none(),
            Message::Hub(hub::Message::ResizeCC(_split_at)) => Task::none(),
            Message::Hub(hub::Message::Navbar(navbar::Message::Reorder(drag_event))) => {
                match drag_event {
                    DragEvent::Picked { .. } => {
                        // Handle Pick Event!
                    }
                    DragEvent::Dropped {
                        index,
                        target_index,
                    } => {
                        let item = self.hub_state.navbar.servers.remove(index);
                        self.hub_state.navbar.servers.insert(target_index, item);
                    }
                    DragEvent::Canceled { .. } => {
                        // Handle canceled event
                    }
                }
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::UserUpdated(msg))) => {
                info!("chat::text_input updated: {}", msg);
                dbg!(self.hub_state.chat.clone());
                self.hub_state.chat.written_text = msg;
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::UserSubmitted)) => {
                info!("chat::text_input submitted: {}", self.hub_state.chat.written_text.clone());
                match &mut self.state {
                    State::Connected(connection) => {
                        let msg = UserMessage {
                            user: self.user.clone(),
                            channel: "General".to_string(),
                            content: self.hub_state.chat.written_text.clone(),
                        };
                        connection.send(websocket::Message::User(msg));
                    }
                    State::Disconnected => {
                        println!("Server is not connected");
                    }
                };
                self.hub_state.chat.written_text.clear();
                Task::none()
            }
            Message::Hub(hub::Message::Navbar(navbar::Message::SelectServer(server))) => {
                info!("Selected server: {}", server.name);
                self.hub_state.chat.server = server;
                Task::none()
            }
            Message::Hub(hub::Message::Navbar(navbar::Message::AddServer)) => {
                info!("Adding server");
                self.hub_state.server_address.clear();
                self.hub_state.open_dialog = true;
                Task::none()
            }
            Message::Hub(hub::Message::ServerAddressUpdate(server)) => {
                info!("Server address updated: {}", server);
                self.hub_state.server_address = server;
                Task::none()
            }
            Message::Hub(hub::Message::ServerAddressSubmit) => {
                info!("Server address submitted: {}", self.hub_state.server_address.clone());
                let websocket_address =
                    format!("ws://{}/ws", self.hub_state.server_address.clone());
                
                Task::batch(vec![
                    Task::perform(
                        {
                            let server_address = format!("http://{}/info", self.hub_state.server_address.clone());
                            async move { ServerInfo::from_url(&server_address).await }
                        },
                        |output| Message::ReceivedServerInfo(output.unwrap())
                    ),
                    self.update(Message::Hub(hub::Message::CloseDialog)),
                    Task::sip(
                        websocket::connect(websocket_address),
                        |event| Message::Websocket(event),
                        |_| Message::Websocket(websocket::Event::Disconnected),
                    ),
                ])
            }
            Message::Hub(hub::Message::CloseDialog) => {
                info!("Closing dialog");
                self.hub_state.open_dialog = false;
                Task::none()
            }
            Message::ReceivedServerInfo(info) => {
                info!("Received server info: {:?}", info);
                info.clone().channel_list.into_iter().for_each(|channel| self.hub_state.channel_navbar.channels.push(channel));
                self.hub_state.chat.channel = self.hub_state.channel_navbar.channels[0].clone();
                self.hub_state.navbar.servers.push(info);
                Task::none()
            },
            Message::Hub(hub::Message::ChannelNavbar(channel_navbar::Message::ChannelSelected(channel))) => {
                info!("Selected channel: {}", channel);
                let test = self.hub_state.channel_navbar.channels.iter().find(|c| c.name == channel).unwrap().clone();
                info!("test: {:?}", test);
                self.hub_state.chat.channel = self.hub_state.channel_navbar.channels.iter().find(|c| c.name == channel).unwrap().clone();
                Task::none()
            },
            _ => {
                Task::none()
            }
        }
    }
}

enum Screen {
    Login,
    Chat,
}
