use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::process::*;

pub fn WaitForMultipleObjects(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg(emu, 0));
    let handles_ptr = api_try!(emu, call.arg(emu, 1));
    let wait_all = api_try!(emu, call.arg(emu, 2));
    let millis = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "kernel32!WaitForMultipleObjects count: {} handles: 0x{:x} waitAll: {} millis: {}",
        count,
        handles_ptr,
        wait_all,
        millis
    );

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.set_last_error(0);
    call.set_return(emu, WAIT_OBJECT_0);
}
