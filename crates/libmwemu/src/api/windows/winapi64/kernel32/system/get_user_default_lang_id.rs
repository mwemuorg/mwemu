use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::windows::constants::nls::*;

pub fn GetUserDefaultLangID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetUserDefaultLangID");
    call.set_return(emu, EN_US_LOCALE as u64);
}
