use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn Sleep(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let millis = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(
        emu,
        "kernel32!Sleep millis: {}{}",
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );

    if emu.cfg.short_circuit_sleep {
        emu.tick += 1;
    } else {
        emu.tick += millis as usize;
    }

    call.cleanup_args(emu, 1);
}
