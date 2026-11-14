use crate::emu;

pub fn wcsncat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let dst_s = emu.maps.read_wide_string(dst);
    let src_s = emu.maps.read_wide_string(src);
    let copy_len = src_s.len().min(n);
    let offset = dst + (dst_s.len() * 2) as u64;
    for (i, ch) in src_s.chars().take(copy_len).enumerate() {
        emu.maps.write_word(offset + (i * 2) as u64, ch as u16);
    }
    emu.maps.write_word(offset + (copy_len * 2) as u64, 0);
    log_red!(emu, "msvcrt!wcsncat 0x{:x} '{}' {}", dst, src_s, n);
    emu.regs_mut().rax = dst;
}
