use super::allocate_memory;
use crate::emu;
use crate::kernel::heap::Region;

pub fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    if size > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
        } else {
            allocate_memory(emu, size).unwrap_or(0)
        };
        log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);
        emu.set_last_error(0);
        emu.regs_mut().rax = base;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0x1337;
    }
}
