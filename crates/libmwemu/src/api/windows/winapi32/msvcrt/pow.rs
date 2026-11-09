use crate::emu;

pub fn pow(emu: &mut emu::Emu) {
    let base_val = f64::from_bits(
        (emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64) << 32
            | emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64,
    );
    let exp_val = f64::from_bits(
        (emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64) << 32
            | emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64,
    );
    let result = base_val.powf(exp_val);
    log_red!(emu, "msvcrt!pow {} {} => {}", base_val, exp_val, result);
    emu.fpu_mut().set_st(0, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
