
use crate::emu;
use crate::winapi::helper;
pub fn InitializeSynchronizationBarrier(emu: &mut emu::Emu) {
    let barrier = emu.regs().rcx;
    let spin_count = emu.regs().rdx as u32;
    let num_threads = emu.regs().r8 as u32;
    log_red!(
        emu,
        "kernel32!InitializeSynchronizationBarrier barrier:0x{:x} threads:{}",
        barrier,
        num_threads
    );
    let handle = helper::handler_create("InitializeSynchronizationBarrier");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
