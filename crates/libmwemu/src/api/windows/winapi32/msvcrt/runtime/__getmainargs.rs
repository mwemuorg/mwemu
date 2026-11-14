use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn __getmainargs(emu: &mut emu::Emu) {
    let p_argc = WinApiAbi::arg(emu, 0);
    let p_argv = WinApiAbi::arg(emu, 1);
    let p_env = WinApiAbi::arg(emu, 2);
    log_red!(emu, "msvcrt!__getmainargs");
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
