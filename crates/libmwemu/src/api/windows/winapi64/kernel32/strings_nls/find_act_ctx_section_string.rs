use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindActCtxSectionStringA(emu: &mut Emu) {
    invoke(emu, false, "FindActCtxSectionStringA");
}

pub fn FindActCtxSectionStringW(emu: &mut Emu) {
    invoke(emu, true, "FindActCtxSectionStringW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let argument_2 = api_try!(emu, call.arg(emu, 2));
    let key = strings::read_text(emu, api_try!(emu, call.arg(emu, 3)), wide);
    log_red!(
        emu,
        "kernel32!{} flags:0x{:x} section:{} key:'{}'",
        api,
        argument_0,
        argument_2,
        key
    );
    // No activation-context sections are registered in this profile.
    emu.set_last_error(ERROR_SXS_KEY_NOT_FOUND);
    call.set_return(emu, 0);
}
