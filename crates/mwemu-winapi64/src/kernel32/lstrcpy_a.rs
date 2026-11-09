use crate::emu;

pub fn lstrcpyA(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;

    let s = emu.maps.read_string(src);

    log_red!(emu, "kernel32!lstrcpyA '{}' dst:0x{:x}", s, dst);

    if dst != 0 && !s.is_empty() {
        emu.maps.write_string(dst, &s);
        emu.regs_mut().rax = dst;
    } else {
        emu.regs_mut().rax = 0;
    }
}
