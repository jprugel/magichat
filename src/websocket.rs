use iced::futures;
use iced::task::{Never, Sipper, sipper};
use iced::widget::text;

use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use crate::server_info::*;
use async_tungstenite::tungstenite;
use std::fmt;

pub fn connect(url: String) -> impl Sipper<Never, Event> {
    sipper(async move |mut output| {
        loop {
            let current_url = url.clone();
            println!("Connecting to: {}", &current_url);
            let (mut websocket, mut input) =
                match async_tungstenite::tokio::connect_async(&*current_url).await {
                    Ok((websocket, _)) => {
                        println!("Connected!");
                        let (sender, receiver) = mpsc::channel(100);

                        output.send(Event::Connected(Connection(sender))).await;

                        (websocket.fuse(), receiver)
                    }
                    Err(_) => {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                        output.send(Event::Disconnected).await;
                        continue;
                    }
                };

            loop {
                futures::select! {
                    received = websocket.select_next_some() => {
                        match received {
                            Ok(tungstenite::Message::Text(message)) => {
                                match serde_json::from_str::<UserMessage>(&message) {
                                    Ok(user_msg) => {
                                        output.send(Event::MessageReceived(Message::User(user_msg))).await;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to parse incoming message: {e}");
                                    }
                                }
                            },
                            Err(_) => {
                                output.send(Event::Disconnected).await;
                                break;
                            },
                            Ok(_) => {},
                        }
                    }
                    message = input.select_next_some() => {
    let send_result = match &message {
        Message::User(user_msg) => {
            // Serialize UserMessage to JSON string
            match serde_json::to_string(user_msg) {
                Ok(json) => websocket.send(tungstenite::Message::Text(json.into())).await,
                Err(e) => {
                    eprintln!("Failed to serialize UserMessage: {}", e);
                    continue; // skip sending this message
                }
            }
        }
        _ => {
            // For other message variants, do nothing or handle accordingly
            continue;
        }
    };

    if send_result.is_err() {
        output.send(Event::Disconnected).await;
        break;
    }
}
                }
            }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
    MessageReceived(Message),
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected,
    Disconnected,
    User(UserMessage),
}

impl Message {
    pub fn new(message: UserMessage) -> Option<Self> {
        if message.content.is_empty() {
            None
        } else {
            Some(Self::User(message))
        }
    }

    pub fn connected() -> Self {
        Message::Connected
    }

    pub fn disconnected() -> Self {
        Message::Disconnected
    }

    pub fn as_str(&self) -> &str {
        match self {
            Message::Connected => "Connected successfully!",
            Message::Disconnected => "Connection lost... Retrying...",
            Message::User(user_msg) => &user_msg.content,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> text::IntoFragment<'a> for &'a Message {
    fn into_fragment(self) -> text::Fragment<'a> {
        text::Fragment::Borrowed(self.as_str())
    }
}
