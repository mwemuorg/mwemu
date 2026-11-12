use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _crt_atexit(emu: &mut emu::Emu, api: &str) {
    let func_ptr = emu.regs().rcx;
    log_red!(emu, "msvcrt!{} func:0x{:x}", api, func_ptr);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, func_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
