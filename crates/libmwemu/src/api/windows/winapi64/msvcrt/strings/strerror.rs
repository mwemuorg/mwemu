use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn strerror(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let errnum = api_try!(emu, call.arg_u32(emu, 0));
    let msg = match errnum {
        0 => "No error",
        2 => "No such file or directory",
        13 => "Permission denied",
        22 => "Invalid argument",
        _ => "Unknown error",
    };
    let addr = allocate_memory(emu, msg.len() as u64 + 1).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_string(addr, msg);
    }
    log_red!(emu, "msvcrt!strerror {} => '{}'", errnum, msg);
    emu.set_last_error(0);
    call.set_return(emu, addr);
}
