// Navbar has 3 main components:
// 1.) the direct messages tab.
// 2.) the list of servers the user is connected to.
// 3.) an add server button.
use crate::ServerInfo;
use dragking::DragEvent;
use iced::Alignment;
use iced::Border;
use iced::Length;
use iced::border::radius;
use iced::widget::{Container, button, container, text, column, svg};
use iced::{Element, Renderer, Theme};

#[derive(Default)]
pub struct Navbar {
    pub servers: Vec<ServerInfo>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Reorder(DragEvent),
    AddServer,
    SelectServer(ServerInfo),
    SelectSettings,
}

const WIDTH: f32 = 60.;
const HEIGHT: f32 = 60.;
const ADD_SERVER_SVG: &str = "client/assets/add_server.svg";
const SETTINGS_SVG: &str = "client/assets/settings.svg";
const DEFAULT_SERVER_SVG: &str = "client/assets/server.svg";

pub fn view(state: &Navbar) -> Element<'_, Message> {
    let style = |theme: &Theme, _| button::Style {
        border: Border {
            color: theme.palette().primary,
            width: 1.0,
            radius: radius(10.0),
        },
        background: Some(theme.palette().primary.into()),
        ..Default::default()
    };

    let settings: iced::widget::Button<'_, Message, Theme, Renderer> = button(svg(SETTINGS_SVG).width(Length::Fill).height(Length::Fill))
        .width(Length::Fixed(WIDTH))
        .height(Length::Fixed(HEIGHT))
        .on_press(Message::SelectSettings)
        .style(style);

    let items: Vec<Element<'_, Message>> = state
        .servers
        .iter()
        .map(|server| {
            let letter = server.name.chars().nth(0).unwrap_or('E').to_string();

            let button = button(svg(DEFAULT_SERVER_SVG).width(Length::Fill).height(Length::Fill))
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

    let drag: dragking::column::Column<Message> = dragking::column(items).spacing(5).into();

    container(column![settings, drag, add_server].spacing(5)).width(Length::Fill).align_x(Alignment::Center).into()
}
