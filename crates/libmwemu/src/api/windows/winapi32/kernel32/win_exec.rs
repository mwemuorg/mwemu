use crate::emu;

pub fn WinExec(emu: &mut emu::Emu) {
    let cmdline_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the cmdline parameter of WinExec") as u64;
    let cmdline = emu.maps.read_string(cmdline_ptr);

    //emu.spawn_console();

    log_red!(emu, "WinExec  '{}'", cmdline);

    emu.regs_mut().rax = 0;
    emu.stack_pop32(false);
}
