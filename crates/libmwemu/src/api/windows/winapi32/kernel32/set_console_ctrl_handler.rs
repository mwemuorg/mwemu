use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetConsoleCtrlHandler(emu: &mut emu::Emu) {
    let handler_routine = arg(emu, 0) as u64;
    let add = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!SetConsoleCtrlHandler handler:0x{:x} add:{}",
        handler_routine,
        add
    );
    let val_ptr = arg(emu, 1) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 2);
}
