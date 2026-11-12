use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn fprintf(emu: &mut emu::Emu) {
    let _stream = WinApiAbi::arg(emu, 0);
    let fmt_ptr = WinApiAbi::arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!fprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
