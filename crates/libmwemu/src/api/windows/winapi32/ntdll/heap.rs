use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

const ALLOC_PREFIX: &str = "alloc_";

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
        "RtlReAllocateHeap" => RtlReAllocateHeap(emu),
        _ => return false,
    }
    true
}

fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlGetProcessHeaps");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlFreeHeap error reading handle param") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlFreeHeap error reading flags param");
    let base_addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlFreeHeap error reading base_addr param") as u64;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(handle);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let name = emu.maps.get_addr_name(base_addr).unwrap_or("").to_string();
    if name.is_empty() {
        if emu.cfg.verbose >= 1 {
            log::trace!("map not allocated, so cannot free it.");
        }
        emu.regs_mut().rax = 0;
        return;
    }

    if name.starts_with(ALLOC_PREFIX) {
        emu.maps.dealloc(base_addr);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", name);
        }
    }
}

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlAllocateHeap error reading handle param") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlAllocateHeap error reading handle param");
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlAllocateHeap error reading handle param") as u64;

    let base = emu
        .maps
        .alloc(size)
        .expect("ntdll!RtlAllocateHeap out of memory");
    emu.maps
        .create_map(
            format!("{}{:x}", ALLOC_PREFIX, base).as_str(),
            base,
            size,
            Permission::READ_WRITE,
        )
        .expect("ntdll!RtlAllocateHeap cannot create map");

    log_red!(emu, "ntdll!RtlAllocateHeap sz: {} addr: 0x{:x}", size, base);

    emu.regs_mut().rax = base;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlReAllocateHeap error reading handle") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlReAllocateHeap error reading flags") as u64;
    let old_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlReAllocateHeap error reading ptr") as u64;
    let new_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!RtlReAllocateHeap error reading size") as u64;

    // stdcall: caller expects us to pop all 4 args regardless of outcome.
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    log_red!(
        emu,
        "ntdll!RtlReAllocateHeap hndl: {:x} flags: {:x} old: 0x{:x} sz: {}",
        handle,
        flags,
        old_ptr,
        new_size
    );

    if old_ptr == 0 || new_size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let old_mem = match emu.maps.get_mem_by_addr(old_ptr) {
        Some(m) => m,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let old_base = old_mem.get_base();
    let old_size = old_mem.size();
    let old_name = old_mem.get_name().to_string();
    if old_base != old_ptr || !old_name.starts_with(ALLOC_PREFIX) {
        emu.regs_mut().rax = 0;
        return;
    }

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if new_size <= old_size as u64 {
            emu.regs_mut().rax = old_ptr;
            return;
        }
        emu.regs_mut().rax = 0;
        return;
    }

    let new_addr = match emu.maps.alloc(new_size) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let new_name = format!("{}{:x}", ALLOC_PREFIX, new_addr);
    if emu
        .maps
        .create_map(&new_name, new_addr, new_size, Permission::READ_WRITE)
        .is_err()
    {
        emu.regs_mut().rax = 0;
        return;
    }

    let copy_size = std::cmp::min(old_size, new_size as usize);
    if !emu.maps.memcpy(new_addr, old_ptr, copy_size) {
        emu.maps.dealloc(new_addr);
        emu.regs_mut().rax = 0;
        return;
    }

    if (flags & constants::HEAP_ZERO_MEMORY) != 0 && (new_size as usize) > old_size {
        emu.maps.memset(
            new_addr + old_size as u64,
            0,
            (new_size as usize) - old_size,
        );
    }

    emu.maps.dealloc(old_ptr);
    emu.regs_mut().rax = new_addr;
}
