use crate::emu;

pub fn wcsftime(emu: &mut emu::Emu) {
    let wcs = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _maxsize = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _timeptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if wcs != 0 && !fmt.is_empty() {
        emu.maps.write_wide_string(wcs, &fmt);
    }
    log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());

    emu.regs_mut().rax = fmt.len() as u64;
}
