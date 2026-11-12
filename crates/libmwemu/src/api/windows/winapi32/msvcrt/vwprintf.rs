use crate::emu;

pub fn vwprintf(emu: &mut emu::Emu) {
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _va_list = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    log_red!(emu, "msvcrt!vwprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
