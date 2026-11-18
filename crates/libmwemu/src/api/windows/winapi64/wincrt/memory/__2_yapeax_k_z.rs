use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn __2_YAPEAX_K_Z(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!{}", api);
    helper::advance_tick(emu, 1);
    call.set_return(emu, 0);
}
