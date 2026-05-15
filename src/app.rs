use crate::{
    components::sidebar,
    models::Page,
    pages::{firmware, hardware, software},
};

use iced::{
    Color, Element, Length,
    widget::{container, row},
};

#[derive(Default)]
pub struct Probe {
    page: Page,
}

#[derive(Clone)]
pub enum Message {
    Navigate(Page),
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Software => software::view(),
            Page::Hardware => hardware::view(),
            Page::Firmware => firmware::view(),
        };

        container(
            row![
                sidebar::view(self.page),
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
            ]
            .spacing(40),
        )
        .padding(40)
        .style(|_| container::Style::default().background(Color::from_rgb8(235, 240, 245)))
        .into()
    }
}
