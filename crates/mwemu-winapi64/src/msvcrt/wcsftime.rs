use crate::emu;

pub fn wcsftime(emu: &mut emu::Emu) {
    let wcs = emu.regs().rcx;
    let _maxsize = emu.regs().rdx;
    let fmt_ptr = emu.regs().r8;
    let _timeptr = emu.regs().r9;
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if wcs != 0 && !fmt.is_empty() {
        emu.maps.write_wide_string(wcs, &fmt);
    }
    log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());
    emu.set_last_error(0);
    emu.regs_mut().rax = fmt.len() as u64;
}
