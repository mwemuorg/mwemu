use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_srand(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let seed = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!srand seed:{}", seed);
    emu.set_last_error(0);
}
