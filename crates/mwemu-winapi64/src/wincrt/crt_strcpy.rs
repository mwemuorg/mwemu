use crate::emu;

pub(super) fn crt_strcpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let s = emu.maps.read_string(src);
    emu.maps.write_string(dst, &s);
    log_red!(emu, "wincrt!strcpy 0x{:x} '{}'", dst, s);
    emu.regs_mut().rax = dst;
}
