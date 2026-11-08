use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
const MEM_RELEASE: u64 = 0x8000;
pub fn VirtualFreeEx(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let lp_address = arg(emu, 1) as u64;
    let dw_size = arg(emu, 2) as u64;
    let dw_free_type = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!VirtualFreeEx hProcess:0x{:x} addr:0x{:x} size:{} type:0x{:x}",
        h_process,
        lp_address,
        dw_size,
        dw_free_type
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    if (dw_free_type & MEM_RELEASE) != 0 && lp_address != 0 {
        // Try to free the memory map allocated by VirtualAllocEx
        emu.maps.free(&format!("alloc_{:x}", lp_address));
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 4);
}
