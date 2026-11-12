use crate::emu::Emu;
use crate::winapi::common::memory::Call;
use crate::windows::constants::nls::EN_US_LOCALE;
use crate::windows::constants::system::*;

// Scan codes for the fixed US keyboard profile, indexed by ASCII letter/digit.
const LETTER_SCAN_CODES: [u8; 26] = [
    0x1e, 0x30, 0x2e, 0x20, 0x12, 0x21, 0x22, 0x23, 0x17, 0x24, 0x25, 0x26, 0x32, 0x31, 0x18, 0x19,
    0x10, 0x13, 0x1f, 0x14, 0x16, 0x2f, 0x11, 0x2d, 0x15, 0x2c,
];
const DIGIT_SCAN_CODES: [u8; 10] = [0x0b, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a];
const CONTROL_SCAN_CODES: [(u32, u8); 5] = [
    (VK_BACK, 0x0e),
    (VK_TAB, 0x0f),
    (VK_RETURN, 0x1c),
    (VK_ESCAPE, 0x01),
    (VK_SPACE, 0x39),
];

fn scan_code(key: u32) -> u32 {
    if (b'A' as u32..=b'Z' as u32).contains(&key) {
        LETTER_SCAN_CODES[(key - b'A' as u32) as usize] as u32
    } else if (b'0' as u32..=b'9' as u32).contains(&key) {
        DIGIT_SCAN_CODES[(key - b'0' as u32) as usize] as u32
    } else {
        CONTROL_SCAN_CODES
            .iter()
            .find(|(value, _)| *value == key)
            .map(|(_, scan)| *scan as u32)
            .unwrap_or(0)
    }
}

fn virtual_key(scan: u32) -> u32 {
    if let Some(index) = LETTER_SCAN_CODES
        .iter()
        .position(|value| *value as u32 == scan)
    {
        b'A' as u32 + index as u32
    } else if let Some(index) = DIGIT_SCAN_CODES
        .iter()
        .position(|value| *value as u32 == scan)
    {
        b'0' as u32 + index as u32
    } else {
        CONTROL_SCAN_CODES
            .iter()
            .find(|(_, value)| *value as u32 == scan)
            .map(|(key, _)| *key)
            .unwrap_or(0)
    }
}

pub(crate) fn map_virtual_key(emu: &mut Emu, api: &str, extended: bool) {
    let code = Call::Api.arg(emu, 0) as u32;
    let kind = Call::Api.arg(emu, 1) as u32;
    let layout = if extended { Call::Api.arg(emu, 2) } else { 0 };
    let result = if layout != 0 && layout as u16 as u32 != EN_US_LOCALE {
        0
    } else {
        match kind {
            MAPVK_VK_TO_VSC | MAPVK_VK_TO_VSC_EX => scan_code(code),
            MAPVK_VSC_TO_VK | MAPVK_VSC_TO_VK_EX => virtual_key(code),
            MAPVK_VK_TO_CHAR
                if (b'A' as u32..=b'Z' as u32).contains(&code)
                    || (b'0' as u32..=b'9' as u32).contains(&code)
                    || CONTROL_SCAN_CODES.iter().any(|(key, _)| *key == code) =>
            {
                code
            }
            _ => 0,
        }
    };
    log_red!(
        emu,
        "user32!{} code:0x{:x} type:{} layout:0x{:x} => 0x{:x}",
        api,
        code,
        kind,
        layout,
        result
    );
    Call::Api.finish(emu, result as u64, if extended { 3 } else { 2 });
}
