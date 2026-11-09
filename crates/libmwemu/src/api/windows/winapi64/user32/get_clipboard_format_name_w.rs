use crate::emu;

pub fn GetClipboardFormatNameW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "user32!GetClipboardFormatNameW '{}' p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
}
