use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let p4 = api_try!(emu, call.arg(emu, 4));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!RegOpenKeyExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 5);
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg_u32(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let options = api_try!(emu, call.arg_u32(emu, 2));
    let sam = api_try!(emu, call.arg_u32(emu, 3));
    let result = api_try!(emu, call.arg_u32(emu, 4));

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "kernel32!RegOpenKeyExW {}", subkey);

    call.cleanup_args(emu, 5);

    call.set_return(emu, 1);
}
