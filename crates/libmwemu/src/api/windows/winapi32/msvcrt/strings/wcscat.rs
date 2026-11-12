use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn wcscat(emu: &mut emu::Emu) {
    let dst = WinApiAbi::arg(emu, 0);
    let src = WinApiAbi::arg(emu, 1);
    let existing = emu.maps.read_wide_string(dst);
    let append = emu.maps.read_wide_string(src);
    let result = format!("{}{}", existing, append);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &result);
    }
    log_red!(emu, "msvcrt!wcscat => '{}'", result);

    emu.regs_mut().rax = dst;
}
