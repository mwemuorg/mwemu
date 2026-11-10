use crate::emu;

pub fn RegisterClipboardFormatA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!RegisterClipboardFormatA '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xc000;
}

pub fn RegisterClipboardFormatW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!RegisterClipboardFormatW '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xc000;
}
