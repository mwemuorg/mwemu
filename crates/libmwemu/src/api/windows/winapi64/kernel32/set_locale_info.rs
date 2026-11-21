use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn SetLocaleInfoA(emu: &mut emu::Emu) {
    set_locale_info(emu, "SetLocaleInfoA");
}

pub fn SetLocaleInfoW(emu: &mut emu::Emu) {
    set_locale_info(emu, "SetLocaleInfoW");
}

fn set_locale_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!{} => TRUE", api);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
