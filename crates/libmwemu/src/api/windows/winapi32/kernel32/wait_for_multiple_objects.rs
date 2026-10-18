use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn WaitForMultipleObjects(emu: &mut emu::Emu) {
    let count = arg(emu, 0);
    let handles_ptr = arg(emu, 1);
    let wait_all = arg(emu, 2);
    let millis = arg(emu, 3);
    log_red!(
        emu,
        "kernel32!WaitForMultipleObjects count: {} handles: 0x{:x} waitAll: {} millis: {}",
        count,
        handles_ptr,
        wait_all,
        millis
    );
    if !emu.cfg.short_circuit_sleep && millis > 0 && millis != 0xFFFF_FFFF {
        emu.tick += millis as usize;
    }
    emu.regs_mut().rax = constants::WAIT_OBJECT_0;
    stdcall_ret(emu, 4);
}
