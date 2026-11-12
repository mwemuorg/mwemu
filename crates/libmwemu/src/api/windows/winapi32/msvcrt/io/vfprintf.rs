use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn vfprintf(emu: &mut emu::Emu) {
    let _stream = WinApiAbi::arg(emu, 0);
    let fmt_ptr = WinApiAbi::arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!vfprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
