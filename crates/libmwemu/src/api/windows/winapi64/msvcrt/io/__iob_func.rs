use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn __iob_func(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    log_red!(emu, "msvcrt!__iob_func => 0x{:x}", iob_addr);
    emu.set_last_error(0);
    call.set_return(emu, iob_addr);
}
