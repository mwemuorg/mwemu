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

pub fn CharLowerW(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    if ptr < 0x10000 {
        let ch = (ptr & 0xFFFF) as u16;
        let lower = if (0x41..=0x5A).contains(&ch) {
            ch + 0x20
        } else {
            ch
        };
        log_red!(emu, "user32!CharLowerW char: 0x{:x}", ch);
        emu.regs_mut().rax = lower as u64;
    } else {
        let s = emu.maps.read_wide_string(ptr);
        emu.maps.write_wide_string(ptr, &s.to_lowercase());
        log_red!(emu, "user32!CharLowerW '{}'", s);
        emu.regs_mut().rax = ptr;
    }
}
