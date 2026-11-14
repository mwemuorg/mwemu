use crate::emu;

pub(super) fn crt_memcmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8;
    let mut res: i64 = 0;
    for i in 0..n {
        let b1 = emu.maps.read_byte(s1 + i).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i).unwrap_or(0);
        if b1 != b2 {
            res = (b1 as i64) - (b2 as i64);
            break;
        }
    }
    emu.regs_mut().rax = res as u64;
}
