use crate::emu;

pub fn VkKeyScanExW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!VkKeyScanExW '{}' p1:0x{:x}", s, p1);
}
