use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CompareStringEx(emu: &mut emu::Emu) {
    let _locale = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64;
    let s1_ptr = arg(emu, 2) as u64;
    let s1_len = arg(emu, 3) as u64 as i64;
    let s2_ptr = arg(emu, 4) as u64;

    let s1 = emu.maps.read_wide_string(s1_ptr);
    let s2 = emu.maps.read_wide_string(s2_ptr);

    let (cmp1, cmp2) = if flags & 1 != 0 {
        (s1.to_lowercase(), s2.to_lowercase())
    } else {
        (s1.clone(), s2.clone())
    };

    let result = match cmp1.cmp(&cmp2) {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2u64,
        std::cmp::Ordering::Greater => 3u64,
    };

    log_red!(
        emu,
        "kernel32!CompareStringEx '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 5);
}
