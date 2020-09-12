#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Gamepad {
    pub id: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GamepadEventType {
    Connected,
    Disconnected,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GamepadEvent {
    pub gamepad: Gamepad,
    pub event_type: GamepadEventType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonCode {
    South,
    East,
    North,
    West,
    C,
    Z,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GamepadButton {
    pub gamepad: Gamepad,
    pub code: ButtonCode,
}

impl GamepadButton {
    pub fn new(gamepad: Gamepad, code: ButtonCode) -> GamepadButton {
        GamepadButton { gamepad, code }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AxisCode {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GamepadAxis {
    pub gamepad: Gamepad,
    pub code: AxisCode,
}

impl GamepadAxis {
    pub fn new(gamepad: Gamepad, code: AxisCode) -> GamepadAxis {
        GamepadAxis { gamepad, code }
    }
}
