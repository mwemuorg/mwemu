use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let _options = WinApiAbi::arg(emu, 0) as u32;
    let _options_hi = WinApiAbi::arg(emu, 1) as u32;
    let _stream = WinApiAbi::arg(emu, 2) as u32;
    let fmt_ptr = WinApiAbi::arg(emu, 3);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
