use crate::emu;

pub fn __wgetmainargs(emu: &mut emu::Emu) {
    let p_argc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_argv = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_env = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(emu, "msvcrt!__wgetmainargs");
    if p_argc != 0 {
        emu.maps.write_dword(p_argc, 1);
    }
    if p_argv != 0 {
        emu.maps.write_dword(p_argv, 0);
    }
    if p_env != 0 {
        emu.maps.write_dword(p_env, 0);
    }
    emu.regs_mut().rax = 0;
}
