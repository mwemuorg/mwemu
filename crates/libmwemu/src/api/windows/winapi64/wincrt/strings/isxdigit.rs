use crate::emu;
use crate::winapi::helper;

pub fn isxdigit(emu: &mut emu::Emu) {
    let ch = emu.regs().rcx as u8;
    log_red!(emu, "wincrt!isxdigit ch:{}", ch as char);
    helper::advance_tick(emu, 1);
    let result = if ch.is_ascii_hexdigit() { 1u64 } else { 0u64 };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
