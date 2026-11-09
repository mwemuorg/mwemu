use crate::emu;

pub fn atan2(emu: &mut emu::Emu) {
    let y = f64::from_bits(
        (emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64) << 32
            | emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64,
    );
    let x = f64::from_bits(
        (emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64) << 32
            | emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64,
    );
    let result = y.atan2(x);
    log_red!(emu, "msvcrt!atan2 {} {} => {}", y, x, result);
    emu.fpu_mut().set_st(0, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
