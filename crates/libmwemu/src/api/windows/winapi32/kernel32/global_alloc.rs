use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GlobalAlloc(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let size = arg(emu, 1) as u64;
    let zeroinit = flags & 0x0040 != 0; // GMEM_ZEROINIT
    let addr = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    if addr != 0 && zeroinit {
        emu.maps.memset(addr, 0, size as usize);
    }
    log_red!(
        emu,
        "kernel32!GlobalAlloc flags: 0x{:x} size: {} =0x{:x}",
        flags,
        size,
        addr
    );
    emu.regs_mut().rax = addr;
    stdcall_ret(emu, 2);
}
