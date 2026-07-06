use crate::emu;

pub fn Thread32First(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let entry = emu.regs().rdx;

    log_red!(emu, "kernel32!Thread32First");

    emu.regs_mut().rax = 1;
    //emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}
