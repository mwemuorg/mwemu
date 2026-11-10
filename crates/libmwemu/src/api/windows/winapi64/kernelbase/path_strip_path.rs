use crate::emu;

pub fn PathStripPathA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!PathStripPathA '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PathStripPathW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!PathStripPathW '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
