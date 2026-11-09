use crate::emu;

pub fn VariantCopy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    if dst != 0 && src != 0 {
        let vt = emu.maps.read_word(src).unwrap_or(0);
        let val = emu.maps.read_qword(src + 8).unwrap_or(0);
        emu.maps.write_word(dst, vt);
        emu.maps.write_qword(dst + 8, val);
    }
    log_red!(emu, "oleaut32!VariantCopy dst:0x{:x} src:0x{:x}", dst, src);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
