use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _set_new_mode(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    // int _set_new_mode(int newmode): selects whether malloc failures call the
    // new-handler. Returns the previous mode. We keep no CRT state, so report the
    // default (0). Pure init-time bookkeeping — a safe no-op for emulation.
    let newmode = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!_set_new_mode newmode: 0x{:x}", newmode);
    call.set_return(emu, 0);
}
