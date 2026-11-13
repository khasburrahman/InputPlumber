use std::{error::Error, ffi::CString};

use hidapi::HidDevice;

use super::{
    event::{BinaryInput, Event, GamepadButtonEvent},
    HID_TIMEOUT, PACKET_SIZE, PID, QAM_BYTE_INDEX, QAM_BYTE_MASK, REPORT_ID_BUTTON,
    REPORT_ID_BUTTON_ALT, VID,
};

pub struct Driver {
    device: HidDevice,
    qam_pressed: bool,
    filtered_events: Vec<Event>,
}

impl Driver {
    pub fn new(path: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let cs_path = CString::new(path.clone())?;
        let api = hidapi::HidApi::new()?;
        let device = api.open_path(&cs_path)?;
        let info = device.get_device_info()?;
        if info.vendor_id() != VID || info.product_id() != PID {
            return Err(format!("Device '{path}' is not an ADVAN game controller").into());
        }

        Ok(Self {
            device,
            qam_pressed: false,
            filtered_events: vec![],
        })
    }

    pub fn update_filtered_events(&mut self, events: Vec<Event>) {
        self.filtered_events = events;
    }

    fn is_filtered(&self, event: &Event) -> bool {
        self.filtered_events.iter().any(|f| f.variant_eq(event))
    }

    fn emit(&mut self, events: &mut Vec<Event>, event: Event) {
        if !self.is_filtered(&event) {
            events.push(event);
        }
    }

    /// Poll the device and read input reports
    pub fn poll(&mut self) -> Result<Vec<Event>, Box<dyn Error + Send + Sync>> {
        let mut buf = [0u8; PACKET_SIZE];
        let bytes_read = self.device.read_timeout(&mut buf, HID_TIMEOUT)?;

        if bytes_read != PACKET_SIZE {
            return Ok(Vec::new());
        }

        if buf[0] != REPORT_ID_BUTTON && buf[0] != REPORT_ID_BUTTON_ALT {
            return Ok(Vec::new());
        }

        let pressed = buf[QAM_BYTE_INDEX] & QAM_BYTE_MASK != 0;
        if pressed == self.qam_pressed {
            return Ok(Vec::new());
        }
        self.qam_pressed = pressed;

        let mut events = Vec::new();
        self.emit(
            &mut events,
            Event::GamepadButton(GamepadButtonEvent::QuickAccess(BinaryInput { pressed })),
        );
        Ok(events)
    }
}
