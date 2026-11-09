use crate::emu;

pub fn CharLowerA(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    if ptr < 0x10000 {
        // Single character in low word
        let ch = (ptr & 0xFF) as u8;
        let lower = (ch as char).to_ascii_lowercase() as u64;
        log_red!(
            emu,
            "user32!CharLowerA char: '{}' -> '{}'",
            ch as char,
            lower as u8 as char
        );
        emu.regs_mut().rax = lower;
    } else {
        // Pointer to string — lowercase in-place
        let s = emu.maps.read_string(ptr);
        log_red!(emu, "user32!CharLowerA str: '{}'", s);
        let lower = s.to_ascii_lowercase();
        emu.maps.write_string(ptr, &lower);
        emu.regs_mut().rax = ptr;
    }
}
