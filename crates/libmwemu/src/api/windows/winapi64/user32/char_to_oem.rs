use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CharToOemA(emu: &mut emu::Emu) {
    char_to_oem(emu, "CharToOemA", false);
}

fn char_to_oem(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!{api} src: 0x{:x} dst: 0x{:x}", src, dst);
    if src != 0 && dst != 0 {
        let s = strings::read_text(emu, src, wide);
        strings::write_string(emu, dst, &s, wide);
    }
    call.set_return(emu, 1); // TRUE
}

pub fn CharToOemW(emu: &mut emu::Emu) {
    char_to_oem(emu, "CharToOemW", true);
}
