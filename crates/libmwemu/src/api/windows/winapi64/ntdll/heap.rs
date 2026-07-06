use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        // kernel32 Heap* are forwarders to these Rtl workers (same arg layout:
        // rcx=heap, rdx=flags, r8=ptr/size), so GetProcAddress-resolved callers
        // that land on the ntdll target hit the same handlers.
        "RtlAllocateHeap" | "HeapAlloc" => RtlAllocateHeap(emu),
        "RtlFreeHeap" | "HeapFree" => RtlFreeHeap(emu),
        "RtlReAllocateHeap" | "HeapReAlloc" => RtlReAllocateHeap(emu),
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeAnsiString" => RtlFreeAnsiString(emu),
        _ => return false,
    }
    true
}

fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mut size = emu.regs().r8;

    if size < 1024 {
        size = 1024
    }
    let alloc_addr = match emu.maps.alloc(size) {
        Some(a) => a,
        None => panic!("/!\\ out of memory cannot allocate ntdll!RtlAllocateHeap"),
    };

    let map_name = format!("valloc_{:x}", alloc_addr);
    emu.maps
        .create_map(&map_name, alloc_addr, size, Permission::READ_WRITE)
        .expect("ntdll!RtlAllocateHeap cannot create map");

    log_red!(
        emu,
        "ntdll!RtlAllocateHeap  hndl: {:x} sz: {}   =addr: 0x{:x}",
        handle,
        size,
        alloc_addr
    );

    emu.regs_mut().rax = alloc_addr;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let base_addr = emu.regs().r8;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(hndl);
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

fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let sz = emu.regs().r8;

    let mapname = format!("valloc_{:x}", hndl);
    emu.regs_mut().rax = match emu.maps.get_map_by_name_mut(&mapname) {
        Some(mem) => {
            mem.set_size(sz + 1024);
            mem.get_base()
        }
        None => 0,
    };

    log_red!(emu, "ntdll!RtlReAllocateHeap hndl: {:x} sz: {}", hndl, sz);
}

fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let num_of_heaps = emu.regs().rcx;
    let out_process_heaps = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlGetProcessHeaps num: {} out: 0x{:x}",
        num_of_heaps,
        out_process_heaps
    );

    emu.regs_mut().rax = 1;
}

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{}", ptr);
}
