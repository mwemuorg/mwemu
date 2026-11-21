use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSIPGetCaps(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_subject_info = api_try!(emu, call.arg(emu, 0));
    let p_caps = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptSIPGetCaps p_subject_info:0x{:x} p_caps:0x{:x}",
        p_subject_info,
        p_caps
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
