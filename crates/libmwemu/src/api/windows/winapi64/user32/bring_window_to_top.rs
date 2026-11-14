use crate::emu;

pub fn BringWindowToTop(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "user32!BringWindowToTop");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
