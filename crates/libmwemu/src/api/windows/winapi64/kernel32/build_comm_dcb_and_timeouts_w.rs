use crate::emu;
use crate::windows::constants;

pub fn BuildCommDCBAndTimeoutsW(emu: &mut emu::Emu) {
    let _lpDef = emu.regs().rcx as u32;
    let _lpDCB = emu.regs().rdx as u32;
    let _lpCommTimeouts = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!BuildCommDCBAndTimeoutsW");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
