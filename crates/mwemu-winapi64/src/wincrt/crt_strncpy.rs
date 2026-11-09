use crate::emu;

pub(super) fn crt_strncpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let s = emu.maps.read_string(src);
    let copy_len = s.len().min(n as usize);
    for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
        let _ = emu.maps.write_byte(dst + i as u64, *b);
    }
    for i in copy_len..n as usize {
        let _ = emu.maps.write_byte(dst + i as u64, 0);
    }
    log_red!(emu, "wincrt!strncpy 0x{:x} '{}' {}", dst, s, n);
    emu.regs_mut().rax = dst;
}
