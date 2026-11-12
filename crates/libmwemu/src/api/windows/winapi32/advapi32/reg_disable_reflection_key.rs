use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegDisableReflectionKey(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "advapi32!RegDisableReflectionKey h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    stdcall_ret(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
