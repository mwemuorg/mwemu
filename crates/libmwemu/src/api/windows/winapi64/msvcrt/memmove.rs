use crate::emu;

pub fn memmove(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    if dst != 0 && src != 0 && n > 0 {
        let mut buf = vec![0u8; n];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
        }
        for (i, b) in buf.iter().enumerate() {
            emu.maps.write_byte(dst + i as u64, *b);
        }
    }
    log_red!(
        emu,
        "msvcrt!memmove dst:0x{:x} src:0x{:x} n:{}",
        dst,
        src,
        n
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = dst;
}
