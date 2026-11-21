use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FoldStringA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _dwMapFlags = api_try!(emu, call.arg_u32(emu, 0));
    let lpSrcStr = api_try!(emu, call.arg_u32(emu, 1));
    let _cchSrc = api_try!(emu, call.arg_u32(emu, 2));
    let lpDestStr = api_try!(emu, call.arg_u32(emu, 3));
    let _cchDest = api_try!(emu, call.arg_u32(emu, 4));

    let lpSrcStr = emu.maps.read_string(lpSrcStr as u64);
    let lpDestStr = emu.maps.read_string(lpDestStr as u64);

    log_red!(emu, "kernel32!FoldStringA {} {}", lpSrcStr, lpDestStr);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn FoldStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let p4 = api_try!(emu, call.arg(emu, 4));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!FoldStringW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 5);
}
