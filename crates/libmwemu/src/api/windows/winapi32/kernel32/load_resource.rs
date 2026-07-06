use crate::emu;

pub fn LoadResource(emu: &mut emu::Emu) {
    let hModule = emu.regs().rcx;
    let hResInfo = emu.regs().rdx;

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
