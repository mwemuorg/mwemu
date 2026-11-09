use crate::emu;

pub fn strcpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let s = emu.maps.read_string(src);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "msvcrt!strcpy dst:0x{:x} '{}'", dst, s);
    emu.regs_mut().rax = dst;
}
