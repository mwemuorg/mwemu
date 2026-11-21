use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn __deregister_frame_info(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let p1 = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "libgcc!__deregister_frame_info {:x}", p1);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}
