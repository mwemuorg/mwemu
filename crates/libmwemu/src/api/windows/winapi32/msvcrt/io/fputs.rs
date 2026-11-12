use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn fputs(emu: &mut emu::Emu) {
    let s_ptr = WinApiAbi::arg(emu, 0);
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!fputs '{}'", s);
    emu.regs_mut().rax = 0;
}
