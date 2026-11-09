use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    /*
    [in, optional]      LPCSTR                lpApplicationName,
    [in, out, optional] LPSTR                 lpCommandLine,
    */

    let appname_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let cmdline_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let appname = emu.maps.read_string(appname_ptr);
    let cmdline = emu.maps.read_string(cmdline_ptr);

    log_red!(emu, "kernel32!CreateProcessA  {} {}", appname, cmdline);

    for _ in 0..10 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

pub fn CreateProcessW(emu: &mut emu::Emu) {
    let appname_ptr = arg(emu, 0) as u64;
    let cmdline_ptr = arg(emu, 1) as u64;
    let appname = if appname_ptr != 0 {
        emu.maps.read_wide_string(appname_ptr)
    } else {
        String::new()
    };
    let cmdline = if cmdline_ptr != 0 {
        emu.maps.read_wide_string(cmdline_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!CreateProcessW `{}` `{}`", appname, cmdline);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 10);
}
