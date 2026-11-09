use crate::emu;

pub fn DispGetIDsOfNames(emu: &mut emu::Emu) {
    let punk = emu.regs().rcx;
    log_red!(emu, "oleaut32!DispGetIDsOfNames 0x{:x}", punk);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
