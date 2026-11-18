use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _initterm(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let start_ptr = api_try!(emu, call.arg(emu, 0));
    let end_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);
    let mut ptr = start_ptr;
    while ptr < end_ptr {
        let func = emu.maps.read_qword(ptr).unwrap_or(0);
        if func != 0 {
            log_red!(emu, "  _initterm: init func 0x{:x}", func);
        }
        ptr += 8;
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
