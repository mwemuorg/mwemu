use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn getchar(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    log_red!(emu, "wincrt!getchar");
    helper::advance_tick(emu, 1);
    call.set_return(emu, 0x0A);
}
