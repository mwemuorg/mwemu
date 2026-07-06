use crate::emu;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    let appname_ptr = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    let appname = emu.maps.read_string(appname_ptr);
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!CreateProcessA  {} {}", appname, cmdline);

    emu.regs_mut().rax = 1;
}
