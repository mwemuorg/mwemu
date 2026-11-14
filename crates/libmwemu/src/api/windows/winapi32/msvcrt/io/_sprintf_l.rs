use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _sprintf_l(emu: &mut emu::Emu) {
    let buf = WinApiAbi::arg(emu, 0);
    let fmt_ptr = WinApiAbi::arg(emu, 1);
    let _locale = WinApiAbi::arg(emu, 2) as u32;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!_sprintf_l buf:0x{:x} fmt:'{}'", buf, fmt);
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }

    emu.regs_mut().rax = fmt.len() as u64;
}
