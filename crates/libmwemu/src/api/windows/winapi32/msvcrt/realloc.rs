use super::AllocKind;
use super::allocate_memory;
use super::classify;
use super::release;
use crate::emu;
use crate::kernel::heap::Region;

pub(super) fn realloc(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if addr == 0 {
        if size == 0 {
            emu.regs_mut().rax = 0;
            return;
        } else {
            let base = allocate_memory(emu, size).unwrap_or(0);

            log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", addr, size, base);

            emu.regs_mut().rax = base;
            return;
        }
    }

    if size == 0 {
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", addr, size);

        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
        return;
    }

    let prev_size = match classify(emu, addr) {
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
        .memcpy(new_addr, addr, std::cmp::min(prev_size, size as usize));
    if emu.cfg.memory_guard {
        emu.kernel_free(addr, "realloc");
    } else {
        release(emu, addr);
    }

    log_red!(
        emu,
        "msvcrt!realloc 0x{:x} {} =0x{:x}",
        addr,
        size,
        new_addr
    );

    emu.regs_mut().rax = new_addr;
}
