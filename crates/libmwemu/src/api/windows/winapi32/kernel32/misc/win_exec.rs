use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WinExec(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cmdline_ptr = api_try!(emu, call.arg(emu, 0));
    let cmdline = emu.maps.read_string(cmdline_ptr);

    //emu.spawn_console();

    log_red!(emu, "WinExec  '{}'", cmdline);

    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}
