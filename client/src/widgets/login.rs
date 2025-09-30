use crate::action::Action;
use fast_qr::QRBuilder;
use fast_qr::convert::svg::SvgBuilder;
use fast_qr::convert::{Builder, Shape};
use iced::Element;
use iced::Padding;
use iced::Theme;
use iced::border::*;
use iced::widget::{column, container, svg, text, text_input};
use otp_std::Totp;
use otp_std::{Algorithm, Digits, Period, Secret, Skew};
use rand::RngCore;
use tracing::info;

#[derive(Debug, Clone)]
pub enum Message {
    UpdatedUsername(String),
    UpdatedPassword(String),
    UpdatedCode(String),
    SubmittedLogin,
    SubmittedCode,
}

#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Input,
    QRCode,
}

pub enum Instruction {
    Login { username: String, _password: String },
    Authenticated,
}

#[derive(Debug, Clone, Default)]
pub struct Login {
    secret: Secret<'static>,
    state: State,
    code: String,
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn view(&self) -> Element<'_, Message> {
        let internal = match &self.state {
            State::Input => {
                let text = container(text("LOGIN").center()).padding(Padding {
                    top: 50.,
                    bottom: 50.,
                    left: 80.,
                    ..Default::default()
                });

                let username = text_input("Enter username...", &self.username)
                    .on_input(Message::UpdatedUsername)
                    .on_submit(Message::SubmittedLogin);

                let password = text_input("Enter password...", &self.password)
                    .on_input(Message::UpdatedPassword)
                    .on_submit(Message::SubmittedLogin);

                let username_container = container(username).padding([10, 0]);
                let password_container = container(password).padding([10, 0]);
                column![text, username_container, password_container]
            }
            State::QRCode => {
                info!("Generating QR code");
                // Then use in URL
                let url = format!(
                    "otpauth://totp/MagiChat:jonat@localhost?secret={}&issuer=MagiChat&algorithm=SHA1&digits=6&period=30",
                    self.secret
                );
                let qrcode = QRBuilder::new(url).build().unwrap();

                info!("QR code generated and converting to svg");
                let svg = SvgBuilder::default()
                    .shape(Shape::RoundedSquare)
                    .to_str(&qrcode);

                info!("SVG generated");
                let code_input = text_input("Enter code...", &self.code)
                    .on_input(Message::UpdatedCode)
                    .on_submit(Message::SubmittedCode);

                column![
                    iced::widget::svg(svg::Handle::from_memory(svg.into_bytes()))
                        .height(300)
                        .width(300),
                    code_input
                ]
            }
        };

        let style = |theme: &Theme| container::Style {
            border: iced::Border {
                width: 2.,
                radius: Radius::new(10.),
                ..iced::Border::default()
            },
            ..container::rounded_box(theme)
        };

        container(internal)
            .align_x(iced::alignment::Horizontal::Center)
            .style(style)
            .height(400)
            .width(300)
            .padding([0, 50])
            .into()
    }

    pub fn update(&mut self, msg: Message) -> Action<Instruction, Message> {
        match msg {
            Message::UpdatedUsername(username) => {
                self.username = username;
                Action::none()
            }
            Message::UpdatedPassword(password) => {
                self.password = password;
                Action::none()
            }
            Message::UpdatedCode(code) => {
                self.code = code;
                Action::none()
            }
            Message::SubmittedLogin => {
                info!("Submitted username: {}", self.username);
                self.secret = generate_totp_secret();
                self.state = State::QRCode;
                Action::instruction(Instruction::Login {
                    username: self.username.clone(),
                    _password: self.password.clone(),
                })
            }
            Message::SubmittedCode => {
                info!("Submitted code: {}", self.code);
                let base = otp_std::Base::builder()
                    .secret(self.secret.clone())
                    .algorithm(Algorithm::Sha1)
                    .digits(Digits::new(6).unwrap())
                    .build();

                let totp = Totp::builder()
                    .base(base)
                    .period(Period::new(30).expect("Could not create period"))
                    .skew(Skew::disabled())
                    .build();

                let code = totp.generate();
                info!("Generated code: {}", code);
                if !code.eq(&self.code.parse::<u32>().unwrap()) {
                    return Action::none();
                }
                Action::instruction(Instruction::Authenticated)
            }
        }
    }
}

fn generate_totp_secret() -> Secret<'static> {
    let mut bytes = [0u8; 20]; // 160 bits
    rand::rng().fill_bytes(&mut bytes);
    Secret::owned(bytes.to_vec()).unwrap()
}
