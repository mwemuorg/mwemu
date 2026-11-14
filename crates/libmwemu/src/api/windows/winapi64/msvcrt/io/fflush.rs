use crate::emu;
use crate::winapi::helper;

pub fn fflush(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let filename = helper::handler_get_uri(file);
    log_red!(emu, "msvcrt!fflush `{}`", filename);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
