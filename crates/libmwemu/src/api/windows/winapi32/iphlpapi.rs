use crate::emu::Emu;
use crate::winapi::common::{fallback, iphlpapi};
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if iphlpapi::dispatch(api, emu) {
        return String::new();
    }
    fallback::unimplemented(addr, api, emu)
}
