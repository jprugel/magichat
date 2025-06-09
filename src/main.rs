mod widgets;
mod websocket;
mod server_info;
mod screens;

use server_info::*;
use widgets::*;
use widgets::chat::*;
use screens::*;
use iced::Element;
use iced::Length;
use iced::alignment::*;
use iced::task::Task;
use iced::widget::{container};
use widgets::login::Login;
use dragking::DragEvent;
use server_info::*;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .window(App::window())
        .settings(App::settings())
        .run()
}

struct App {
    screen: Screen,
    login: Login,
    chat: Chat,
    state: State,
    user: User,
    hub_state: screens::hub::State,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Login(login::Message),
    //Chat(chat::Message),
    Websocket(websocket::Event),
    Hub(hub::Message)
}

enum State {
    Connected(websocket::Connection),
    Disconnected,
}

impl App {
    fn window() -> iced::window::Settings {
        let icon =
            iced::window::icon::from_file("./assets/magichat_icon.png").expect("Failed to get icon.");

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

        let task = Task::batch(vec![
            load_font(include_bytes!("../assets/fonts/JetBrainsMonoNLNerdFont-Regular.ttf")),
        ]);


        let app = Self {
            screen: Screen::Login,
            login: Login::default(),
            chat: Chat::default(),
            state: State::Disconnected,
            user: User::default(),
            hub_state: hub::State {
                split_at_sc: 80.,
                split_at_cc: 300.,
                navbar: navbar::Navbar::default(),
                chat: Chat::default(),
            }
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
            },
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {
                dbg!("Fonts loaded.");
                Task::none()
            },
            Message::Login(login::Message::UpdatedUsername(username)) => {
                self.login.username = username;
                Task::none()
            }
            Message::Login(login::Message::UpdatedUrl(url)) => {
                self.login.url = url;
                Task::none()
            }
            Message::Login(login::Message::Submitted) => {
                // Do login logic here.
                self.screen = Screen::Chat;
                Task::sip(
                    websocket::connect(self.login.url.clone()), 
                    |event| Message::Websocket(event),
                    |_| Message::Websocket(websocket::Event::Disconnected)
                )
            }
            Message::Websocket(websocket::Event::Connected(connection)) => {
                dbg!("websocket::connected");
                self.hub_state.navbar.servers.push(ServerInfo::default());
                self.state = State::Connected(connection);
                Task::none()
            }
            Message::Websocket(websocket::Event::MessageReceived(msg)) => {
                dbg!("websocket::message_received");
                self.chat.text_log.push(msg.to_string());
                Task::none()
            }
            Message::Websocket(websocket::Event::Disconnected) => {
                dbg!("websocket::disconnected");
                Task::none()
            },
            Message::Hub(hub::Message::ResizeSC(_split_at)) => {
                Task::none()
            },
            Message::Hub(hub::Message::ResizeCC(_split_at)) => {
                Task::none()
            },
            Message::Hub(hub::Message::Navbar(navbar::Message::Reorder(drag_event))) => {
                match drag_event {
                    DragEvent::Picked { .. } => {
                        // Handle Pick Event!
                    },
                    DragEvent::Dropped {
                        index, target_index
                    } => {
                        let item = self.hub_state.navbar.servers.remove(index);
                        self.hub_state.navbar.servers.insert(target_index, item);
                    },
                    DragEvent::Canceled { .. } => {
                        // Handle canceled event
                    }
                }
                Task::none()
            },
            Message::Hub(hub::Message::Chat(chat::Message::UserUpdated(msg))) => {
                self.chat.written_text = msg;
                Task::none()
            }
            Message::Hub(hub::Message::Chat(chat::Message::UserSubmitted)) => {
                let text = format!(
                    "{}: {}",
                    self.login.username,
                    self.chat.written_text.clone()
                );
                dbg!("message::user_submitted");
                match &mut self.state {
                    State::Connected(connection) => {
                        connection.send(websocket::Message::User(text));
                        self.hub_state.navbar.servers.push(ServerInfo::default())
                    }
                    State::Disconnected => {
                        println!("Server is not connected");
                    }
                };
                self.chat.written_text.clear();
                Task::none()
            }
            Message::Hub(_) => Task::none()
        }
    }
}

enum Screen {
    Login,
    Chat,
}


