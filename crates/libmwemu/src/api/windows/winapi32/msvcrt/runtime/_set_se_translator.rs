use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _set_se_translator(emu: &mut emu::Emu, api: &str) {
    let handler = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!{} handler:0x{:x}", api, handler);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
