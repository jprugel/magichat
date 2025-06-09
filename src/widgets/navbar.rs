// Navbar has 3 main components:
// 1.) the direct messages tab.
// 2.) the list of servers the user is connected to.
// 3.) an add server button.
use crate::ServerInfo;
use dragking::DragEvent;
use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Length;
use iced::border::Radius;
use iced::border::radius;
use iced::theme::*;
use iced::widget::{Container, button, container, text};
use iced::{Element, Renderer, Theme};

#[derive(Default)]
pub struct Navbar {
    pub servers: Vec<ServerInfo>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Reorder(DragEvent),
    AddServer,
    SelectServer(String),
}

pub fn view(state: &Navbar) -> Element<'_, Message> {
    let mut items: Vec<Element<'_, Message>> = state
        .servers
        .iter()
        .map(|server| {
            let letter = server.name.chars().nth(0).unwrap_or('E').to_string();

            let button = button(text(letter))
                .width(Length::Fixed(60.))
                .height(Length::Fixed(60.))
                .style(|theme: &Theme, _| button::Style {
                    border: Border {
                        color: theme.palette().primary,
                        width: 1.0,
                        radius: radius(10.0),
                    },
                    background: Some(theme.palette().primary.into()),
                    ..Default::default()
                });

            let container: Container<'_, Message, Theme, Renderer> = container(button)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding([5, 10]);

            container.into()
        })
        .collect();

    let add_server = button(text("add server"))
        .width(Length::Fixed(60.))
        .height(Length::Fixed(60.))
        .style(|theme: &Theme, _| button::Style {
            border: Border {
                color: theme.palette().primary,
                width: 1.0,
                radius: radius(10.0),
            },
            background: Some(theme.palette().primary.into()),
            ..Default::default()
        });

    let cont: Container<'_, Message, Theme, Renderer> = container(add_server)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding([5, 10]);

    items.push(cont.into());

    let drag: dragking::column::Column<Message> = dragking::column(items).spacing(5).into();

    container(drag).into()
}
