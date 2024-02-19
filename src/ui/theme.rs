use iced::border::Radius;
use iced::widget::{self, button, scrollable};
use iced::{Background, Border, Color, Shadow};

pub enum ConnectionStyle {
    Selected,
    Unselected,
}

impl iced::widget::button::StyleSheet for ConnectionStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            ConnectionStyle::Selected => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.3, 0.4))),
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
            ConnectionStyle::Unselected => button::Appearance {
                background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
        }
    }
}

pub struct MyScrollableStyle;

impl scrollable::StyleSheet for MyScrollableStyle {
    type Style = ();

    fn active(&self, style: &Self::Style) -> scrollable::Appearance {
        // Define the appearance of the active scrollbar here
        scrollable::Appearance {
            container: widget::container::Appearance {
                text_color: Some(Color::from_rgba8(0, 0, 0, 1.0)), // Option<Color>
                background: Some(Background::Color(Color::from_rgba8(240, 240, 240, 0.7))), // Option<Background>
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 2.0,
                    radius: Radius::default(),
                }, // Border
                shadow: Shadow::default(), // Shadow,
            },
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(Color::from_rgba8(240, 240, 240, 0.7))),
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 2.0,
                    radius: Radius::default(),
                },
                scroller: scrollable::Scroller {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    border: Border {
                        color: Color::from_rgb(0.5, 0.5, 0.5),
                        width: 2.0,
                        radius: Radius::default(),
                    },
                },
            },

            gap: None,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        scrollable::Appearance {
            container: widget::container::Appearance {
                text_color: Some(Color::from_rgba8(0, 0, 0, 1.0)), // Option<Color>
                background: Some(Background::Color(Color::from_rgba8(240, 240, 240, 0.7))), // Option<Background>
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 2.0,
                    radius: Radius::default(),
                }, // Border
                shadow: Shadow::default(), // Shadow,
            },
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(Color::from_rgba8(240, 240, 240, 0.7))),
                border: Border {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    width: 2.0,
                    radius: Radius::default(),
                },
                scroller: scrollable::Scroller {
                    color: Color::from_rgb(0.5, 0.5, 0.5),
                    border: Border {
                        color: Color::from_rgb(0.5, 0.5, 0.5),
                        width: 2.0,
                        radius: Radius::default(),
                    },
                },
            },

            gap: None,
        }
    }
}
