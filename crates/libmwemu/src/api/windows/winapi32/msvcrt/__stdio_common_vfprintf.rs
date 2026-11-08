use crate::emu;

pub fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _stream = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
