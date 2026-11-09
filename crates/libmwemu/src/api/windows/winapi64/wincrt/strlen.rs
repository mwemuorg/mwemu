use crate::emu;

pub(super) fn strlen(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let mut n: u64 = 0;
    while let Some(b) = emu.maps.read_byte(s + n) {
        if b == 0 {
            break;
        }
        n += 1;
        if n > 0x10_0000 {
            break;
        }
    }
    emu.regs_mut().rax = n;
}
