use crate::emu;

pub fn __setusermatherr(emu: &mut emu::Emu) {
    let handler = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "msvcrt!__setusermatherr handler:0x{:x}", handler);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
