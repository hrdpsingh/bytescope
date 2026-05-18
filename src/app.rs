use crate::{
    components::sidebar,
    models::Page,
    pages::{firmware, hardware, software},
};
use iced::{
    Color, Element, Length, Subscription, time,
    widget::{container, row},
};
use std::time::Duration;
use sysinfo::System;

pub struct Probe {
    page: Page,
    system: System,
    cpu_usage_history: Vec<f32>,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            page: Page::default(),
            system: System::new_all(),
            cpu_usage_history: vec![0.0; 60],
        }
    }
}

#[derive(Clone)]
pub enum Message {
    Navigate(Page),
    Tick,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
            Message::Tick => {
                self.system.refresh_cpu_all();
                let cpu_usage = self.system.global_cpu_usage();
                self.cpu_usage_history.remove(0);
                self.cpu_usage_history.push(cpu_usage);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Software => software::view(),
            Page::Hardware => hardware::view(&self.cpu_usage_history),
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

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
