use std::{collections::HashSet, error::Error, fmt::Debug};

use crate::{
    drivers::advan::{driver::Driver, event},
    input::{
        capability::{Capability, Gamepad, GamepadButton},
        event::{native::NativeEvent, value::InputValue},
        source::{EventCapability, InputError, SourceInputDevice, SourceOutputDevice},
    },
    udev::device::UdevDevice,
};

/// ADVAN game controller source device implementation.
/// Reports the Quick Access Menu button from the vendor HID interface.
pub struct Advan {
    driver: Driver,
}

impl Advan {
    pub fn new(device_info: UdevDevice) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let driver = Driver::new(device_info.devnode())?;
        Ok(Self { driver })
    }
}

impl SourceInputDevice for Advan {
    fn poll(&mut self) -> Result<Vec<NativeEvent>, InputError> {
        let events = self.driver.poll()?;
        Ok(translate_events(events))
    }

    fn update_event_filter(&mut self, events: HashSet<Capability>) -> Result<(), InputError> {
        let filtered = events
            .iter()
            .filter_map(EventCapability::from_capability)
            .collect();
        self.driver.update_filtered_events(filtered);
        Ok(())
    }

    fn get_capabilities(&self) -> Result<Vec<Capability>, InputError> {
        Ok(CAPABILITIES.into())
    }
}

impl SourceOutputDevice for Advan {}

impl Debug for Advan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Advan").finish()
    }
}

fn translate_events(events: Vec<event::Event>) -> Vec<NativeEvent> {
    events.into_iter().filter_map(translate_event).collect()
}

fn translate_event(event: event::Event) -> Option<NativeEvent> {
    let value = event.to_value()?;
    Some(NativeEvent::new(event.to_capability(), value))
}

impl EventCapability for event::Event {
    fn to_value(&self) -> Option<InputValue> {
        match self {
            event::Event::GamepadButton(button) => Some(InputValue::Bool(button.pressed())),
        }
    }

    fn to_capability(&self) -> Capability {
        match self {
            event::Event::GamepadButton(button) => match button {
                event::GamepadButtonEvent::QuickAccess(_) => {
                    Capability::Gamepad(Gamepad::Button(GamepadButton::QuickAccess))
                }
            },
        }
    }

    fn from_capability(capability: &Capability) -> Option<Self> {
        match capability {
            Capability::Gamepad(Gamepad::Button(GamepadButton::QuickAccess)) => {
                Some(event::Event::GamepadButton(
                    event::GamepadButtonEvent::QuickAccess(Default::default()),
                ))
            }
            _ => None,
        }
    }
}

/// List of all input capabilities that the ADVAN driver implements
pub const CAPABILITIES: &[Capability] = &[Capability::Gamepad(Gamepad::Button(
    GamepadButton::QuickAccess,
))];
