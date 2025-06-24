// Navbar has 3 main components:
// 1.) the direct messages tab.
// 2.) the list of servers the user is connected to.
// 3.) an add server button.
use crate::action::Action;
use dragking::DragEvent;
use iced::Alignment;
use iced::Border;
use iced::Length;
use iced::advanced::image::Handle;
use iced::border::radius;
use iced::widget::{button, column, container, image as iced_image, svg};
use iced::{Element, Renderer, Theme};
use protocol::{Icon, Server};
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
                let button = match &server.icon {
                    Icon::Default => button(
                        svg(DEFAULT_SERVER_SVG)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .style(style),
                    Icon::Svg(_path) => button(
                        svg(DEFAULT_SERVER_SVG)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .style(style),
                    Icon::Image(bytes) => {
                        let mut image = image::ImageReader::new(std::io::Cursor::new(bytes))
                            .with_guessed_format()
                            .unwrap()
                            .decode()
                            .unwrap()
                            .to_rgba8();

                        if !has_rounded_corners(&image) {
                            info!("without rounded corners");
                            round_corners(&mut image, 10.);
                        }

                        let (width, height) = image.dimensions();

                        let transparent = |_: &Theme, _| button::Style {
                            border: Border {
                                color: iced::Color::TRANSPARENT,
                                ..Default::default()
                            },
                            ..Default::default()
                        };

                        button(iced_image(Handle::from_rgba(
                            width,
                            height,
                            image.as_raw().to_vec(),
                        )))
                        .style(transparent)
                    }
                }
                .padding(0.)
                .clip(true)
                .width(Length::Fixed(WIDTH))
                .height(Length::Fixed(HEIGHT))
                .on_press(Message::SelectServer(server.clone()));

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

fn has_rounded_corners(rgba: &image::RgbaImage) -> bool {
    rgba.get_pixel(0, 0).0[3] == 0
}

// Courtesy of ChatGPT
fn round_corners(rgba: &mut image::RgbaImage, radius: f32) {
    let (width, height) = rgba.dimensions();

    let radius = (width as f32 * (radius / 50.)) as u32;
    let radius_sq = radius * radius;
    let aa_span = radius / 4;

    for y in 0..height {
        for x in 0..width {
            let dist_x = if x < radius {
                radius - x
            } else if x >= width - radius {
                x - (width - radius - 1)
            } else {
                0
            };

            let dist_y = if y < radius {
                radius - y
            } else if y >= height - radius {
                y - (height - radius - 1)
            } else {
                0
            };

            let dist_sq = dist_x * dist_x + dist_y * dist_y;

            if dist_sq > radius_sq {
                let dist = (dist_sq as f32).sqrt();

                if dist <= (radius + aa_span) as f32 {
                    let alpha_scale =
                        1.0 - (dist_sq - radius_sq) as f32 / (aa_span * aa_span) as f32;

                    let pixel = rgba.get_pixel_mut(x, y);
                    pixel.0[3] = (pixel.0[3] as f32 * alpha_scale) as u8;
                } else {
                    let pixel = rgba.get_pixel_mut(x, y);
                    pixel.0 = [0; 4];
                }
            }
        }
    }
}
