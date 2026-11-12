use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = WinApiAbi::arg(emu, 0);
    let end_ptr = WinApiAbi::arg(emu, 1);

    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
