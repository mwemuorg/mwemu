use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CompareStringW(emu: &mut emu::Emu) {
    let flags = arg(emu, 1);
    let str1 = arg(emu, 2) as u64;
    let len1 = arg(emu, 3) as i32;
    let str2 = arg(emu, 4) as u64;
    let len2 = arg(emu, 5) as i32;
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
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 6);
}
