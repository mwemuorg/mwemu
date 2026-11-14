use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CompareStringA(emu: &mut emu::Emu) {
    compare_string(emu, "CompareStringA", false);
}

pub fn CompareStringW(emu: &mut emu::Emu) {
    compare_string(emu, "CompareStringW", true);
}

fn compare_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _locale = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let str1_ptr = emu.regs().r8;
    let _len1 = emu.regs().r9 as i32;
    let str2_ptr = WinApiAbi::arg(emu, 4);
    let s1 = if str1_ptr != 0 {
        strings::read_text(emu, str1_ptr, wide)
    } else {
        String::new()
    };
    let s2 = if str2_ptr != 0 {
        strings::read_text(emu, str2_ptr, wide)
    } else {
        String::new()
    };
    if wide {
        let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        };
        log_red!(emu, "kernelbase!{api} '{}' vs '{}' -> {}", s1, s2, result);
        emu.set_last_error(0);
        emu.regs_mut().rax = result;
    } else {
        let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
            std::cmp::Ordering::Less => 1u64,
            std::cmp::Ordering::Equal => 2,
            std::cmp::Ordering::Greater => 3,
        };
        log_red!(emu, "kernelbase!{api} '{}' vs '{}' => {}", s1, s2, result);
        emu.set_last_error(0);
        emu.regs_mut().rax = result;
    }
}
