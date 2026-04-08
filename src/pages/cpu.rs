use iced::{
    Color, Element, Length, Theme,
    widget::{container, text},
};

pub fn view<'a, Message: 'a>() -> Element<'a, Message> {
    container(text("CPU page"))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(|_theme: &Theme| container::Appearance {
            background: Some(Color::from_rgb8(230, 240, 250).into()),
            ..Default::default()
        })
        .into()
}
