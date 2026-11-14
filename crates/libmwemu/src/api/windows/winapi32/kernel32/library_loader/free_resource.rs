use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;

pub fn FreeResource(emu: &mut emu::Emu) {
    let hResData = arg(emu, 0) as u64;

    emu.stack_pop32(false);

    log_red!(emu, "** {} kernel32!FreeResource {:x}", emu.pos, hResData);
    helper::handler_close(hResData);

    emu.regs_mut().rax = 1;
}
