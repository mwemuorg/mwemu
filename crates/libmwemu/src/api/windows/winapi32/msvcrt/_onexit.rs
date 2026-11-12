use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn _onexit(emu: &mut emu::Emu) {
    let fptr = WinApiAbi::arg(emu, 0);

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = fptr;
}
