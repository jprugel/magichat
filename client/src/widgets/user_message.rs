use iced::widget::text::Shaping;
use iced::widget::{column, container, horizontal_space, row, svg, text};
use iced::{Alignment, Element, Length};
use protocol::UserMessage;

#[derive(Clone, Debug)]
pub(crate) enum Message {}
pub fn view(state: &UserMessage) -> Element<Message> {
    let icon = svg("client/assets/user_icon.svg").height(45).width(45);

    container(
        row![
            container(icon).align_y(Alignment::Center),
            horizontal_space().width(7),
            container(column![
                container(
                    text(&state.user.username)
                        .shaping(Shaping::Advanced)
                        .style(text::success),
                ),
                container(text(&state.content).shaping(Shaping::Advanced),).width(Length::Fill),
            ])
            .width(Length::Fill)
            .style(container::secondary),
        ]
        .width(Length::Fill)
        .padding(3),
    )
    .style(container::bordered_box)
    .into()
}
