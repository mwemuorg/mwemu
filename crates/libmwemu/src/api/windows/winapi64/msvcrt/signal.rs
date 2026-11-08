use super::allocate_memory;
use crate::emu;
use crate::winapi::helper;

pub fn signal(emu: &mut emu::Emu) {
    let signum = emu.regs().rcx as u32;
    let handler = emu.regs().rdx;
    log_red!(emu, "msvcrt!signal sig:{} handler:0x{:x}", signum, handler);
    let slot = allocate_memory(emu, 16).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, signum);
        emu.maps.write_qword(slot + 8, handler);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // SIG_DFL (previous handler)
}
