use crate::emu;

pub fn VarCyAbs(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    log_red!(emu, "oleaut32!VarCyAbs src:0x{:x}", src);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
