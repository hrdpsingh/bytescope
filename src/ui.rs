use crate::pages::{cpu, overview};
use crate::styles::SidebarButton;
use crate::types::{Message, Page};

use iced::{
    Application, Color, Command, Element, Length, Theme,
    widget::{button, column, container, row},
};

pub struct Probe {
    pub page: Page,
}

impl Application for Probe {
    type Theme = Theme;
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                page: Page::Overview,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Probe")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Navigate(page) => {
                self.page = page;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                button("Overview")
                    .on_press(Message::Navigate(Page::Overview))
                    .padding(10)
                    .width(Length::Fill)
                    .style(iced::theme::Button::Custom(Box::new(SidebarButton {
                        active: self.page == Page::Overview,
                    }))),
                button("CPU")
                    .on_press(Message::Navigate(Page::Cpu))
                    .padding(10)
                    .width(Length::Fill)
                    .style(iced::theme::Button::Custom(Box::new(SidebarButton {
                        active: self.page == Page::Cpu,
                    }))),
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fixed(150.0))
            .height(Length::Fill),
        )
        .style(|_theme: &Theme| container::Appearance {
            background: Some(Color::from_rgb8(240, 245, 250).into()),
            ..Default::default()
        })
        .height(Length::Fill);

        let content = match self.page {
            Page::Overview => overview::view(),
            Page::Cpu => cpu::view(),
        };

        row![sidebar, content].into()
    }
}
