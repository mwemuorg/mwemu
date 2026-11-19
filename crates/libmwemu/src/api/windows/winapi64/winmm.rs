use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
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
    helper::advance_tick(emu, 1);
    call.set_return(emu, emu.tick as u64);
}
