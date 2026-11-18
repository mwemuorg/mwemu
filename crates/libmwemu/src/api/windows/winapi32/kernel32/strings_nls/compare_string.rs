use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CompareStringA(emu: &mut emu::Emu) {
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
        "kernel32!CompareStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 6);
    call.set_return(emu, 2);
}

pub fn CompareStringW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let str1 = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let len1 = api_try!(emu, call.arg_i32(emu, 3));
    let str2 = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let len2 = api_try!(emu, call.arg_i32(emu, 5));
    let read = |emu: &emu::Emu, ptr: u64, len: i32| -> String {
        if ptr == 0 || len == 0 {
            String::new()
        } else if len < 0 {
            emu.maps.read_wide_string(ptr)
        } else {
            emu.maps.read_wide_string_n(ptr, len as usize)
        }
    };
    let s1 = read(emu, str1, len1);
    let s2 = read(emu, str2, len2);
    let ord = if flags & 1 != 0 {
        s1.to_lowercase().cmp(&s2.to_lowercase()) // NORM_IGNORECASE
    } else {
        s1.cmp(&s2)
    };
    let result = match ord {
        std::cmp::Ordering::Less => 1,    // CSTR_LESS_THAN
        std::cmp::Ordering::Equal => 2,   // CSTR_EQUAL
        std::cmp::Ordering::Greater => 3, // CSTR_GREATER_THAN
    };
    log_red!(emu, "kernel32!CompareStringW '{}' '{}' ={}", s1, s2, result);
    call.set_return(emu, result);
    call.cleanup_args(emu, 6);
}
