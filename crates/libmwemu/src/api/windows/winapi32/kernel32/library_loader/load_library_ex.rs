use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let libname_ptr = api_try!(emu, call.arg(emu, 0));
    let libname = emu.maps.read_string(libname_ptr);

    log_red!(emu, "kernel32!LoadLibraryExA '{}'", libname);

    let return_value = load_library(emu, &libname);
    call.set_return(emu, return_value);
    call.cleanup_args(emu, 3);
}

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let libname_ptr = api_try!(emu, call.arg(emu, 0));
    let hfile = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));

    let libname = emu.maps.read_wide_string(libname_ptr);

    log_red!(emu, "LoadLibraryExW '{}'", libname);

    let return_value = load_library(emu, &libname);
    call.set_return(emu, return_value);

    /*
    if emu.regs_mut().rax == 0 {
        emu.regs_mut().rax = 1;
    }*/

    call.cleanup_args(emu, 3);
}
