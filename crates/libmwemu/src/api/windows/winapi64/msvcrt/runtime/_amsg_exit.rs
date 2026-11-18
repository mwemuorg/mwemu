use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _amsg_exit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, code);
    }
    emu.stop();
}
