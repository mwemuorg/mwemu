use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn _lock(emu: &mut emu::Emu) {
    let lock_num = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
