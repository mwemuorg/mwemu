use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn __2_YAPAXI_Z(emu: &mut emu::Emu, addr: u32) {
    let call = WinApi(Cdecl);
    // operator new(unsigned int)
    let size = api_try!(emu, call.arg(emu, 0));

    let addr = if size > 0 {
        allocate_memory(emu, size).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "msvcrt!operator_new sz:{} => 0x{:x}", size, addr);
    emu.set_last_error(0);
    call.set_return(emu, addr);
}
