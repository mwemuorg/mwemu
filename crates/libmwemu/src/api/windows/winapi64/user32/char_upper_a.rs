use crate::emu;

pub fn CharUpperA(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    if ptr < 0x10000 {
        let ch = (ptr & 0xFF) as u8;
        let upper = (ch as char).to_ascii_uppercase() as u64;
        log_red!(
            emu,
            "user32!CharUpperA char: '{}' -> '{}'",
            ch as char,
            upper as u8 as char
        );
        emu.regs_mut().rax = upper;
    } else {
        let s = emu.maps.read_string(ptr);
        log_red!(emu, "user32!CharUpperA str: '{}'", s);
        let upper = s.to_ascii_uppercase();
        emu.maps.write_string(ptr, &upper);
        emu.regs_mut().rax = ptr;
    }
}
