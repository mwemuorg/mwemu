use crate::emu;

pub fn StringCchLengthW(emu: &mut emu::Emu) {
    let psz = emu.regs().rcx;
    let cch_max = emu.regs().rdx;
    let pcch_length = emu.regs().r8;

    let s = emu.maps.read_wide_string(psz);
    let slen = s.len();

    log_red!(
        emu,
        "kernel32!StringCchLengthW psz:0x{:x} cchMax:{} len:{}",
        psz,
        cch_max,
        slen
    );

    if pcch_length != 0 {
        emu.maps.write_qword(pcch_length, slen as u64);
    }
    emu.regs_mut().rax = 0;
}
