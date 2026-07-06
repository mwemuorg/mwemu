use crate::emu;
use crate::winapi::helper;

pub fn FreeResource(emu: &mut emu::Emu) {
    let hResData = emu.regs().rcx;

    emu.stack_pop32(false);

    log_red!(emu, "** {} kernel32!FreeResource {:x}", emu.pos, hResData);
    helper::handler_close(hResData);

    emu.regs_mut().rax = 1;
}
