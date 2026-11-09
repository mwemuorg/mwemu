use crate::emu;

pub fn CompareStringEx(emu: &mut emu::Emu) {
    let _locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let s1_ptr = emu.regs().r8;
    let s1_len = emu.regs().r9 as i64;
    let s2_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
}
