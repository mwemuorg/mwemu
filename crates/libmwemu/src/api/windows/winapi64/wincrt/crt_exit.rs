use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_exit(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let code = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!{} code:{}", api, code);
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    call.set_return(emu, 0);
}
