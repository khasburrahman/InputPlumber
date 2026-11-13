use std::mem;

/// Events that can be emitted by the ADVAN driver
#[derive(Clone, Debug)]
pub enum Event {
    GamepadButton(GamepadButtonEvent),
}

impl Event {
    /// Whether two events are the same type, ignoring payload data
    pub fn variant_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Event::GamepadButton(a), Event::GamepadButton(b)) => a.variant_eq(b),
        }
    }
}

/// Binary input contains either pressed or unpressed
#[derive(Clone, Debug, Default)]
pub struct BinaryInput {
    pub pressed: bool,
}

/// Gamepad button events represent binary button presses
#[derive(Clone, Debug)]
pub enum GamepadButtonEvent {
    /// Quick Access Menu button
    QuickAccess(BinaryInput),
}

impl GamepadButtonEvent {
    /// Whether the button is pressed
    pub fn pressed(&self) -> bool {
        match self {
            Self::QuickAccess(input) => input.pressed,
        }
    }

    /// Whether two events are the same type, ignoring payload data
    pub fn variant_eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
