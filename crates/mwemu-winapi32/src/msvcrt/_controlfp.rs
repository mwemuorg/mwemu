use crate::emu;

pub fn _controlfp(emu: &mut emu::Emu) {
    let new_val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let mask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "msvcrt!_controlfp new:0x{:x} mask:0x{:x}",
        new_val,
        mask
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0009_001f; // default x87 control word
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
