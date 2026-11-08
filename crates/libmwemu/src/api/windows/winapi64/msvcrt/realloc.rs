use super::AllocKind;
use super::allocate_memory;
use super::classify;
use super::release;
use crate::emu;
use crate::kernel::heap::Region;

pub fn realloc(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = emu.regs().rdx;
    if ptr == 0 {
        if size == 0 {
            emu.regs_mut().rax = 0;
        } else {
            let base = allocate_memory(emu, size).unwrap_or(0);
            log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", ptr, size, base);
            emu.set_last_error(0);
            emu.regs_mut().rax = base;
        }
    } else if size == 0 {
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", ptr, size);
        emu.regs_mut().rax = 0x1337;
    } else {
        let prev_size = match classify(emu, ptr) {
            AllocKind::Arena { size } | AllocKind::Map { size, .. } => size,
            AllocKind::Invalid => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        let new_addr = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "realloc", false)
        } else {
            allocate_memory(emu, size).unwrap_or(0)
        };
        emu.maps
            .memcpy(new_addr, ptr, std::cmp::min(prev_size, size as usize));
        if emu.cfg.memory_guard {
            emu.kernel_free(ptr, "realloc");
        } else {
            release(emu, ptr);
        }
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", ptr, size, new_addr);
        emu.regs_mut().rax = new_addr;
    }
}
