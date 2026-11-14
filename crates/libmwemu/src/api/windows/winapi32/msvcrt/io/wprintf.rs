use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn wprintf(emu: &mut emu::Emu) {
    let fmt_ptr = WinApiAbi::arg(emu, 0);
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    log_red!(emu, "msvcrt!wprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
