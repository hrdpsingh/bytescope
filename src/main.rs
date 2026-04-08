use iced::{
    Alignment, Background, Color, Element, Length, Sandbox, Settings, Theme,
    widget::{button, column, container, row, text},
};

fn main() -> iced::Result {
    Probe::run(Settings::default())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Overview,
    Cpu,
}

#[derive(Debug, Clone)]
enum Message {
    Navigate(Page),
}

struct Probe {
    page: Page,
}

struct SidebarButton {
    active: bool,
}

impl button::StyleSheet for SidebarButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let background = if self.active {
            Color::from_rgb8(230, 240, 250)
        } else {
            Color::from_rgb8(240, 245, 250)
        };

        button::Appearance {
            background: Some(Background::Color(background)),
            text_color: Color::from_rgb8(0, 0, 0),
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..style.active(&iced::theme::Button::Primary)
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(230, 240, 250))),
            text_color: Color::from_rgb8(0, 0, 0),
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..style.active(&iced::theme::Button::Primary)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        style.disabled(&iced::theme::Button::Primary)
    }
}

impl Sandbox for Probe {
    type Message = Message;

    fn new() -> Self {
        Self {
            page: Page::Overview,
        }
    }

    fn title(&self) -> String {
        String::from("Probe")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                button("Overview")
                    .on_press(Message::Navigate(Page::Overview))
                    .width(Length::Fill)
                    .style(iced::theme::Button::Custom(Box::new(SidebarButton {
                        active: self.page == Page::Overview,
                    }))),
                button("CPU")
                    .on_press(Message::Navigate(Page::Cpu))
                    .width(Length::Fill)
                    .style(iced::theme::Button::Custom(Box::new(SidebarButton {
                        active: self.page == Page::Cpu,
                    }))),
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fixed(150.0))
            .height(Length::Fill)
            .align_items(Alignment::Start),
        )
        .style(|_theme: &Theme| container::Appearance {
            background: Some(Color::from_rgb8(240, 245, 250).into()),
            ..Default::default()
        });

        let content = match self.page {
            Page::Overview => container(text("Overview page"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(|_theme: &Theme| container::Appearance {
                    background: Some(Color::from_rgb8(230, 240, 250).into()),
                    ..Default::default()
                }),
            Page::Cpu => container(text("CPU page"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(|_theme: &Theme| container::Appearance {
                    background: Some(Color::from_rgb8(230, 240, 250).into()),
                    ..Default::default()
                }),
        };

        row![sidebar, content]
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
