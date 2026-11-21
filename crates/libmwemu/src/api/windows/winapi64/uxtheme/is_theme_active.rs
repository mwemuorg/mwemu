use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn IsThemeActive(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "** {} uxtheme!IsThemeActive", emu.pos);
    call.set_return(emu, 1);
}
