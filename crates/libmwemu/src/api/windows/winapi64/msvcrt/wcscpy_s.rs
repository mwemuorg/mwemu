use crate::emu;

pub fn wcscpy_s(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let _dst_size = emu.regs().rdx;
    let src = emu.regs().r8;
    let s = emu.maps.read_wide_string(src);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &s);
    }
    log_red!(emu, "msvcrt!wcscpy_s dst:0x{:x} '{}'", dst, s);
    emu.regs_mut().rax = 0;
}
