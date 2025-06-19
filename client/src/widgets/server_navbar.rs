// Navbar has 3 main components:
// 1.) the direct messages tab.
// 2.) the list of servers the user is connected to.
// 3.) an add server button.
use crate::action::Action;
use dragking::DragEvent;
use iced::Alignment;
use iced::Border;
use iced::Length;
use iced::border::radius;
use iced::widget::{button, column, container, svg};
use iced::{Element, Renderer, Theme};
use protocol::Server;
use tracing::info;

#[derive(Default)]
pub struct Navbar {
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Message {
    Reorder(DragEvent),
    AddServer,
    SelectServer(Server),
    SelectSettings,
}

pub enum Instruction {
    //SelectSettings,
    SelectServer(Server),
    AddServer,
}

const WIDTH: f32 = 60.;
const HEIGHT: f32 = 60.;
const ADD_SERVER_SVG: &str = "client/assets/add_server.svg";
const SETTINGS_SVG: &str = "client/assets/settings.svg";
const DEFAULT_SERVER_SVG: &str = "client/assets/server.svg";

impl Navbar {
    pub fn view(&self) -> Element<'_, Message> {
        let style = |theme: &Theme, _| button::Style {
            border: Border {
                color: theme.palette().primary,
                width: 1.0,
                radius: radius(10.0),
            },
            background: Some(theme.palette().primary.into()),
            ..Default::default()
        };

        let settings: iced::widget::Button<'_, Message, Theme, Renderer> =
            button(svg(SETTINGS_SVG).width(Length::Fill).height(Length::Fill))
                .width(Length::Fixed(WIDTH))
                .height(Length::Fixed(HEIGHT))
                .on_press(Message::SelectSettings)
                .style(style);

        let items: Vec<Element<'_, Message>> = self
            .servers
            .iter()
            .map(|server| {
                let button = button(
                    svg(DEFAULT_SERVER_SVG)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fixed(WIDTH))
                .height(Length::Fixed(HEIGHT))
                .on_press(Message::SelectServer(server.clone()))
                .style(style);

                button.into()
            })
            .collect();

        let add_server = button(svg(ADD_SERVER_SVG).width(Length::Fill).height(Length::Fill))
            .width(Length::Fixed(WIDTH))
            .height(Length::Fixed(HEIGHT))
            .style(style)
            .on_press(Message::AddServer);

        let drag: dragking::column::Column<Message> = dragking::column(items).spacing(5);

        container(column![settings, drag, add_server].spacing(5))
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }
    pub fn update(&mut self, message: Message) -> Action<Instruction, Message> {
        match message {
            Message::SelectServer(server) => {
                info!("Selected server: {}", server.name);
                Action::instruction(Instruction::SelectServer(server))
            }
            Message::AddServer => {
                info!("Adding server");
                Action::instruction(Instruction::AddServer)
            }
            Message::Reorder(drag_event) => {
                match drag_event {
                    DragEvent::Picked { .. } => {
                        // Handle Pick Event!
                    }
                    DragEvent::Dropped {
                        index,
                        target_index,
                    } => {
                        let item = self.servers.remove(index);
                        self.servers.insert(target_index, item);
                    }
                    DragEvent::Canceled { .. } => {
                        // Handle canceled event
                    }
                }
                Action::none()
            }
            Message::SelectSettings => todo!(),
        }
    }
}
