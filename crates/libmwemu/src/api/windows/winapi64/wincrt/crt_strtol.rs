use crate::emu;

pub(super) fn crt_strtol(emu: &mut emu::Emu) {
    let nptr = emu.regs().rcx;
    let endptr = emu.regs().rdx;
    let base = emu.regs().r8 as u32;
    let s = emu.maps.read_string(nptr);
    let trimmed = s.trim();
    let val = i64::from_str_radix(
        trimmed.trim_start_matches("0x").trim_start_matches("0X"),
        if base == 0 {
            if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
                16
            } else {
                10
            }
        } else {
            base
        },
    )
    .unwrap_or(0);
    if endptr != 0 {
        emu.maps.write_qword(endptr, nptr + s.len() as u64);
    }
    log_red!(emu, "wincrt!strtol '{}' base:{} => {}", s, base, val);
    emu.regs_mut().rax = val as u64;
}
