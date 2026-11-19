use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WinExec(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cmdline_ptr = api_try!(emu, call.arg(emu, 0));
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!WinExec  '{}'", cmdline);

    emu.set_last_error(0);
    call.set_return(emu, 32);
}
