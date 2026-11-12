use crate::emu;
use crate::winapi::helper;

pub fn putc(emu: &mut emu::Emu, api: &str) {
    let ch = emu.regs().rcx;
    log_red!(
        emu,
        "wincrt!{} ch:'{}' (0x{:x})",
        api,
        (ch & 0xff) as u8 as char,
        ch
    );
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = ch & 0xff;
}
