use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetStdHandle(emu: &mut emu::Emu) {
    let nstd = arg(emu, 0) as u64 as u32;
    let handle = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!SetStdHandle nstd:0x{:x} handle:0x{:x}",
        nstd,
        handle
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
