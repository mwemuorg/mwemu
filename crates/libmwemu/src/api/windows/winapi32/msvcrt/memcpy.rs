use crate::emu;

pub fn memcpy(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    if dst != 0 && src != 0 && n > 0 {
        let mut buf = vec![0u8; n];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
        }
        for (i, b) in buf.iter().enumerate() {
            emu.maps.write_byte(dst + i as u64, *b);
        }
    }
    log_red!(emu, "msvcrt!memcpy dst:0x{:x} src:0x{:x} n:{}", dst, src, n);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = dst;
}
