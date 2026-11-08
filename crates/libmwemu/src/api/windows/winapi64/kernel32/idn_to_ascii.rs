use crate::emu;

pub fn IdnToAscii(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let p_src = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let p_dest = emu.regs().r9;
    let dest_len = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;

    let src = emu.maps.read_wide_string(p_src);
    log_red!(
        emu,
        "kernel32!IdnToAscii flags:0x{:x} src:'{}' dest:0x{:x}",
        flags,
        src,
        p_dest
    );

    if p_dest != 0 && dest_len > 0 {
        emu.maps.write_wide_string(p_dest, &src);
    }
    emu.regs_mut().rax = src.len() as u64 + 1;
}
