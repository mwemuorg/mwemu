use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetThreadLocale(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetThreadLocale");
    call.set_return(emu, 0x409); // English (United States)
    call.cleanup_args(emu, 0);
}
