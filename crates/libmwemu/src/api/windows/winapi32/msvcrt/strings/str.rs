use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn str(emu: &mut emu::Emu) {
    let s_ptr = WinApiAbi::arg(emu, 0);
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);

    emu.regs_mut().rax = s_ptr;
}
