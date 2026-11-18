use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split('!').next_back().unwrap_or(&api);
    match api {
        "timeGetTime" => timeGetTime(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

fn timeGetTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "winmm!timeGetTime");
    let return_value = emu.tick as u64;
    call.set_return(emu, return_value);
}
