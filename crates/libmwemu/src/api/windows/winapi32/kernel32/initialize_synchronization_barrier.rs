use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn InitializeSynchronizationBarrier(emu: &mut emu::Emu) {
    let barrier = arg(emu, 0) as u64;
    let spin_count = arg(emu, 1);
    let num_threads = arg(emu, 2);
    log_red!(
        emu,
        "kernel32!InitializeSynchronizationBarrier barrier:0x{:x} threads:{}",
        barrier,
        num_threads
    );
    let handle = helper::handler_create("InitializeSynchronizationBarrier");
    let out_ptr = arg(emu, 2) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 3);
}
