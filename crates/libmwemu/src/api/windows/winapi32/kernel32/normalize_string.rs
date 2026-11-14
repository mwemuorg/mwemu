use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn NormalizeString(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "kernel32!NormalizeString arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    stdcall_ret(emu, 5);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
