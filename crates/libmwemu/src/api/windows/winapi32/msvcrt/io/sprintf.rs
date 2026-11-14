use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn sprintf(emu: &mut emu::Emu) {
    let buf = WinApiAbi::arg(emu, 0);
    let fmt_ptr = WinApiAbi::arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_string(buf, &fmt);
    }
    log_red!(emu, "msvcrt!sprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
