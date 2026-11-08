use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn HeapSize(emu: &mut emu::Emu) {
    let mem = arg(emu, 2) as u64;
    let size = heap_engine::heap_allocation_size(emu, mem);
    log_red!(emu, "kernel32!HeapSize mem: 0x{:x} ={:?}", mem, size);
    emu.regs_mut().rax = size.map(|s| s as u64).unwrap_or(0xffff_ffff);
    stdcall_ret(emu, 3);
}
