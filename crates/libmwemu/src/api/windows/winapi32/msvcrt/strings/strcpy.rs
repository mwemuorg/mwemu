use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strcpy(emu: &mut emu::Emu) {
    let dst = WinApiAbi::arg(emu, 0);
    let src = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_string(src);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "msvcrt!strcpy dst:0x{:x} '{}'", dst, s);

    emu.regs_mut().rax = dst;
}
