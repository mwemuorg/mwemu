use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn LocalLock(emu: &mut emu::Emu) {
    let h_mem = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!LocalLock hMem:0x{:x}", h_mem);
    let handle = helper::handler_create("LocalLock");
    // For LMEM_FIXED allocations, the handle is the pointer itself
    emu.set_last_error(0);
    emu.regs_mut().rax = h_mem;
    stdcall_ret(emu, 1);
}
