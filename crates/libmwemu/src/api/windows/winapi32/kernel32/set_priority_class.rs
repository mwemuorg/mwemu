use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetPriorityClass(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    log_red!(emu, "kernel32!SetPriorityClass p1:0x{:x}", p1);
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
