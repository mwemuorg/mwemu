use crate::emu;

pub fn IdnToUnicode(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let p_src = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let p_dest = emu.regs().r9;
    let dest_len = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;

    let src = emu.maps.read_wide_string(p_src);
    log_red!(
        emu,
        "kernel32!IdnToUnicode flags:0x{:x} src:'{}'",
        flags,
        src
    );

    if p_dest != 0 && dest_len > 0 {
        emu.maps.write_wide_string(p_dest, &src);
    }
    emu.regs_mut().rax = src.len() as u64 + 1;
}
