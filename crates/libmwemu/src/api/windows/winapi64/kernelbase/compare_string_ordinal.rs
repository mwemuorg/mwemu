use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CompareStringOrdinal(emu: &mut emu::Emu) {
    let str1_ptr = emu.regs().rcx;
    let len1 = emu.regs().rdx as i32;
    let str2_ptr = emu.regs().r8;
    let len2 = emu.regs().r9 as i32;
    let ignore_case = WinApiAbi::arg(emu, 4);
    let s1 = emu.maps.read_wide_string(str1_ptr);
    let s2 = emu.maps.read_wide_string(str2_ptr);
    let result = if ignore_case != 0 {
        match s1.to_lowercase().cmp(&s2.to_lowercase()) {
            std::cmp::Ordering::Less => 1u64,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        }
    } else {
        match s1.cmp(&s2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        }
    };
    log_red!(
        emu,
        "kernelbase!CompareStringOrdinal '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
