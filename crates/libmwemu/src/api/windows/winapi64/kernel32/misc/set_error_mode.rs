use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let mode = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!SetErrorMode 0x{:x}", mode);
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    call.set_return(emu, 0); // return previous error mode
}
