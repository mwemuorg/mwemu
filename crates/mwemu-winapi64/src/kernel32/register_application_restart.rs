use crate::emu;

pub fn RegisterApplicationRestart(emu: &mut emu::Emu) {
    let cmd_line = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let cmd = emu.maps.read_wide_string(cmd_line);
    log_red!(
        emu,
        "kernel32!RegisterApplicationRestart cmd:'{}' flags:0x{:x}",
        cmd,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
