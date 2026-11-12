use crate::emu;
use crate::winapi::common::strings;

pub fn VkKeyScanA(emu: &mut emu::Emu) {
    vk_key_scan(emu, "VkKeyScanA", false);
}

fn vk_key_scan(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}'", s);
}

pub fn VkKeyScanW(emu: &mut emu::Emu) {
    vk_key_scan(emu, "VkKeyScanW", true);
}
