use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn getenv(emu: &mut emu::Emu, api: &str) {
    let ptr = WinApiAbi::arg(emu, 0);
    let name = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
    emu.regs_mut().rax = 0;
}
