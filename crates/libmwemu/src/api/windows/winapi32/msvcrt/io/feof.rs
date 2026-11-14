use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn feof(emu: &mut emu::Emu) {
    let fp = WinApiAbi::arg(emu, 0);
    let is_eof = helper::handler_eof(fp);
    log_red!(emu, "msvcrt!feof fp:0x{:x} => {}", fp, is_eof as u32);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = is_eof as u64;
}
