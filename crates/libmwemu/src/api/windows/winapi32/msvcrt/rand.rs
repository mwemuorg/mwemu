use crate::emu;
use crate::winapi::helper;

pub fn rand(emu: &mut emu::Emu) {
    let val = ((emu.tick as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        >> 33) as u32
        & 0x7fff;
    log_red!(emu, "msvcrt!rand => {}", val);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = val as u64;
}
