use crate::emu;

pub fn uaw_wcsicmp(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;

    let s1 = emu.maps.read_wide_string(s1_ptr);
    let s2 = emu.maps.read_wide_string(s2_ptr);

    let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
        std::cmp::Ordering::Less => -1i64 as u64,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    log_red!(
        emu,
        "kernel32!uaw_wcsicmp '{}' vs '{}' => {}",
        s1,
        s2,
        result as i64
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
