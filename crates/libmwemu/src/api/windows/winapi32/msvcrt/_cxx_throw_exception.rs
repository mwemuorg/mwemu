use crate::emu;

pub fn _CxxThrowException(emu: &mut emu::Emu) {
    let exc_object = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let throw_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "msvcrt!_CxxThrowException obj:0x{:x} info:0x{:x}",
        exc_object,
        throw_info
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
