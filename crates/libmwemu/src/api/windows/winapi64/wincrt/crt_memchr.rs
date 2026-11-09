use crate::emu;

pub(super) fn crt_memchr(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let c = emu.regs().rdx as u8;
    let n = emu.regs().r8;
    let mut result: u64 = 0;
    for i in 0..n {
        if emu.maps.read_byte(s + i).unwrap_or(0) == c {
            result = s + i;
            break;
        }
    }
    emu.regs_mut().rax = result;
}
