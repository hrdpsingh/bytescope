use crate::{app::Message, components::category};
use iced::Element;
use iced::widget::{column, text};

pub fn view<'a>() -> Element<'a, Message> {
    column![category::view("Kernel", column![text("Version")]),]
        .spacing(20)
        .into()
}
