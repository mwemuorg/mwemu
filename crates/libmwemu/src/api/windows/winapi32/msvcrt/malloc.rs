use super::allocate_memory;
use crate::emu;
use crate::kernel::heap::Region;

pub(super) fn malloc(emu: &mut emu::Emu) {
    let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if size > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
        } else {
            allocate_memory(emu, size).unwrap_or(0)
        };

        log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);

        emu.regs_mut().rax = base;
    } else {
        emu.regs_mut().rax = 0x1337;
    }
}
