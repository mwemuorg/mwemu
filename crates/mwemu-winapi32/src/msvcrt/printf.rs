use crate::emu;

pub fn printf(emu: &mut emu::Emu) {
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!printf fmt:'{}'", fmt);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
