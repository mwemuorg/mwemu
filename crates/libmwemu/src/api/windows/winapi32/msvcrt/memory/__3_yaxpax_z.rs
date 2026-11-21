use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi32::msvcrt::release;

pub fn __3_YAXPAX_Z(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    // operator delete(void*)
    let ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
    if ptr != 0 {
        if emu.cfg.memory_guard {
            emu.kernel_free(ptr, "operator_delete");
        } else {
            release(emu, ptr);
        }
    }
}
