use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EtwEventEnabled(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reg_handle = api_try!(emu, call.arg(emu, 0));
    let event_descriptor = api_try!(emu, call.arg(emu, 1));
    let level = if event_descriptor != 0 {
        emu.maps.read_byte(event_descriptor).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "kernel32!EventEnabled handle:0x{:x} level:{}",
        reg_handle,
        level
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
