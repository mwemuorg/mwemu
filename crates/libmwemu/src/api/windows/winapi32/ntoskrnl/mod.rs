use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod rtl_init_unicode_string;
use rtl_init_unicode_string::RtlInitUnicodeString;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
