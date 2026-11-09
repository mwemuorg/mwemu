use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod launch;
pub use launch::*;
mod filesystem;
pub use filesystem::*;
mod ui;
pub use ui::*;
mod objects;
pub use objects::*;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if launch::dispatch(api, emu)
        || filesystem::dispatch(api, emu)
        || ui::dispatch(api, emu)
        || objects::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}

fn csidl_to_path(csidl: i32) -> Option<&'static str> {
    match csidl & 0xFFFF {
        0x0000 => Some("C:\\Users\\User\\Desktop"),
        0x0002 => {
            Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs")
        }
        0x0005 => Some("C:\\Users\\User\\Documents"),
        0x0006 => Some("C:\\Users\\User\\Favorites"),
        0x0007 => Some(
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        ),
        0x0008 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Recent"),
        0x0009 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\SendTo"),
        0x000b => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu"),
        0x000d => Some("C:\\Users\\User\\Music"),
        0x000e => Some("C:\\Users\\User\\Videos"),
        0x0010 => Some("C:\\Users\\User\\Desktop"),
        0x0013 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Network Shortcuts"),
        0x0014 => Some("C:\\Windows\\Fonts"),
        0x0015 => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Templates"),
        0x0016 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu"),
        0x0017 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs"),
        0x0018 => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
        0x0019 => Some("C:\\Users\\Public\\Desktop"),
        0x001a => Some("C:\\Users\\User\\AppData\\Roaming"),
        0x001b => Some("C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Printer Shortcuts"),
        0x001c => Some("C:\\Users\\User\\AppData\\Local"),
        0x001d => Some(
            "C:\\Users\\User\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        ),
        0x001f => Some("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
        0x0020 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCache"),
        0x0021 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\INetCookies"),
        0x0022 => Some("C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\History"),
        0x0023 => Some("C:\\ProgramData"),
        0x0024 => Some("C:\\Windows"),
        0x0025 => Some("C:\\Windows\\System32"),
        0x0026 => Some("C:\\Program Files"),
        0x0027 => Some("C:\\Users\\User\\Pictures"),
        0x0028 => Some("C:\\Users\\User"),
        0x0029 => Some("C:\\Windows\\SysWOW64"),
        0x002a => Some("C:\\Program Files (x86)"),
        0x002b => Some("C:\\Program Files\\Common Files"),
        0x002c => Some("C:\\Program Files (x86)\\Common Files"),
        0x002d => Some("C:\\ProgramData\\Microsoft\\Windows\\Templates"),
        0x002e => Some("C:\\Users\\Public\\Documents"),
        0x0035 => Some("C:\\Users\\Public\\Music"),
        0x0036 => Some("C:\\Users\\Public\\Pictures"),
        0x0037 => Some("C:\\Users\\Public\\Videos"),
        0x0038 => Some("C:\\Windows\\Resources"),
        0x003b => Some(
            "C:\\Users\\User\\AppData\\Local\\Microsoft\\Windows\\Temporary Internet Files\\Low",
        ),
        _ => None,
    }
}
