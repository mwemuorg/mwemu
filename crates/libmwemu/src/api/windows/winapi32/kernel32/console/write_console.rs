use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WriteConsoleA(emu: &mut emu::Emu) {
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
        "kernel32!WriteConsoleA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

pub fn WriteConsoleW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let count = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let written_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let text = emu.maps.read_wide_string_n(buf, count);
    log_red!(emu, "kernel32!WriteConsoleW `{}`", text);
    if written_ptr != 0 {
        emu.maps.write_dword(written_ptr, count as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
