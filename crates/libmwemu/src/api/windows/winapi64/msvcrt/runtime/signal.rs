use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn signal(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let signum = api_try!(emu, call.arg_u32(emu, 0));
    let handler = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "msvcrt!signal sig:{} handler:0x{:x}", signum, handler);
    let slot = allocate_memory(emu, 16).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, signum);
        emu.maps.write_qword(slot + 8, handler);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0); // SIG_DFL (previous handler)
}
