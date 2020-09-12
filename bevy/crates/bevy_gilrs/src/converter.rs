use bevy_input::gamepad::{AxisCode, ButtonCode, Gamepad};

pub fn convert_gamepad_id(gamepad_id: gilrs::GamepadId) -> Gamepad {
    Gamepad {
        id: gamepad_id.into(),
    }
}

pub fn convert_button(button: gilrs::Button) -> Option<ButtonCode> {
    match button {
        gilrs::Button::South => Some(ButtonCode::South),
        gilrs::Button::East => Some(ButtonCode::East),
        gilrs::Button::North => Some(ButtonCode::North),
        gilrs::Button::West => Some(ButtonCode::West),
        gilrs::Button::C => Some(ButtonCode::C),
        gilrs::Button::Z => Some(ButtonCode::Z),
        gilrs::Button::LeftTrigger => Some(ButtonCode::LeftTrigger),
        gilrs::Button::LeftTrigger2 => Some(ButtonCode::LeftTrigger2),
        gilrs::Button::RightTrigger => Some(ButtonCode::RightTrigger),
        gilrs::Button::RightTrigger2 => Some(ButtonCode::RightTrigger2),
        gilrs::Button::Select => Some(ButtonCode::Select),
        gilrs::Button::Start => Some(ButtonCode::Start),
        gilrs::Button::Mode => Some(ButtonCode::Mode),
        gilrs::Button::LeftThumb => Some(ButtonCode::LeftThumb),
        gilrs::Button::RightThumb => Some(ButtonCode::RightThumb),
        gilrs::Button::DPadUp => Some(ButtonCode::DPadUp),
        gilrs::Button::DPadDown => Some(ButtonCode::DPadDown),
        gilrs::Button::DPadLeft => Some(ButtonCode::DPadLeft),
        gilrs::Button::DPadRight => Some(ButtonCode::DPadRight),
        gilrs::Button::Unknown => None,
    }
}

pub fn convert_axis(axis: gilrs::Axis) -> Option<AxisCode> {
    match axis {
        gilrs::Axis::LeftStickX => Some(AxisCode::LeftStickX),
        gilrs::Axis::LeftStickY => Some(AxisCode::LeftStickY),
        gilrs::Axis::LeftZ => Some(AxisCode::LeftZ),
        gilrs::Axis::RightStickX => Some(AxisCode::RightStickX),
        gilrs::Axis::RightStickY => Some(AxisCode::RightStickY),
        gilrs::Axis::RightZ => Some(AxisCode::RightZ),
        gilrs::Axis::DPadX => Some(AxisCode::DPadX),
        gilrs::Axis::DPadY => Some(AxisCode::DPadY),
        gilrs::Axis::Unknown => None,
    }
}
