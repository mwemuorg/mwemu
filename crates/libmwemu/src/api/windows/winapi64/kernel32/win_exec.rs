use crate::emu;

pub fn WinExec(emu: &mut emu::Emu) {
    let cmdline_ptr = emu.regs().rcx;
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!WinExec  '{}'", cmdline);

    emu.regs_mut().rax = 32;
}
