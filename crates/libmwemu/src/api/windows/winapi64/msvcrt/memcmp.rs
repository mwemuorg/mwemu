use crate::emu;

pub fn memcmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let mut result: i32 = 0;
    for i in 0..n {
        let b1 = emu.maps.read_byte(s1 + i as u64).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i as u64).unwrap_or(0);
        if b1 != b2 {
            result = if b1 < b2 { -1 } else { 1 };
            break;
        }
    }
    log_red!(
        emu,
        "msvcrt!memcmp 0x{:x} 0x{:x} n:{} => {}",
        s1,
        s2,
        n,
        result
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u32 as u64;
}
