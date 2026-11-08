use crate::emu;

pub fn CharNextA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let b = if p0 != 0 {
        emu.maps.read_byte(p0).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "kernelbase!CharNextA 0x{:x}", p0);
    emu.set_last_error(0);
    emu.regs_mut().rax = if b == 0 { p0 } else { p0 + 1 };
}
