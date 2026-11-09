use crate::emu;

pub fn VariantChangeType(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let _lcid = emu.regs().r8;
    let vt_new = emu.regs().r9;
    if dst != 0 {
        emu.maps.write_word(dst, vt_new as u16);
    }
    log_red!(
        emu,
        "oleaut32!VariantChangeType vt:{} dst:0x{:x}",
        vt_new,
        dst
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
