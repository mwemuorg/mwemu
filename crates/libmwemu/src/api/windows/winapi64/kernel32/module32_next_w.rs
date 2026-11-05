use crate::emu;

pub fn Module32NextW(emu: &mut emu::Emu) {
    let h_snapshot = emu.regs().rcx;
    let p_entry = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Module32NextW h_snapshot:0x{:x} p_entry:0x{:x}",
        h_snapshot,
        p_entry
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
