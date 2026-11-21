use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn LCMapStringA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let p4 = api_try!(emu, call.arg(emu, 4));
    let p5 = api_try!(emu, call.arg(emu, 5));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!LCMapStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 6);
}

pub fn LCMapStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let src_ptr = api_try!(emu, call.arg(emu, 2));
    let src_sz = api_try!(emu, call.arg_u32(emu, 3));
    let dest_ptr = api_try!(emu, call.arg(emu, 4));
    let dest_sz = api_try!(emu, call.arg_u32(emu, 5));

    let s = emu.maps.read_wide_string(src_ptr);
    let char_count = if src_sz as i32 == -1 {
        s.len() as u32 + 1
    } else {
        src_sz
    };

    log_red!(
        emu,
        "kernel32!LCMapStringW `{}` dst:0x{:x} sz:{}->{}",
        s,
        dest_ptr,
        src_sz,
        dest_sz
    );

    if dest_ptr > 0 && dest_sz > 0 {
        emu.maps.write_wide_string(dest_ptr, &s);
    }

    call.cleanup_args(emu, 6);
    call.set_return(emu, char_count as u64);
}
