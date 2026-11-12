use crate::emu;

pub fn mbstowcs(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let s = emu.maps.read_string(src);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &s);
    }
    log_red!(emu, "wincrt!mbstowcs '{}' n:{}", s, n);
    emu.regs_mut().rax = s.len() as u64;
}
