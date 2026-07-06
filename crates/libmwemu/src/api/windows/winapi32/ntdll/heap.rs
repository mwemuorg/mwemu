use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
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

    if name.starts_with("alloc_") {
        emu.maps.dealloc(base_addr);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", name);
        }
    }
}

fn RtlAllocateHeap(emu: &mut emu::Emu) {
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
            format!("alloc_{:x}", base).as_str(),
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
