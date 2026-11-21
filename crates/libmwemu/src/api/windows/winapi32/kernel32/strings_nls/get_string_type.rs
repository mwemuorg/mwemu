use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetStringTypeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!GetStringTypeA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

pub fn GetStringTypeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_type = api_try!(emu, call.arg_u32(emu, 0));
    let str_ptr = api_try!(emu, call.arg(emu, 1));
    let sz = api_try!(emu, call.arg_u32(emu, 2));
    let char_type = api_try!(emu, call.arg_u32(emu, 3));

    let ustr = emu.maps.read_wide_string(str_ptr);

    log_red!(emu, "kernel32!GetStringTypeW `{}` 0x{}", ustr, sz);

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}
