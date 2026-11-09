use crate::emu;

pub fn __wgetmainargs(emu: &mut emu::Emu) {
    let p_argc = emu.regs().rcx;
    let p_argv = emu.regs().rdx;
    let p_env = emu.regs().r8;
    log_red!(emu, "msvcrt!__wgetmainargs");
    if p_argc != 0 {
        emu.maps.write_qword(p_argc, 1);
    }
    if p_argv != 0 {
        emu.maps.write_qword(p_argv, 0);
    }
    if p_env != 0 {
        emu.maps.write_qword(p_env, 0);
    }
    emu.regs_mut().rax = 0;
}
