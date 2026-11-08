use super::allocate_memory;
use crate::emu;

pub fn abs(emu: &mut emu::Emu, api: &str) {
    let val = emu.regs().rcx as i32;
    let result = val.unsigned_abs();
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
