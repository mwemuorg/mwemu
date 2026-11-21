use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy;
use crate::winapi::policy::api_try;
use crate::windows::constants::nls::EN_US_LOCALE;

pub fn GetThreadUILanguage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "GetThreadUILanguage=0x{:x}", EN_US_LOCALE);
    call.set_return(emu, u64::from(EN_US_LOCALE));
}
pub fn SetThreadUILanguage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let requested = api_try!(emu, call.arg_u16(emu, 0));
    log_red!(emu, "SetThreadUILanguage 0x{:x}", requested);
    if requested != 0 && u32::from(requested) != EN_US_LOCALE {
        policy::unsupported(emu, "kernel32", "SetThreadUILanguage", emu.pc());
        return;
    }
    call.set_return(emu, u64::from(EN_US_LOCALE));
    call.cleanup_args(emu, 1);
}
