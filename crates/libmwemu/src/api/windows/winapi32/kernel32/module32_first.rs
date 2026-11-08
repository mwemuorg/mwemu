use crate::emu;

pub fn Module32First(emu: &mut emu::Emu) {
    let snap = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let entry = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!Module32First snap:0x{:x} entry:0x{:x}",
        snap,
        entry
    );
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
