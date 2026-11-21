use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn TranslateAccelerator(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "user32!TranslateAccelerator p1:0x{:x} p2:0x{:x}",
        p1,
        p2
    );
    let src = api_try!(emu, call.arg(emu, 0));
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn TranslateAcceleratorA(emu: &mut emu::Emu) {
    translate_accelerator(emu, "TranslateAcceleratorA", false);
}

fn translate_accelerator(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
}

pub fn TranslateAcceleratorW(emu: &mut emu::Emu) {
    translate_accelerator(emu, "TranslateAcceleratorW", true);
}
