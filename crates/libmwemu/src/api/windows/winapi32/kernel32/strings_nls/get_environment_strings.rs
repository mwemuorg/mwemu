use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetEnvironmentStrings");
    let ptr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    call.set_return(emu, ptr);
}

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "kernel32!GetEnvironmentStringsA arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetEnvironmentStringsW");
    let addr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps
        .write_wide_string(addr, "PATH=c:\\Windows\\System32");
    call.set_return(emu, addr);
}
