use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn __lconv_init(emu: &mut emu::Emu, addr: u32) {
    let addr = allocate_memory(emu, 64).unwrap_or(0);
    log_red!(emu, "msvcrt!__lconv_init => 0x{:x}", addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
