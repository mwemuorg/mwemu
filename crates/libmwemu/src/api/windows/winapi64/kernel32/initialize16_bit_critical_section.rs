use crate::emu;
use crate::windows::constants;

pub fn Initialize16BitCriticalSection(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;
    let _a3 = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!Initialize16BitCriticalSection");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
