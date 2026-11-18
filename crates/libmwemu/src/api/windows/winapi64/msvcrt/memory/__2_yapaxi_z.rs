use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn __2_YAPAXI_Z(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let size = api_try!(emu, call.arg(emu, 0));
    let base = allocate_memory(emu, size).unwrap_or(0);
    log_red!(emu, "msvcrt!operator_new size:{} => 0x{:x}", size, base);
    emu.set_last_error(0);
    call.set_return(emu, base);
}
