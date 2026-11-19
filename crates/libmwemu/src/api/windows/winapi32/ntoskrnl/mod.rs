use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod rtl_init_unicode_string;
use rtl_init_unicode_string::RtlInitUnicodeString;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    dispatch_api!(api, {
        "RtlInitUnicodeString" => RtlInitUnicodeString(emu),
    },
        handled(()) => String::new(),
        fallback => fallback::unimplemented(addr, api, emu),
    )
}
