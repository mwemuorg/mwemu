use crate::emu;

pub fn wprintf(emu: &mut emu::Emu) {
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    log_red!(emu, "msvcrt!wprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
