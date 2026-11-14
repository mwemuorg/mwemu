use crate::emu;

pub fn IsZoomed(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!IsZoomed hwnd:0x{:x}", hwnd);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
