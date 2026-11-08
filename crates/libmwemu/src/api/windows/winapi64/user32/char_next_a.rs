use crate::emu;

pub fn CharNextA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!CharNextA '{}'", s);
    if p0 != 0 {
        let ch = emu.maps.read_byte(p0).unwrap_or(0);
        if ch != 0 {
            emu.set_last_error(0);
            emu.regs_mut().rax = p0 + 1;
        } else {
            emu.regs_mut().rax = p0;
        }
    } else {
        emu.regs_mut().rax = 0;
    }
}
