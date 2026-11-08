use crate::emu;

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
