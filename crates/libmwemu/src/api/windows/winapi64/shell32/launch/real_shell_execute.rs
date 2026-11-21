use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RealShellExecuteA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let operation = api_try!(emu, call.arg(emu, 1));
    let file_ptr = api_try!(emu, call.arg(emu, 2));
    let params_ptr = api_try!(emu, call.arg(emu, 3));
    let dir = api_try!(emu, call.arg(emu, 4));
    let bShowWindow = api_try!(emu, call.arg(emu, 5));

    let file = emu.maps.read_string(file_ptr);
    let params = emu.maps.read_string(params_ptr);

    log_red!(emu, "shell32!RealShellExecuteA {} {}", file, params);

    emu.set_last_error(0);
    call.set_return(emu, 34);
}

pub fn RealShellExecuteW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "shell32!RealShellExecuteW '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
