use crate::emu;

pub fn Module32NextW(emu: &mut emu::Emu) {
    let snapshot = emu.regs().rcx;
    let module_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Module32NextW snapshot:0x{:x} entry:0x{:x}",
        snapshot,
        module_entry
    );

    // No more modules — return FALSE so the caller stops iterating.
    emu.regs_mut().rax = 0;
}
