pub mod driver;
pub mod event;

pub const VID: u16 = 0x2f24;
pub const PID: u16 = 0x0135;
pub const IID: i32 = 0x03;

const PACKET_SIZE: usize = 32;

// Report IDs used for the vendor input reports that carry the QAM state.
const REPORT_ID_BUTTON: u8 = 0x02;
const REPORT_ID_BUTTON_ALT: u8 = 0x03;

const QAM_BYTE_INDEX: usize = 9;
const QAM_BYTE_MASK: u8 = 0x01;

// HID buffer read timeout
const HID_TIMEOUT: i32 = 10;
