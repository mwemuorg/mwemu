use crate::emu;

pub fn memset(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let val = emu.regs().rdx as u8;
    let n = emu.regs().r8 as usize;
    if dst != 0 && n > 0 {
        for i in 0..n {
            emu.maps.write_byte(dst + i as u64, val);
        }
    }
    log_red!(emu, "msvcrt!memset dst:0x{:x} val:0x{:x} n:{}", dst, val, n);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = dst;
}
