use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split('!').next_back().unwrap_or(&api);
    dispatch_api!(api, {
        "timeGetTime" => timeGetTime(emu),
    },
        handled(()) => String::new(),
        fallback => fallback::unimplemented(addr, api, emu),
    )
}

fn timeGetTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "winmm!timeGetTime");
    let return_value = emu.tick as u64;
    call.set_return(emu, return_value);
}
