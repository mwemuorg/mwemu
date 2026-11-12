use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CompareStringOrdinal(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let len1 = emu.regs().rdx as i64;
    let s2 = emu.regs().r8;
    let _len2 = emu.regs().r9 as i64;
    let ignore_case = WinApiAbi::arg(emu, 4);
    let str1 = emu.maps.read_wide_string(s1);
    let str2 = emu.maps.read_wide_string(s2);
    let r = if ignore_case != 0 {
        str1.to_lowercase().cmp(&str2.to_lowercase()) as i32
    } else {
        str1.cmp(&str2) as i32
    };
    let result = match r.cmp(&0) {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    };
    log_red!(
        emu,
        "kernel32!CompareStringOrdinal '{}' '{}' ic:{} => {}",
        str1,
        str2,
        ignore_case,
        result
    );
    emu.regs_mut().rax = result;
}
