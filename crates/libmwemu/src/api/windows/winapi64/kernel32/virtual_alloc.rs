use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::virtual_alloc::{permissions, round_up};
use crate::winapi::winapi64::kernel32::set_last_error;
use crate::windows::constants;

use super::virtual_alloc_common::{fail, fail_oom};

pub fn VirtualAlloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let orig_size = emu.regs().rdx;
    let typ = emu.regs().r8 as u32;
    let prot = emu.regs().r9 as u32;
    let size = round_up(orig_size);
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let (can_read, can_write, can_execute) = permissions(prot);
    let ctx = format!(
        "addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {}",
        addr, orig_size, size, typ, prot
    );

    if orig_size == 0 {
        fail(emu, "VirtualAlloc", &ctx, "zero size");
        return;
    }

    if !mem_reserve && !mem_commit {
        fail(emu, "VirtualAlloc", &ctx, "unsupported allocation type");
        return;
    }

    if mem_commit && !mem_reserve && addr > 0 && !emu.maps.is_allocated(addr) {
        fail(emu, "VirtualAlloc", &ctx, "commit target unmapped");
        return;
    }

    let base: u64 = if mem_commit && !mem_reserve && addr > 0 {
        addr
    } else if addr > 0 {
        if emu.maps.is_allocated(addr) {
            fail(emu, "VirtualAlloc", &ctx, "address already mapped");
            return;
        }
        addr
    } else {
        match emu.maps.alloc(size) {
            Some(b) => b,
            None => {
                fail_oom(emu, "VirtualAlloc", &ctx);
                return;
            }
        }
    };

    if mem_commit && !mem_reserve {
        set_last_error(constants::ERROR_SUCCESS);
        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
            addr,
            orig_size,
            size,
            typ,
            prot,
            base
        );
        emu.regs_mut().rax = base;
        return;
    }

    if let Err(err) = emu.maps.create_map(
        format!("alloc_{:x}", base).as_str(),
        base,
        size,
        Permission::from_flags(can_read, can_write, can_execute),
    ) {
        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {} flags: {} prot: {} = 0 reason: create_map failed: {}",
            addr,
            size,
            typ,
            prot,
            err
        );
        set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY);
        emu.regs_mut().rax = 0;
        return;
    }

    set_last_error(constants::ERROR_SUCCESS);

    log_red!(
        emu,
        "kernel32!VirtualAlloc addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
        addr,
        orig_size,
        size,
        typ,
        prot,
        base
    );
    emu.regs_mut().rax = base;
}
