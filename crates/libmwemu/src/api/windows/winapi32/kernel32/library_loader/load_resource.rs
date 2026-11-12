use crate::emu;
use crate::winapi::winapi32::arg;

pub fn LoadResource(emu: &mut emu::Emu) {
    let hModule = arg(emu, 0) as u64;
    let hResInfo = arg(emu, 1) as u64;

    log_red!(
        emu,
        "** {} kernel32!LoadResource {:x} {:x}",
        emu.pos,
        hModule,
        hResInfo
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = hResInfo;
}
