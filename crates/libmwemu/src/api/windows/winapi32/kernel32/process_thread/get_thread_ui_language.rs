use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::nls::*;

pub fn GetThreadUILanguage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetThreadUILanguage (0x0409 en_US)");

    call.set_return(emu, EN_US_LOCALE as u64);
}
