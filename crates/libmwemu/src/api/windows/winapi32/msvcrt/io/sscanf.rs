use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn sscanf(emu: &mut emu::Emu, api: &str) {
    let buf = WinApiAbi::arg(emu, 0);
    let fmt_ptr = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!{} buf:'{}' fmt:'{}'", api, s, fmt);
    emu.regs_mut().rax = 1;
}
