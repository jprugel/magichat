mod screens;
mod server_info;
mod websocket;
mod widgets;
mod action;

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
use uuid::Uuid;
use crate::action::Action;
use crate::screens::hub::Hub;

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
            hub: hub::Hub {
                split_at_sc: 80.,
                split_at_cc: 300.,
                navbar: server_navbar::Navbar::default(),
                chat: Chat::default(),
                open_dialog: false,
                server_address: String::default(),
                server_addresses: Vec::default(),
                channel_navbar: ChannelNavbar::default(),
            },
        };

        (app, task)
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Login => container(self.login.view().map(|msg| Message::Login(msg)))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            Screen::Hub => {
                self.hub.view().map(|msg| Message::Hub(msg))
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
                let Action { instruction, task } = self.login.update(msg);
                if let Some(instruction) = instruction {
                    match instruction {
                        login::Instruction::Login { username } => {
                            self.user.username = username;
                            self.screen = Screen::Hub;
                        },
                        _ => {}
                    }
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
                    let channel = self.hub.channel_navbar.channels.iter_mut().find(|c| c.name == user_message.channel).unwrap();
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
            Message::Hub(hub::Message::Navbar(server_navbar::Message::Reorder(drag_event))) => {
                match drag_event {
                    DragEvent::Picked { .. } => {
                        // Handle Pick Event!
                    }
                    DragEvent::Dropped {
                        index,
                        target_index,
                    } => {
                        let item = self.hub.navbar.servers.remove(index);
                        self.hub.navbar.servers.insert(target_index, item);
                    }
                    DragEvent::Canceled { .. } => {
                        // Handle canceled event
                    }
                }
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::UserUpdated(msg))) => {
                info!("chat::text_input updated: {}", msg);
                self.hub.chat.written_text = msg;
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::UserSubmitted)) => {
                info!("chat::text_input submitted: {}", self.hub.chat.written_text.clone());
                match &mut self.state {
                    State::Connected(connection) => {
                        let msg = UserMessage {
                            user: self.user.clone(),
                            channel: "General".to_string(),
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
            Message::Hub(hub::Message::Navbar(server_navbar::Message::SelectServer(server))) => {
                info!("Selected server: {}", server.name);
                self.hub.chat.server = server;
                Task::none()
            }
            Message::Hub(hub::Message::Navbar(server_navbar::Message::AddServer)) => {
                info!("Adding server");
                self.hub.server_address.clear();
                self.hub.open_dialog = true;
                Task::none()
            }
            Message::Hub(hub::Message::ServerAddressUpdate(server)) => {
                info!("Server address updated: {}", server);
                self.hub.server_address = server;
                Task::none()
            }
            Message::Hub(hub::Message::ServerAddressSubmit) => {
                info!("Server address submitted: {}", self.hub.server_address.clone());
                let websocket_address =
                    format!("ws://{}/ws", self.hub.server_address.clone());

                Task::batch(vec![
                    Task::perform(
                        {
                            let server_address = format!("http://{}/info", self.hub.server_address.clone());
                            async move { ServerInfo::from_url(&server_address).await }
                        },
                        |output| Message::ReceivedServerInfo(output.unwrap())
                    ),
                    Task::sip(
                        websocket::connect(websocket_address),
                        |event| Message::Websocket(event),
                        |_| Message::Websocket(websocket::Event::Disconnected),
                    ),
                ])
            }
            Message::Hub(hub::Message::CloseDialog) => {
                info!("Closing dialog");
                self.hub.open_dialog = false;
                Task::none()
            }
            Message::ReceivedServerInfo(info) => {
                info!("Received server info: {:?}", info);
                
                info.clone().channel_list.into_iter().for_each(|channel| self.hub.channel_navbar.channels.push(channel));
                self.hub.chat.channel = self.hub.channel_navbar.channels[0].clone();
                self.hub.navbar.servers.push(info);
                self.update(Message::Hub(hub::Message::CloseDialog))
            },
            Message::Hub(hub::Message::ChannelNavbar(channel_navbar::Message::ChannelSelected(channel))) => {
                info!("Selected channel: {}", channel);
                let test = self.hub.channel_navbar.channels.iter().find(|c| c.name == channel).unwrap().clone();
                info!("test: {:?}", test);
                self.hub.chat.channel = self.hub.channel_navbar.channels.iter().find(|c| c.name == channel).unwrap().clone();
                Task::none()
            },
            _ => {
                Task::none()
            }
        }
    }
}
