use crate::emu;
use crate::winapi::helper;

pub fn CharNextW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let w = if p0 != 0 {
        emu.maps.read_word(p0).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "kernelbase!CharNextW 0x{:x}", p0);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = if w == 0 { p0 } else { p0 + 2 };
}
