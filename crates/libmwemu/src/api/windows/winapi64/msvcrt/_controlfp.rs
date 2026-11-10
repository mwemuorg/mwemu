use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _controlfp(emu: &mut emu::Emu) {
    let new_val = emu.regs().rcx as u32;
    let mask = emu.regs().rdx as u32;
    log_red!(
        emu,
        "msvcrt!_controlfp new:0x{:x} mask:0x{:x}",
        new_val,
        mask
    );
    let result = 0x0009_001fu32;
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
