use crate::emu;
use crate::maps::mem64::Permission;

pub fn HeapAlloc(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapAlloc cannot read the handle");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapAlloc cannot read the flags");
    let mut size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapAlloc cannot read the size") as u64;

    // Apply minimum padding
    if size < emu.cfg.heap_alloc_min_size {
        size = emu.cfg.heap_alloc_min_size;
    }

    let heap_addr: u64 = if size < 0x8000 {
        let heap_manage = emu.heap_mut();
        heap_manage
            .allocate(size as usize)
            .expect("failed to allocate heap")
    } else {
        let allocation = emu.maps.alloc(size).unwrap_or_default();
        emu.maps
            .create_map(
                format!("alloc_{:x}", allocation).as_str(),
                allocation,
                size,
                Permission::READ_WRITE,
            )
            .expect("kernel32!HeapAlloc out of memory");
        allocation
    };

    emu.regs_mut().rax = heap_addr;

    log_red!(
        emu,
        "kernel32!HeapAlloc eip: 0x{:x} flags: 0x{:x} size: {} =0x{:x}",
        emu.regs().get_eip(),
        flags,
        size,
        emu.regs().get_eax() as u32
    );

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
