use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64::kernel32;

mod marshaling;
mod objects;
mod runtime;
mod storage;

pub use marshaling::*;
pub use objects::*;
pub use runtime::*;
pub use storage::*;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if objects::dispatch(api, emu)
        || runtime::dispatch(api, emu)
        || marshaling::dispatch(api, emu)
        || storage::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}

fn read_guid_string(emu: &emu::Emu, addr: u64) -> String {
    let d1 = emu.maps.read_dword(addr).unwrap_or(0);
    let d2 = emu.maps.read_word(addr + 4).unwrap_or(0);
    let d3 = emu.maps.read_word(addr + 6).unwrap_or(0);
    let mut d4 = [0u8; 8];
    for (i, byte) in d4.iter_mut().enumerate() {
        *byte = emu.maps.read_byte(addr + 8 + i as u64).unwrap_or(0);
    }
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
    )
}
