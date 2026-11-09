use super::allocate_memory;
use crate::emu;

pub fn putchar(emu: &mut emu::Emu, api: &str) {
    let c = emu.regs().rcx as u32;
    let ch_str = format!("{}", c as u8 as char);
    let slot = allocate_memory(emu, 1).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_byte(slot, c as u8);
    }
    log_red!(emu, "msvcrt!{} '{}'", api, ch_str);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = c as u64;
}
