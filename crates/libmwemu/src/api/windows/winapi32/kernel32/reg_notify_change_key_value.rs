use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RegNotifyChangeKeyValue(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    let p3 = arg(emu, 3) as u64;
    let p4 = arg(emu, 4) as u64;
    log_red!(
        emu,
        "kernel32!RegNotifyChangeKeyValue p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
