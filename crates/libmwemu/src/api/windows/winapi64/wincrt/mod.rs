use crate::emu::Emu;
use crate::winapi::libraries::wincrt;
use crate::winapi::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut Emu) -> String {
    let name = kernel32::guess_api_name(emu, addr);
    let api = name.rsplit('!').next().unwrap_or(&name);
    wincrt::gateway(api, addr, emu)
}
