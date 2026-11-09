use crate::emu;

pub(super) fn crt_wcscpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let ws = emu.maps.read_wide_string(src);
    emu.maps.write_wide_string(dst, &ws);
    log_red!(emu, "wincrt!wcscpy 0x{:x} '{}'", dst, ws);
    emu.regs_mut().rax = dst;
}
