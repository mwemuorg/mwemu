use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::heap::owned_size;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::ERROR_INVALID_HANDLE;

pub fn GlobalHandle(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "heap handle for 0x{:x}", ptr);
    let handle = if owned_size(emu, 0, ptr).is_some() {
        ptr
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        0
    };
    call.set_return(emu, handle);
    call.cleanup_args(emu, 1);
}
pub use GlobalHandle as LocalHandle;
