use crate::emu;
use crate::winapi::helper;

pub fn GetLargestConsoleWindowSize(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetLargestConsoleWindowSize h:0x{:x} => 120x50",
        h_console
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = (50u64 << 16) | 120;
}
