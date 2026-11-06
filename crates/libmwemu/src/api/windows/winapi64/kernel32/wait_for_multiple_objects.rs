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
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFFFFFF {
        emu.tick += millis as usize;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::WAIT_OBJECT_0;
}
