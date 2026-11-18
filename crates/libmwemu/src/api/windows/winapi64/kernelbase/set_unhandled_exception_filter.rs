use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr1 = api_try!(emu, call.arg(emu, 0));

    log::trace!(
        "{}** {} kernelbase!SetUnhandledExceptionFilter 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        ptr1,
        emu.colors.nc
    );

    emu.set_uef(ptr1);
    call.set_return(emu, 0);
}
