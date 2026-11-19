use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn crt_xcpt_filter(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!{}", api);
    call.set_return(emu, 0);
}
