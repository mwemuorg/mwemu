use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetAltTabInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!GetAltTabInfo h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetAltTabInfoA(emu: &mut emu::Emu) {
    get_alt_tab_info(emu, "GetAltTabInfoA", false);
}

fn get_alt_tab_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetAltTabInfoW(emu: &mut emu::Emu) {
    get_alt_tab_info(emu, "GetAltTabInfoW", true);
}
