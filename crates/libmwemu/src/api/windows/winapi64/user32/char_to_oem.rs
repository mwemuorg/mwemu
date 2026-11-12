use crate::emu;
use crate::winapi::common::strings;

pub fn CharToOemA(emu: &mut emu::Emu) {
    char_to_oem(emu, "CharToOemA", false);
}

fn char_to_oem(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    log_red!(emu, "user32!{api} src: 0x{:x} dst: 0x{:x}", src, dst);
    if src != 0 && dst != 0 {
        let s = strings::read_text(emu, src, wide);
        strings::write_string(emu, dst, &s, wide);
    }
    emu.regs_mut().rax = 1; // TRUE
}

pub fn CharToOemW(emu: &mut emu::Emu) {
    char_to_oem(emu, "CharToOemW", true);
}
