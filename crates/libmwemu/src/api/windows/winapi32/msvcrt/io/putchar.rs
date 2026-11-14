use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn putchar(emu: &mut emu::Emu, api: &str) {
    let c = WinApiAbi::arg(emu, 0) as u32;
    let ch_str = format!("{}", c as u8 as char);
    log_red!(emu, "msvcrt!{} '{}'", api, ch_str);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = c as u64;
}
