use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn tolower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u32;
    let result = if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    };
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, result);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
