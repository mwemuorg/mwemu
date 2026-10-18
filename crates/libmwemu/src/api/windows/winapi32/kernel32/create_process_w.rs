use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

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
