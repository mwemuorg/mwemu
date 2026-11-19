use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn crt_clock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!clock => 1000");
    call.set_return(emu, 1000);
}
