use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EncodePointer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!EncodePointer ptr: 0x{:x}", ptr);

    call.cleanup_args(emu, 1);
    call.set_return(emu, ptr);
}
