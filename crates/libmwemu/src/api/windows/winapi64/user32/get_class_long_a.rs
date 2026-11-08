use crate::emu;

pub fn GetClassLongA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!GetClassLongA '{}' p1:0x{:x}", s, p1);
}
