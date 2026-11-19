use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetThemeAppProperties(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "** {} uxtheme!GetThemeAppProperties", emu.pos);
    call.set_return(emu, 1);
}
