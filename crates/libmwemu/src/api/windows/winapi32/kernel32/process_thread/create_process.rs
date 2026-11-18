use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    /*
    [in, optional]      LPCSTR                lpApplicationName,
    [in, out, optional] LPSTR                 lpCommandLine,
    */

    let appname_ptr = api_try!(emu, call.arg(emu, 0));
    let cmdline_ptr = api_try!(emu, call.arg(emu, 1));
    let appname = emu.maps.read_string(appname_ptr);
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!CreateProcessA  {} {}", appname, cmdline);

    call.cleanup_args(emu, 10);

    call.set_return(emu, 1);
}

pub fn CreateProcessW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let appname_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let cmdline_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let appname = if appname_ptr != 0 {
        emu.maps.read_wide_string(appname_ptr)
    } else {
        String::new()
    };
    let cmdline = if cmdline_ptr != 0 {
        emu.maps.read_wide_string(cmdline_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!CreateProcessW `{}` `{}`", appname, cmdline);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 10);
}
