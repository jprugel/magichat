mod chat;
mod login;
mod websocket;

use chat::Chat;
use futures::StreamExt;
use iced::Element;
use iced::Length;
use iced::Subscription;
use iced::alignment::*;
use iced::task::Task;
use iced::widget::*;
use login::Login;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .window(App::window())
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
    login: Login,
    chat: Chat,
    state: State,
}

enum State {
    Connected(websocket::Connection),
    Disconnected,
}

impl App {
    fn window() -> iced::window::Settings {
        let icon =
            iced::window::icon::from_file("./magichat_icon.png").expect("Failed to get icon.");

        let settings = iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            ..Default::default()
        };

        settings
    }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Login,
                login: Login::default(),
                chat: Chat::default(),
                state: State::Disconnected,
            },
            Task::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Login => container(login::view(&self.login).map(|msg| Message::Login(msg)))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            Screen::Chat => chat::view(&self.chat).map(|msg| Message::Chat(msg)).into(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
                Task::none()
            }
            Message::Chat(chat::Message::UserUpdated(msg)) => {
                self.chat.written_text = msg;
                Task::none()
            }
            Message::Chat(chat::Message::UserSubmitted) => {
                let text = format!(
                    "{}: {}",
                    self.login.username,
                    self.chat.written_text.clone()
                );
                dbg!("message::user_submitted");
                match &mut self.state {
                    State::Connected(connection) => {
                        connection.send(websocket::Message::User(text));
                    }
                    State::Disconnected => {
                        println!("Server is not connected");
                    }
                };
                self.chat.written_text.clear();
                Task::none()
            }
            Message::Chat(_) => Task::none(),
            Message::Websocket(websocket::Event::Connected(connection)) => {
                dbg!("websocket::connected");
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
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let url = self.login.url.clone();
        if !self.login.submitted { return Subscription::none() }
        match url::Url::parse(&url) {
            Ok(_) => Subscription::run_with(url, |url| websocket::connect(url.to_string()).map(Message::Websocket)),
            Err(_) => Subscription::none(),
        }
    }
}

enum Screen {
    Login,
    Chat,
}

#[derive(Debug, Clone)]
enum Message {
    Login(login::Message),
    Chat(chat::Message),
    Websocket(websocket::Event),
}
