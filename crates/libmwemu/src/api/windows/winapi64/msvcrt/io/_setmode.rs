use crate::emu;
use crate::winapi::helper;

pub fn _setmode(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx;
    let mode = emu.regs().rdx;
    let prev_mode = 0x4000u64; // _O_TEXT
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_setmode fd:{} '{}' mode:0x{:x} => 0x{:x}",
        fd,
        name,
        mode,
        prev_mode
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = prev_mode;
}
