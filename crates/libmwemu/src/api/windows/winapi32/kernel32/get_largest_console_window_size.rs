use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetLargestConsoleWindowSize(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetLargestConsoleWindowSize h:0x{:x} => 120x50",
        h_console
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = (50u64 << 16) | 120;
    stdcall_ret(emu, 1);
}
