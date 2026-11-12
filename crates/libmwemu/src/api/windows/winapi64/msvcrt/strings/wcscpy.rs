use crate::emu;

pub fn wcscpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let s = emu.maps.read_wide_string(src);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &s);
    }
    log_red!(emu, "msvcrt!wcscpy dst:0x{:x} '{}'", dst, s);
    emu.regs_mut().rax = dst;
}
