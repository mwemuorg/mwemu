use crate::emu;

pub fn VariantTimeToDosDateTime(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    if arg1 != 0 {
        emu.maps.write_word(arg1, 0); // VT_EMPTY
    }
    log_red!(emu, "oleaut32!VariantTimeToDosDateTime ptr:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
