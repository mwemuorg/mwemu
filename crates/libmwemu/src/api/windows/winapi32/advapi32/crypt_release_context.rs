use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptReleaseContext(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0);
    let flags = WinApiAbi::arg(emu, 1);

    log_red!(emu, "advapi32!CryptReleaseContext");

    helper::handler_close(hndl);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
