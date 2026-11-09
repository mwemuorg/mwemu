use crate::emu;

pub(super) fn crt_rand(emu: &mut emu::Emu) {
    let val = (emu
        .pos
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        >> 33)
        & 0x7fff;
    log_red!(emu, "wincrt!rand => {}", val);
    emu.regs_mut().rax = val;
}
