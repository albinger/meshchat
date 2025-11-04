use iced::border::Radius;
use iced::widget::text_input;
use iced::widget::text_input::Status;
use iced::{Background, Border, Color, Shadow, Theme};

pub const TEXT_INPUT_BACKGROUND: Background =
    Background::Color(Color::from_rgba(0.25, 0.25, 0.25, 1.0));

const TEXT_INPUT_R: f32 = 20.0;

const RADIUS_2: Radius = Radius {
    top_left: 2.0,
    top_right: 2.0,
    bottom_right: 2.0,
    bottom_left: 2.0,
};

pub const NO_SHADOW: Shadow = Shadow {
    color: Color::TRANSPARENT,
    offset: iced::Vector { x: 0.0, y: 0.0 },
    blur_radius: 0.0,
};

pub const NO_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 0.0,
    radius: RADIUS_2,
};

pub const RADIUS_12: Radius = Radius {
    top_left: 12.0,
    top_right: 12.0,
    bottom_right: 12.0,
    bottom_left: 12.0,
};

pub const WHITE_BORDER: Border = Border {
    color: Color::WHITE,
    width: 2.0,
    radius: RADIUS_2,
};

const TEXT_INPUT_RADIUS: Radius = Radius {
    top_left: TEXT_INPUT_R,
    top_right: TEXT_INPUT_R,
    bottom_right: TEXT_INPUT_R,
    bottom_left: TEXT_INPUT_R,
};

const TEXT_INPUT_BORDER_ACTIVE: Border = Border {
    radius: TEXT_INPUT_RADIUS, // rounded corners
    width: 2.0,
    color: Color::from_rgba(0.0, 0.8, 0.8, 1.0), // Cyan
};

const TEXT_INPUT_BORDER: Border = Border {
    radius: TEXT_INPUT_RADIUS, // rounded corners
    width: 2.0,
    color: Color::WHITE,
};

const TEXT_INPUT_BORDER_DISABLED: Border = Border {
    radius: TEXT_INPUT_RADIUS, // rounded corners
    width: 2.0,
    color: Color::TRANSPARENT,
};

pub const TEXT_INPUT_PLACEHOLDER_COLOR: Color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);

pub fn text_input_style(_theme: &Theme, status: Status) -> text_input::Style {
    match status {
        Status::Active => text_input::Style {
            background: TEXT_INPUT_BACKGROUND,
            border: TEXT_INPUT_BORDER,
            icon: Color::WHITE,
            placeholder: TEXT_INPUT_PLACEHOLDER_COLOR,
            value: Color::WHITE,
            selection: Default::default(),
        },
        Status::Hovered => text_input::Style {
            background: TEXT_INPUT_BACKGROUND,
            border: TEXT_INPUT_BORDER,
            icon: Color::WHITE,
            placeholder: TEXT_INPUT_PLACEHOLDER_COLOR,
            value: Color::WHITE,
            selection: Default::default(),
        },
        Status::Focused => text_input::Style {
            background: TEXT_INPUT_BACKGROUND,
            border: TEXT_INPUT_BORDER_ACTIVE,
            icon: Color::WHITE,
            placeholder: TEXT_INPUT_PLACEHOLDER_COLOR,
            value: Color::WHITE,
            selection: Default::default(),
        },
        Status::Disabled => text_input::Style {
            background: TEXT_INPUT_BACKGROUND,
            border: TEXT_INPUT_BORDER_DISABLED,
            icon: Color::WHITE,
            placeholder: TEXT_INPUT_PLACEHOLDER_COLOR,
            value: Color::WHITE,
            selection: Default::default(),
        },
    }
}
