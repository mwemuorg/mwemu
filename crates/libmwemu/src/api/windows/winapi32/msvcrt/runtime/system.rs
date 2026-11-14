use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn system(emu: &mut emu::Emu) {
    let cmd_ptr = WinApiAbi::arg(emu, 0);
    let cmd = emu.maps.read_string(cmd_ptr);
    log_red!(emu, "msvcrt!system '{}'", cmd);
    emu.regs_mut().rax = 0;
}
