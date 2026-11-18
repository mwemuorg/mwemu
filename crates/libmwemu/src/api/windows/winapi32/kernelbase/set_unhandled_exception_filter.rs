use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr1 = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernelbase!SetUnhandledExceptionFilter 0x{:x}", ptr1);

    emu.set_uef(ptr1 as u64);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 0);
}
