use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GlobalFree(emu: &mut emu::Emu) {
    let mem = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GlobalFree mem: 0x{:x}", mem);
    heap_engine::heap_free(emu, 0, mem);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
