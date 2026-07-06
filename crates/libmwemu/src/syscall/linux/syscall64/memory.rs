use crate::emu;
use crate::windows::constants;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_BRK => handle_syscall64_brk(emu),
        constants::NR64_NANOSLEEP => handle_syscall64_nanosleep(emu),
        constants::NR64_MREMAP => handle_syscall64_mremap(emu),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_brk(emu: &mut emu::Emu) {
    // Real-libc mode: ld.so bootstrap leaves no heap, so `brk` must return a
    // real, growable program break. A bogus break (0 / the requested value
    // with no backing map) makes glibc's main malloc arena unusable and it
    // falls back to a half-broken mmap arena whose chunk metadata gets
    // corrupted (manifesting as `free(): invalid size`, truncated strings, or
    // infinite hash loops in dependent programs like id / ls -l).
    if emu.cfg.linux_real_libc && !emu.cfg.arch.is_aarch64() {
        use crate::maps::mem64::Permission;
        const HEAP_START: u64 = 0x6000_0000_0000;
        const HEAP_MAX: u64 = 0x0800_0000; // 128 MiB
        let requested = emu.regs().rdi;

        if emu.heap_addr < HEAP_START || emu.heap_addr > HEAP_START + HEAP_MAX {
            emu.heap_addr = HEAP_START;
            if !emu.maps.is_mapped(HEAP_START) {
                let _ = emu
                    .maps
                    .create_map("heap", HEAP_START, HEAP_MAX, Permission::READ_WRITE);
            }
        }
        if (HEAP_START..=HEAP_START + HEAP_MAX).contains(&requested) {
            emu.heap_addr = requested; // move the program break
        }
        emu.regs_mut().rax = emu.heap_addr;
        log::trace!(
            "{}** {} syscall brk(0x{:x}) =0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            requested,
            emu.heap_addr,
            emu.colors.nc
        );
        return;
    }

    let heap_base = 0x4b5b00;
    let heap_size = 0x4d8000 - 0x4b5000;

    /*
    heap = emu
        .maps
        .create_map("heap", heap_base, heap_size)
        .expect("cannot create heap map from brk syscall");
    */

    if emu.regs_mut().rdi == 0 {
        emu.regs_mut().r11 = 0x346;
        emu.regs_mut().rcx = 0x4679f7;
        emu.regs_mut().rax = emu.heap_addr;
    } else {
        // let bottom = emu.regs().rdi;
        // let new_sz = bottom - heap_base;
        // heap.set_size(new_sz);
        emu.regs_mut().rax = emu.regs().rdi;
        emu.regs_mut().rcx = 0x4679f7;
        emu.regs_mut().rdx = 0x2f;
        emu.regs_mut().r11 = 0x302;
    }

    //emu.fs.insert(0xffffffffffffffc8, 0x4b6c50);

    log::trace!(
        "{}** {} syscall brk({:x}) ={:x} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rdi,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_nanosleep(emu: &mut emu::Emu) {
    let req = emu.regs().rdi;
    let tv_sec = emu.maps.read_qword(req).unwrap_or(0);
    let tv_nsec = emu.maps.read_qword(req + 8).unwrap_or(0);

    log::info!(
        "{}** {} syscall nanosleep(tv_sec: 0x{:x}, tv_nsec: 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        tv_sec,
        tv_nsec,
        emu.colors.nc
    );

    // TODO: implement actual sleep
    emu.regs_mut().rax = 0;
}

pub(super) fn handle_syscall64_mremap(emu: &mut emu::Emu) {
    let old_addr = emu.regs().rdi;
    let old_sz = emu.regs().rsi;
    let new_sz = emu.regs().rdx;
    let flags = emu.regs().r10;
    let new_addr = emu.regs().r8;

    log::info!(
        "{}** {} syscall mremap(old_addr: 0x{:x}, old_sz: 0x{:x}, new_sz: 0x{:x}, flags: 0x{:x}, new_addr: 0x{:x}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        old_addr,
        old_sz,
        new_sz,
        flags,
        new_addr,
        emu.regs().rax,
        emu.colors.nc
    );

    // TODO: implement actual mremap
    emu.regs_mut().rax = 0;
}
