use crate::emu;

pub fn Module32Next(emu: &mut emu::Emu) {
    let h_snapshot = emu.regs().rcx;
    let p_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Module32Next snap:0x{:x} entry:0x{:x}",
        h_snapshot,
        p_entry
    );

    // No more modules after the first one
    emu.set_last_error(18); // ERROR_NO_MORE_FILES
    emu.regs_mut().rax = 0; // FALSE
}
