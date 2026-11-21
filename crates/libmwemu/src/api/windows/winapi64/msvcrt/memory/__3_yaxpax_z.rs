use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::msvcrt::release;

pub fn __3_YAXPAX_Z(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "msvcrt!operator_delete ptr:0x{:x}", ptr);
    release(emu, ptr);
}
