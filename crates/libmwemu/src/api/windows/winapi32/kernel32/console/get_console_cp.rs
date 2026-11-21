use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetConsoleCP");
    call.set_return(emu, 0x409);
    call.cleanup_args(emu, 0);
}
