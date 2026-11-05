use crate::emu;

pub fn GetConsoleFontInfo(emu: &mut emu::Emu) {
    let h_console = emu.regs().rcx;
    let max_window = emu.regs().rdx as u32;
    let num_fonts = emu.regs().r8 as u32;
    let info_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetConsoleFontInfo h:0x{:x} max:{} count:{} ptr:0x{:x}",
        h_console,
        max_window,
        num_fonts,
        info_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
