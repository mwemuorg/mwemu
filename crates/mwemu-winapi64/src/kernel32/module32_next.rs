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
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    // No more modules after the first one
    emu.set_last_error(18); // ERROR_NO_MORE_FILES
    emu.regs_mut().rax = 0; // FALSE
}
