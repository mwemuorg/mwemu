use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn exit(emu: &mut emu::Emu) {
    let status = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!exit {}", status);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, status);
    }
    emu.stop();
}
