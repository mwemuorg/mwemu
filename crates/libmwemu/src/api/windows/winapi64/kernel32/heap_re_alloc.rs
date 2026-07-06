use crate::maps::mem64::Permission;
use crate::{emu, windows::constants};

pub fn HeapReAlloc(emu: &mut emu::Emu) {
    let heap_handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let old_mem = emu.regs().r8;
    let new_size = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!HeapReAlloc heap: 0x{:x} flags: 0x{:x} old_mem: 0x{:x} new_size: {}",
        heap_handle,
        flags,
        old_mem,
        new_size
    );

    // Check if we're dealing with a valid memory pointer
    if !emu.maps.is_mapped(old_mem) {
        emu.regs_mut().rax = 0;
        return;
    }

    // Allocate new block
    match emu.maps.alloc(new_size) {
        Some(new_addr) => {
            // Create new memory map for the allocated space
            if emu.maps.create_map(
                format!("alloc_{:x}", new_addr).as_str(),
                new_addr,
                new_size,
                Permission::READ_WRITE,
            ).is_err() {
                emu.regs_mut().rax = 0;
                return;
            }

            // Copy old content to new location if HEAP_REALLOC_IN_PLACE_ONLY is not set
            if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) == 0 {
                // Get the size of the old allocation to know how much to copy
                let old_size = emu.maps.get_mem_size(old_mem).unwrap_or(new_size as usize);
                let copy_size = std::cmp::min(old_size, new_size as usize);

                // Copy the data
                if !emu.maps.memcpy(old_mem, new_addr, copy_size) {
                    emu.regs_mut().rax = 0;
                    return;
                }

                // If HEAP_ZERO_MEMORY is set and we're expanding, zero the additional memory
                if (flags & constants::HEAP_ZERO_MEMORY) != 0 && new_size > old_size as u64 {
                    let zero_start = new_addr + old_size as u64;
                    let zero_size = (new_size - old_size as u64) as usize;
                    emu.maps.memset(zero_start, 0, zero_size);
                }
            } else {
                // HEAP_REALLOC_IN_PLACE_ONLY was set but we couldn't comply
                emu.regs_mut().rax = 0;
                return;
            }

            emu.regs_mut().rax = new_addr;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }
}
