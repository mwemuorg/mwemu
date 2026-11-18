use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_lock_noop(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let lock_num = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!{} lock:{}", api, lock_num);
    emu.set_last_error(0);
}
