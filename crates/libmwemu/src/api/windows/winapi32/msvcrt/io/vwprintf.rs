use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn vwprintf(emu: &mut emu::Emu) {
    let fmt_ptr = WinApiAbi::arg(emu, 0);
    let _va_list = WinApiAbi::arg(emu, 1) as u32;
    let fmt = emu.maps.read_wide_string(fmt_ptr);
    log_red!(emu, "msvcrt!vwprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
