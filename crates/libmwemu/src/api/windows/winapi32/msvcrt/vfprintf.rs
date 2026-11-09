use crate::emu;

pub fn vfprintf(emu: &mut emu::Emu) {
    let _stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!vfprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
