use crate::emu;

pub fn VarCyFromDate(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    log_red!(emu, "oleaut32!VarCyFromDate src:0x{:x}", src);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
