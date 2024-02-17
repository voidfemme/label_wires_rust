use iced::widget::button;
use iced::{Background, Color};

pub enum ConnectionStyle {
    Selected,
    Unselected,
}

impl button::StyleSheet for ConnectionStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            ConnectionStyle::Selected => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.3, 0.4))),
                text_color: Color::WHITE,
                border_radius: 0.0.into(),
                ..button::Appearance::default()
            },
            ConnectionStyle::Unselected => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
                text_color: Color::WHITE,
                border_radius: 0.0.into(),
                ..button::Appearance::default()
            },
        }
    }
}
