use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::virtual_alloc::{permissions, round_up};
use crate::winapi::winapi64::kernel32::set_last_error;
use crate::windows::constants;

use super::virtual_alloc_common::{fail, fail_oom};

pub fn VirtualAllocExNuma(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let orig_size = emu.regs().r8;
    let typ = emu.regs().r9;
    let prot = emu
        .maps
        .read_dword(emu.regs().rsp + 0x20)
        .expect("kernel32!VirtualAllocExNuma cannot read the protect");
    let nnd = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("kernel32!VirtualAllocExNuma cannot read the nndPreferred");

    let size = round_up(orig_size);
    let typ32 = typ as u32;
    let mem_reserve = (typ32 & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ32 & constants::MEM_COMMIT) != 0;
    let (can_read, can_write, can_execute) = permissions(prot);
    let ctx = format!(
        "hproc: 0x{:x} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} nnd: {}",
        proc_hndl, addr, orig_size, size, typ, prot, nnd
    );

    if orig_size == 0 {
        fail(emu, "VirtualAllocExNuma", &ctx, "zero size");
        return;
    }

    if !mem_reserve && !mem_commit {
        fail(
            emu,
            "VirtualAllocExNuma",
            &ctx,
            "unsupported allocation type",
        );
        return;
    }

    if mem_commit && !mem_reserve && addr > 0 && !emu.maps.is_allocated(addr) {
        fail(emu, "VirtualAllocExNuma", &ctx, "commit target unmapped");
        return;
    }

    let base: u64 = if mem_commit && !mem_reserve && addr > 0 {
        addr
    } else if addr > 0 {
        if emu.maps.is_allocated(addr) {
            fail(emu, "VirtualAllocExNuma", &ctx, "address already mapped");
            return;
        }
        addr
    } else {
        match emu.maps.alloc(size) {
            Some(b) => b,
            None => {
                fail_oom(emu, "VirtualAllocExNuma", &ctx);
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
        set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY);
        emu.regs_mut().rax = 0;
        return;
    }

    if mem_commit && !mem_reserve {
        set_last_error(constants::ERROR_SUCCESS);
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
        return;
    }

    set_last_error(constants::ERROR_SUCCESS);
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
}
