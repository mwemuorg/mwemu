use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::virtual_alloc::{permissions, round_up};
use crate::winapi::winapi32::kernel32::set_last_error;
use crate::windows::constants;

use super::virtual_alloc_common::{fail, fail_oom};

pub fn VirtualAllocEx(emu: &mut emu::Emu) {
    let proc_hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!VirtualAllocEx cannot read the proc handle") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualAllocEx cannot read the address") as u64;
    let orig_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualAllocEx cannot read the size") as u64;
    let typ = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!VirtualAllocEx cannot read the type");
    let prot = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!VirtualAllocEx cannot read the protect");

    let size = round_up(orig_size);
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let (can_read, can_write, can_execute) = permissions(prot);
    let ctx = format!(
        "hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {}",
        proc_hndl, addr, orig_size, size, typ, prot
    );

    if orig_size == 0 {
        fail(emu, "VirtualAllocEx", &ctx, "zero size");
        for _ in 0..5 {
            emu.stack_pop32(false);
        }
        return;
    }

    if !mem_reserve && !mem_commit {
        fail(emu, "VirtualAllocEx", &ctx, "unsupported allocation type");
        for _ in 0..5 {
            emu.stack_pop32(false);
        }
        return;
    }

    if mem_commit && !mem_reserve && addr > 0 && !emu.maps.is_allocated(addr) {
        fail(emu, "VirtualAllocEx", &ctx, "commit target unmapped");
        for _ in 0..5 {
            emu.stack_pop32(false);
        }
        return;
    }

    let base: u64 = if mem_commit && !mem_reserve && addr > 0 {
        addr
    } else if addr > 0 {
        if emu.maps.is_allocated(addr) {
            fail(emu, "VirtualAllocEx", &ctx, "address already mapped");
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            return;
        }
        addr
    } else {
        match emu.maps.alloc(size) {
            Some(b) => b,
            None => {
                fail_oom(emu, "VirtualAllocEx", &ctx);
                for _ in 0..5 {
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
            "kernel32!VirtualAllocEx hproc: 0x{:x} addr: 0x{:x} sz: {} flags: {} prot: {} = 0 reason: create_map failed: {}",
            proc_hndl,
            addr,
            size,
            typ,
            prot,
            err
        );
        set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY as u32);
        emu.regs_mut().rax = 0;
        for _ in 0..5 {
            emu.stack_pop32(false);
        }
        return;
    }

    if mem_commit && !mem_reserve {
        set_last_error(constants::ERROR_SUCCESS as u32);
        log_red!(
            emu,
            "kernel32!VirtualAllocEx hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
            proc_hndl,
            addr,
            orig_size,
            size,
            typ,
            prot,
            base
        );
        emu.regs_mut().rax = base;
        for _ in 0..5 {
            emu.stack_pop32(false);
        }
        return;
    }

    set_last_error(constants::ERROR_SUCCESS as u32);
    log_red!(
        emu,
        "kernel32!VirtualAllocEx hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
        proc_hndl,
        addr,
        orig_size,
        size,
        typ,
        prot,
        base
    );
    emu.regs_mut().rax = base;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
