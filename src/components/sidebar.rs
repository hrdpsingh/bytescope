use crate::{app::Message, components::button, models::Page};
use iced::{
    Color, Element, Length,
    widget::{column, container},
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    container(column![
        button::view("Software", Page::Software, page),
        button::view("Hardware", Page::Hardware, page),
        button::view("Firmware", Page::Firmware, page),
    ])
    .width(120)
    .height(Length::Fill)
    .padding(10)
    .style(|_| container::Style::default().background(Color::from_rgb8(255, 255, 255)))
    .into()
}
