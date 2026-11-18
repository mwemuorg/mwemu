use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn EncryptFileA(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileA", false);
}

fn encrypt_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn EncryptFileW(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileW", true);
}
