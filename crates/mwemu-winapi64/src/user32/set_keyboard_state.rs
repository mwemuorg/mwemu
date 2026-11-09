use crate::emu;

pub fn SetKeyboardState(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    log_red!(emu, "user32!SetKeyboardState buf:0x{:x}", buf);
    if buf != 0 && emu.maps.is_mapped(buf) {
        let _ = emu.maps.read_dword(buf).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
