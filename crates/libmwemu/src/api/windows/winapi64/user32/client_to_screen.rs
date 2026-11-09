use crate::emu;

pub fn ClientToScreen(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let pt = emu.regs().rdx;
    log_red!(emu, "user32!ClientToScreen hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
