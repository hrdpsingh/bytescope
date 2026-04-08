use iced::{Background, Color, Theme, widget::button};

pub struct SidebarButton {
    pub active: bool,
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

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(230, 240, 250))),
            text_color: Color::from_rgb8(0, 0, 0),
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Default::default()
        }
    }
}
