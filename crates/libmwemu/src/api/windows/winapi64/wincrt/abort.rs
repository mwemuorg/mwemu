use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn abort(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!abort");
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    call.set_return(emu, 0);
}
