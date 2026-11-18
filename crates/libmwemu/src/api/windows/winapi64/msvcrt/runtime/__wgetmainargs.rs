use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __wgetmainargs(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let p_argc = api_try!(emu, call.arg(emu, 0));
    let p_argv = api_try!(emu, call.arg(emu, 1));
    let p_env = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "msvcrt!__wgetmainargs");
    if p_argc != 0 {
        emu.maps.write_qword(p_argc, 1);
    }
    if p_argv != 0 {
        emu.maps.write_qword(p_argv, 0);
    }
    if p_env != 0 {
        emu.maps.write_qword(p_env, 0);
    }
    call.set_return(emu, 0);
}
