use crate::{emu, windows::constants};

pub fn WaitForMultipleObjects(emu: &mut emu::Emu) {
    let count = emu.regs().rcx;
    let handles_ptr = emu.regs().rdx;
    let wait_all = emu.regs().r8;
    let millis = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!WaitForMultipleObjects count: {} handles: 0x{:x} waitAll: {} millis: {}",
        count,
        handles_ptr,
        wait_all,
        millis
    );

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.regs_mut().rax = constants::WAIT_OBJECT_0;
}
