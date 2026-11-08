use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsThreadpoolTimerSet(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;
    let param4 = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!IsThreadpoolTimerSet handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
