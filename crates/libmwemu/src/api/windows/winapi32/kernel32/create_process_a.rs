use crate::emu;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    /*
    [in, optional]      LPCSTR                lpApplicationName,
    [in, out, optional] LPSTR                 lpCommandLine,
    */

    let appname_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CreateProcessA: cannot read stack") as u64;
    let cmdline_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CreateProcessA: cannot read stack2") as u64;
    let appname = emu.maps.read_string(appname_ptr);
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!CreateProcessA  {} {}", appname, cmdline);

    for _ in 0..10 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
