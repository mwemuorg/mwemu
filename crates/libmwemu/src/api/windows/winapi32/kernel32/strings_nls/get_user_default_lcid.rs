use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::nls::*;

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetUserDefaultLCID");
    call.set_return(emu, LOCALE_USER_DEFAULT);
    call.cleanup_args(emu, 0);
}
