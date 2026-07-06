use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::virtual_alloc::{permissions, round_up};
use crate::winapi::winapi32::kernel32::set_last_error;
use crate::windows::constants;

use super::virtual_alloc_common::{fail, fail_oom};

pub fn VirtualAllocExNuma(emu: &mut emu::Emu) {
    let proc_hndl =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!VirtualAllocExNuma cannot read the proc handle") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualAllocExNuma cannot read the address") as u64;
    let orig_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualAllocExNuma cannot read the size") as u64;
    let typ = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!VirtualAllocExNuma cannot read the type");
    let prot = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!VirtualAllocExNuma cannot read the protect");
    let nnd = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!VirtualAllocExNuma cannot read the nndPreferred");

    let size = round_up(orig_size);
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let (can_read, can_write, can_execute) = permissions(prot);
    let ctx = format!(
        "hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} nnd: {}",
        proc_hndl, addr, orig_size, size, typ, prot, nnd
    );

    if orig_size == 0 {
        fail(emu, "VirtualAllocExNuma", &ctx, "zero size");
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        return;
    }

    if !mem_reserve && !mem_commit {
        fail(
            emu,
            "VirtualAllocExNuma",
            &ctx,
            "unsupported allocation type",
        );
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        return;
    }

    if mem_commit && !mem_reserve && addr > 0 && !emu.maps.is_allocated(addr) {
        fail(emu, "VirtualAllocExNuma", &ctx, "commit target unmapped");
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        return;
    }

    let base: u64 = if mem_commit && !mem_reserve && addr > 0 {
        addr
    } else if addr > 0 {
        if emu.maps.is_allocated(addr) {
            fail(emu, "VirtualAllocExNuma", &ctx, "address already mapped");
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            return;
        }
        addr
    } else {
        match emu.maps.alloc(size) {
            Some(b) => b,
            None => {
                fail_oom(emu, "VirtualAllocExNuma", &ctx);
                for _ in 0..6 {
                    emu.stack_pop32(false);
                }
                return;
            }
        }
    };

    if let Err(err) = emu.maps.create_map(
        format!("alloc_{:x}", base).as_str(),
        base,
        size,
        Permission::from_flags(can_read, can_write, can_execute),
    ) {
        log_red!(
            emu,
            "kernel32!VirtualAllocExNuma hproc: 0x{:x} addr: 0x{:x} sz: {} flags: {} prot: {} nnd: {} = 0 reason: create_map failed: {}",
            proc_hndl,
            addr,
            size,
            typ,
            prot,
            nnd,
            err
        );
        set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY as u32);
        emu.regs_mut().rax = 0;
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        return;
    }

    if mem_commit && !mem_reserve {
        set_last_error(constants::ERROR_SUCCESS as u32);
        log_red!(
            emu,
            "kernel32!VirtualAllocExNuma hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} nnd: {} = 0x{:x}",
            proc_hndl,
            addr,
            orig_size,
            size,
            typ,
            prot,
            nnd,
            base
        );
        emu.regs_mut().rax = base;
        for _ in 0..6 {
            emu.stack_pop32(false);
        }
        return;
    }

    set_last_error(constants::ERROR_SUCCESS as u32);
    log_red!(
        emu,
        "kernel32!VirtualAllocExNuma hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} nnd: {} = 0x{:x}",
        proc_hndl,
        addr,
        orig_size,
        size,
        typ,
        prot,
        nnd,
        base
    );
    emu.regs_mut().rax = base;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
