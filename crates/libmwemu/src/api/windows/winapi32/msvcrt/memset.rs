use crate::emu;

pub fn memset(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let val = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u8;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    if dst != 0 && n > 0 {
        for i in 0..n {
            emu.maps.write_byte(dst + i as u64, val);
        }
    }
    log_red!(emu, "msvcrt!memset dst:0x{:x} val:0x{:x} n:{}", dst, val, n);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = dst;
}
