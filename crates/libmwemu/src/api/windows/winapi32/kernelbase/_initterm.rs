use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ptr1 = api_try!(emu, call.arg_u32(emu, 0));
    let ptr2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "kernelbase!_initterm 0x{:x} 0x{:x}", ptr1, ptr2);
    call.set_return(emu, 0);
}
