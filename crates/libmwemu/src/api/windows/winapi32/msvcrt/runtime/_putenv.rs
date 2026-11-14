use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _putenv(emu: &mut emu::Emu, api: &str) {
    let ptr = WinApiAbi::arg(emu, 0);
    let s = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, s);
    emu.regs_mut().rax = 0;
}
