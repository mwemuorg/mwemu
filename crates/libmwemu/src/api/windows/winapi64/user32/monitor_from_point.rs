use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn MonitorFromPoint(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pt_x = api_try!(emu, call.arg_i32(emu, 0));
    let pt_y = api_try!(emu, call.arg_i32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let hmon = helper::handler_create("monitor");
    log_red!(
        emu,
        "user32!MonitorFromPoint ({},{}) flags:0x{:x} => 0x{:x}",
        pt_x,
        pt_y,
        flags,
        hmon
    );
    emu.set_last_error(0);
    call.set_return(emu, hmon);
}
