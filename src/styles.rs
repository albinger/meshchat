use iced::border::Radius;
use iced::{Background, Border, Color};

pub const TEXT_INPUT_BACKGROUND: Background =
    Background::Color(Color::from_rgba(0.25, 0.25, 0.25, 1.0));

const TEXT_INPUT_R: f32 = 20.0;

const TEXT_INPUT_RADIUS: Radius = Radius {
    top_left: TEXT_INPUT_R,
    top_right: TEXT_INPUT_R,
    bottom_right: TEXT_INPUT_R,
    bottom_left: TEXT_INPUT_R,
};

pub const TEXT_INPUT_BORDER: Border = Border {
    radius: TEXT_INPUT_RADIUS, // rounded corners
    width: 2.0,
    color: Color::WHITE,
};

pub const TEXT_INPUT_PLACEHOLDER_COLOR: Color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
