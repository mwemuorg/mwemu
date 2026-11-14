use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64::kernel32;
//
pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    fallback::unimplemented(addr, api, emu)
}
