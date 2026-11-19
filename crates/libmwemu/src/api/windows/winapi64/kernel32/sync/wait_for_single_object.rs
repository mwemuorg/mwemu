use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::process::*;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    let millis = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "kernel32!WaitForSingleObject  hndl: {} millis: {}{}",
        hndl,
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.set_last_error(0);
    call.set_return(emu, WAIT_TIMEOUT);
}
