use crate::emu;

pub fn PropVariantClear(emu: &mut emu::Emu) {
    let pvar = emu.regs().rcx;
    if pvar != 0 {
        emu.maps.write_word(pvar, 0); // VT_EMPTY
    }
    log_red!(emu, "ole32!PropVariantClear ptr:0x{:x}", pvar);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
