use crate::emu;

pub fn CharToOemW(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    log_red!(emu, "user32!CharToOemW src: 0x{:x} dst: 0x{:x}", src, dst);
    if src != 0 && dst != 0 {
        let s = emu.maps.read_wide_string(src);
        emu.maps.write_string(dst, &s);
    }
    emu.regs_mut().rax = 1; // TRUE
}
