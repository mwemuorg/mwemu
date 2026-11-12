use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _crt_atexit(emu: &mut emu::Emu, api: &str) {
    let func_ptr = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!{} func:0x{:x}", api, func_ptr);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
