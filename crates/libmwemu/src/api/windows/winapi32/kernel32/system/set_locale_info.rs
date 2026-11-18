use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetLocaleInfoA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Locale = api_try!(emu, call.arg_u32(emu, 0));
    let _LCType = api_try!(emu, call.arg_u32(emu, 1));
    let _lpLCData = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!SetLocaleInfoA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn SetLocaleInfoW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!SetLocaleInfoW '{}' p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
