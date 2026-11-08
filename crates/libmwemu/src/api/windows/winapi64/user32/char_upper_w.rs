use crate::emu;

pub fn CharUpperW(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    if ptr < 0x10000 {
        let ch = (ptr & 0xFFFF) as u16;
        if let Some(c) = char::from_u32(ch as u32) {
            let upper = c.to_uppercase().next().unwrap_or(c) as u64;
            log_red!(emu, "user32!CharUpperW char: '{}' -> upper", c);
            emu.regs_mut().rax = upper;
        } else {
            emu.set_last_error(0);
            emu.regs_mut().rax = ptr;
        }
    } else {
        let s = emu.maps.read_wide_string(ptr);
        log_red!(emu, "user32!CharUpperW str: '{}'", s);
        let upper: String = s
            .chars()
            .map(|c| c.to_uppercase().next().unwrap_or(c))
            .collect();
        emu.maps.write_wide_string(ptr, &upper);
        emu.regs_mut().rax = ptr;
    }
}
