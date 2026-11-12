use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn wcsftime(emu: &mut emu::Emu) {
    let wcs = WinApiAbi::arg(emu, 0);
    let _maxsize = WinApiAbi::arg(emu, 1) as u32;
    let fmt_ptr = WinApiAbi::arg(emu, 2);
    let _timeptr = WinApiAbi::arg(emu, 3) as u32;
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    if wcs != 0 && !fmt.is_empty() {
        emu.maps.write_wide_string(wcs, &fmt);
    }
    log_red!(emu, "msvcrt!wcsftime fmt:'{}' => {}", fmt, fmt.len());

    emu.regs_mut().rax = fmt.len() as u64;
}
