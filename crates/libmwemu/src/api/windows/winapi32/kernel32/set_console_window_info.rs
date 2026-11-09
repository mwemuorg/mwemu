use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetConsoleWindowInfo(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!SetConsoleWindowInfo param1:0x{:x} param2:0x{:x} param3:0x{:x}",
        param1,
        param2,
        param3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
