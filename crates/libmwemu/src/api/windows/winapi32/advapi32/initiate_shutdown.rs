use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InitiateShutdownA(emu: &mut emu::Emu) {
    initiate_shutdown(emu, "InitiateShutdownA", false);
}

fn initiate_shutdown(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

pub fn InitiateShutdownW(emu: &mut emu::Emu) {
    initiate_shutdown(emu, "InitiateShutdownW", true);
}
