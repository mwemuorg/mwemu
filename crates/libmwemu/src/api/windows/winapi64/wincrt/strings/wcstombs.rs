use crate::emu;

pub fn wcstombs(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let s = emu.maps.read_wide_string(src);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "wincrt!wcstombs '{}' n:{}", s, n);
    emu.regs_mut().rax = s.len() as u64;
}
