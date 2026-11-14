use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WinExec(emu: &mut emu::Emu) {
    let cmdline_ptr = WinApiAbi::arg(emu, 0);
    let cmdline = emu.maps.read_string(cmdline_ptr);

    //emu.spawn_console();

    log_red!(emu, "WinExec  '{}'", cmdline);

    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
