use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::constants::*;
use crate::windows::structures::MemoryBasicInformation64;
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

fn permission_to_nt_page_protection(perm: Permission) -> u32 {
    const PAGE_NOACCESS: u32 = 0x01;
    const PAGE_READONLY: u32 = 0x02;
    const PAGE_READWRITE: u32 = 0x04;
    const PAGE_EXECUTE: u32 = 0x10;
    const PAGE_EXECUTE_READ: u32 = 0x20;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    let r = perm.contains(Permission::READ);
    let w = perm.contains(Permission::WRITE);
    let x = perm.contains(Permission::EXECUTE);
    match (r, w, x) {
        (false, false, false) => PAGE_NOACCESS,
        (true, false, false) => PAGE_READONLY,
        (true, true, false) => PAGE_READWRITE,
        (false, false, true) => PAGE_EXECUTE,
        (true, false, true) => PAGE_EXECUTE_READ,
        (true, true, true) => PAGE_EXECUTE_READWRITE,
        _ => PAGE_NOACCESS,
    }
}

fn nt_page_protection_to_permission(protect: u32) -> Permission {
    const PAGE_READONLY: u32 = 0x02;
    const PAGE_READWRITE: u32 = 0x04;
    const PAGE_WRITECOPY: u32 = 0x08;
    const PAGE_EXECUTE: u32 = 0x10;
    const PAGE_EXECUTE_READ: u32 = 0x20;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

    let can_read = (protect
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_write = (protect
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_execute = (protect
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    Permission::from_flags(can_read, can_write, can_execute)
}

fn align_up_page(size: u64) -> u64 {
    const PAGE: u64 = 0x1000;
    size.saturating_add(PAGE - 1) & !(PAGE - 1)
}

/// `create_map` backs every region with an eager `vec![0; size]`, so a multi-GB
/// reservation (e.g. an ntdll heap `MEM_RESERVE` of 0x100020000) would stall the
/// host zeroing RAM — or OOM — even though real Windows commits no pages for a
/// reserve. Cap the backing size to `max_alloc_size`, mirroring the truncation
/// `Maps::alloc` already applies when it picks the base address.
fn clamp_backing_size(emu: &Emu, size: u64) -> u64 {
    let cap = align_up_page(emu.maps.max_alloc_size);
    size.min(cap)
}

/// ntdll heap expansion (`RtlAllocateHeap` internals) can leave a `LIST_ENTRY` at
/// `allocBase + 0xb000` zeroed; the next walk does `mov rax,[rsi]` / follow Flink and faults.
/// Prime a self-linked sentinel at that offset for large private regions (observed with
/// `maps/windows/x86_64/ntdll.dll` during `LdrInitializeThunk` under `--ssdt --init`).
fn patch_ldr_heap_list_sentinel(emu: &mut Emu, base: u64, size: u64) {
    if !emu.cfg.emulate_winapi {
        return;
    }
    const OFF: u64 = 0xb000;
    if size < OFF + 0x10 {
        return;
    }
    let p = base.saturating_add(OFF);
    if !emu.maps.is_mapped(p) || !emu.maps.is_mapped(p + 8) {
        return;
    }
    let _ = emu.maps.write_qword(p, p);
    let _ = emu.maps.write_qword(p + 8, p);
}

/// Runs immediately before `mov rax, qword ptr [rsi]` inside **ntdll** while `RSI` points at
/// emulated heap backing (`ALLOC64_*`).  ntdll `memset`s after `NtAllocateVirtualMemory*` and
/// clears Flink/Blink; self-link when both are still zero.
///
/// Does NOT apply when the instruction that immediately follows the load is a null-check of the
/// destination register (`test reg,reg` or `cmp reg,0`).  In that case the calling code handles
/// an empty/null LIST_ENTRY explicitly and the self-link would send execution down the wrong path
/// (e.g. the loader worker-thread list in `LdrInitializeThunk`).
pub fn ntdll_heap_list_walk_fixup(emu: &mut Emu, ins: &Instruction, rip: u64) {
    if !emu.cfg.emulate_winapi {
        return;
    }
    // Cheap instruction-shape checks FIRST (they run per-instruction, so the
    // expensive map lookup below must only happen for the handful of candidates).
    // We only care about `mov r64, [rsi]`.
    if ins.mnemonic() != Mnemonic::Mov || ins.op_count() < 2 {
        return;
    }
    if ins.op1_kind() != OpKind::Memory {
        return;
    }
    if ins.memory_base() != Register::RSI || ins.memory_index() != Register::None {
        return;
    }
    if ins.memory_displacement64() != 0 {
        return;
    }
    // Code lives in per-section maps (`ntdll.text`, …), not only the small `ntdll.pe` header map.
    let in_ntdll = emu
        .maps
        .get_mem_by_addr(rip)
        .map(|m| {
            let n = m.get_name();
            n == "ntdll.pe" || n.starts_with("ntdll.")
        })
        .unwrap_or(false);
    if !in_ntdll {
        return;
    }

    let rsi = emu.regs().rsi;
    if !(ALLOC64_MIN..ALLOC64_MAX).contains(&rsi) {
        return;
    }
    if !emu.maps.is_mapped(rsi) || !emu.maps.is_mapped(rsi + 8) {
        return;
    }
    if emu.maps.read_qword(rsi).unwrap_or(1) != 0 || emu.maps.read_qword(rsi + 8).unwrap_or(1) != 0
    {
        return;
    }

    // If the next instruction is a null-check on the loaded value the caller handles the
    // empty-list case itself — applying the self-link fixup here would bypass that branch.
    //   test r64,r64  =>  48 85 xx
    //   test r32,r32  =>  85 xx
    //   cmp  r64,imm8 =>  48 83 Fx xx  (e.g. cmp rax,0 = 48 83 F8 00)
    let next = rip.wrapping_add(ins.len() as u64);
    let b0 = emu.maps.read_byte(next).unwrap_or(0);
    let b1 = emu.maps.read_byte(next.wrapping_add(1)).unwrap_or(0);
    let next_is_null_check = (b0 == 0x48 && b1 == 0x85)  // test r64,r64
        || b0 == 0x85                                      // test r32,r32
        || (b0 == 0x48 && b1 == 0x83); // cmp r64,imm8
    if next_is_null_check {
        return;
    }

    let _ = emu.maps.write_qword(rsi, rsi);
    let _ = emu.maps.write_qword(rsi + 8, rsi);
}

fn is_current_process_handle(h: u64) -> bool {
    h == !0 || h == 0xffff_ffff_ffff_fffe
}

/// `NtQueryVirtualMemory` — x64 register/stack layout matches the ntdll syscall stub
/// (RCX..R9 + 5th/6th on stack at `rsp+0x28` / `rsp+0x30`).
pub fn nt_query_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_address = emu.regs().rdx;
    let memory_information_class = emu.regs().r8;
    let memory_information = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let memory_information_length = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let return_length_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryVirtualMemory process: 0x{:x}, base: 0x{:x}, class: 0x{:x}, out: 0x{:x}, len: 0x{:x}, ret_len_ptr: 0x{:x}",
        WIN64_NTQUERYVIRTUALMEMORY,
        process_handle,
        base_address,
        memory_information_class,
        memory_information,
        memory_information_length,
        return_length_ptr
    );

    if memory_information == 0 || !emu.maps.is_mapped(memory_information) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    // MemoryImageInformation (class 6) and MemoryImageExtensionInformation
    // (class 0xe, newer): both return a 0x18-byte image descriptor
    // {ImageBase: u64, SizeOfImage: u64, ImageFlags: u32}. Win11's
    // LdrInitializeThunk queries class 0xe for ntdll's own image; returning the
    // image base/size (flags zeroed = no special extensions) keeps init going,
    // whereas the generic fallback below would answer STATUS_INVALID_PARAMETER
    // and ntdll aborts the process with a hard error.
    if memory_information_class == MEMORY_INFORMATION_CLASS_MEMORY_IMAGE_INFORMATION
        || memory_information_class == MEMORY_INFORMATION_CLASS_MEMORY_IMAGE_EXTENSION_INFORMATION
    {
        const MEMORY_IMAGE_INFO_SIZE: u64 = 0x18;
        if memory_information_length < MEMORY_IMAGE_INFO_SIZE {
            emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
            return;
        }

        match emu.maps.find_pe_image_info(base_address) {
            Some((image_base, size_of_image)) => {
                emu.maps.write_qword(memory_information, image_base);
                emu.maps.write_qword(memory_information + 8, size_of_image);
                emu.maps.write_dword(memory_information + 16, 0);
                if return_length_ptr != 0 {
                    emu.maps
                        .write_qword(return_length_ptr, MEMORY_IMAGE_INFO_SIZE);
                }
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
            None => {
                emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            }
        }
        return;
    }

    if memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_BASIC_INFORMATION
        && memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_PRIVILEGED_BASIC_INFORMATION
    {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if memory_information_length < MemoryBasicInformation64::SIZE {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let mem_info = if !emu.maps.is_mapped(base_address) {
        // Real Windows returns MEM_FREE for unallocated pages, not an error.
        // ntdll's heap-walk loop uses this to discover free virtual address ranges;
        // returning STATUS_INVALID_PARAMETER causes it to exit the loop early and
        // skip heap initialization entirely.
        let page_base = base_address & !0xFFF;
        let region_size = emu
            .maps
            .next_mapped_addr(page_base)
            .map(|next| next.saturating_sub(page_base))
            .unwrap_or(0x1000);
        MemoryBasicInformation64 {
            base_address: page_base,
            allocation_base: 0,
            allocation_protect: 0,
            partition_id: 0,
            reserved: 0,
            region_size,
            state: MEM_FREE,
            protect: PAGE_NOACCESS,
            typ: 0,
        }
    } else {
        let base = emu.maps.get_addr_base(base_address).unwrap_or(0);
        let region_size = emu
            .maps
            .get_mem_by_addr(base_address)
            .map(|m| m.size() as u64)
            .unwrap_or(0);
        let protect = emu
            .maps
            .get_mem_by_addr(base_address)
            .map(|m| permission_to_nt_page_protection(m.permission()))
            .unwrap_or(PAGE_READWRITE);
        MemoryBasicInformation64 {
            base_address: base,
            allocation_base: base,
            allocation_protect: protect,
            partition_id: 0,
            reserved: 0,
            region_size,
            state: MEM_COMMIT,
            protect,
            typ: MEM_PRIVATE,
        }
    };

    mem_info.save(memory_information, &mut emu.maps);

    if return_length_ptr != 0
        && !emu
            .maps
            .write_qword(return_length_ptr, MemoryBasicInformation64::SIZE)
        {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAllocateVirtualMemory` — x64: RCX..R9 + 5th/6th at `[rsp+0x28]` / `[rsp+0x30]`.
pub fn nt_allocate_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let _zero_bits = emu.regs().r8;
    let region_sz_ptr = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let alloc_type = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let protect = emu.maps.read_dword(rsp + 0x30).unwrap_or(0);

    let size_in = if region_sz_ptr != 0 && emu.maps.is_mapped(region_sz_ptr) {
        emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
    } else {
        0
    };
    let pref_in = if base_ptr != 0 && emu.maps.is_mapped(base_ptr) {
        emu.maps.read_qword(base_ptr).unwrap_or(0)
    } else {
        0
    };
    log_orange!(
        emu,
        "syscall 0x{:x}: NtAllocateVirtualMemory rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *Base(in): 0x{:x} r9/region_sz_ptr: 0x{:x} *Size(in): 0x{:x} [rsp+28]/type: 0x{:x} [rsp+30]/prot: 0x{:x}",
        WIN64_NTALLOCATEVIRTUALMEMORY,
        process_handle,
        base_ptr,
        pref_in,
        region_sz_ptr,
        size_in,
        alloc_type,
        protect
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 || region_sz_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let mut size = match emu.maps.read_qword(region_sz_ptr) {
        Some(s) => s,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    size = align_up_page(size);
    size = clamp_backing_size(emu, size);

    let preferred_base = match emu.maps.read_qword(base_ptr) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    let mem_commit = (alloc_type & MEM_COMMIT) != 0;
    let mem_reserve = (alloc_type & MEM_RESERVE) != 0;
    if !mem_commit && !mem_reserve {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let permission = nt_page_protection_to_permission(protect);

    // Windows enforces a 64K (`dwAllocationGranularity`) base alignment for any
    // RESERVE allocation chosen by the kernel. ntdll's heap segment algorithm
    // relies on this — it computes the segment header address as
    // `chunk_addr & ~0xFFFF`, so a 4K-aligned reservation base produces an
    // unmapped read at segment_base + 0x60. Only force this on RESERVE >= 64K;
    // small COMMIT-only allocations (loader scratch buffers under MinGW init)
    // keep 4K granularity to avoid disturbing the existing layout.
    // Windows enforces a 64K (`dwAllocationGranularity`) base alignment for any
    // RESERVE allocation chosen by the kernel. ntdll's heap segment algorithm
    // relies on this — it computes the segment header address as
    // `chunk_addr & ~0xFFFF`, so a 4K-aligned reservation base produces an
    // unmapped read at segment_base + 0x60. Only force this on RESERVE >= 64K
    // (the size threshold avoids disturbing small loader scratch buffers).
    let base = if preferred_base == 0 {
        let allocation = if mem_reserve && size >= 0x10000 {
            alloc_64k_aligned(emu, size)
        } else {
            emu.maps.alloc(size)
        };
        match allocation {
            Some(a) => a,
            None => {
                emu.regs_mut().rax = STATUS_NO_MEMORY;
                return;
            }
        }
    } else {
        preferred_base
    };

    if !emu.maps.write_qword(base_ptr, base) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let name = format!("alloc_{:x}", base);
    match emu.maps.create_map(&name, base, size, permission) {
        Ok(_) => {}
        Err(_) => {
            if !emu.maps.is_mapped(base) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        }
    }

    patch_ldr_heap_list_sentinel(emu, base, size);

    // Kernel writes back the actual (page-rounded) region size; ntdll uses *RegionSize after return.
    if !emu.maps.write_qword(region_sz_ptr, size) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// Allocate a free address range of at least `size` bytes whose base is 64K
/// aligned (matches Windows `dwAllocationGranularity`). Probes our generic
/// 4K-aligned allocator with growing offsets until we land on a free 64K
/// boundary that fits.
fn alloc_64k_aligned(emu: &mut Emu, size: u64) -> Option<u64> {
    const GRAN: u64 = 0x10000;
    // First try: ask for size + GRAN-1 so we can slide forward to a boundary.
    let probe = emu.maps.alloc(size + GRAN - 1)?;
    let aligned = (probe + GRAN - 1) & !(GRAN - 1);
    // Validate the aligned range is still free (the probe gave us [probe, probe+size+GRAN-1)
    // — sliding to `aligned` keeps us inside that window).
    if aligned + size < probe + size + GRAN && !emu.maps.overlaps(aligned, size) {
        Some(aligned)
    } else {
        // Fallback: scan for a free 64K-aligned region directly.
        let mut candidate = (probe + GRAN - 1) & !(GRAN - 1);
        for _ in 0..64 {
            if !emu.maps.overlaps(candidate, size) {
                return Some(candidate);
            }
            candidate += GRAN;
        }
        None
    }
}

/// `NtAllocateVirtualMemoryEx` — extended version with MEM_EXTENDED_PARAMETER support.
/// Parameters: RCX ProcessHandle, RDX *BaseAddress, R8 *RegionSize, R9 AllocationType,
/// [RSP+28h] Protect, [RSP+30h] ExtendedParameters, [RSP+38h] ExtendedParameterCount.
pub fn nt_allocate_virtual_memory_ex(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let alloc_type = emu.regs().r9 as u32;
    let rsp = emu.regs().rsp;
    let protect = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let ext_count = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    let size_in = if region_sz_ptr != 0 && emu.maps.is_mapped(region_sz_ptr) {
        emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
    } else {
        0
    };
    let pref_in = if base_ptr != 0 && emu.maps.is_mapped(base_ptr) {
        emu.maps.read_qword(base_ptr).unwrap_or(0)
    } else {
        0
    };
    log_orange!(
        emu,
        "syscall 0x{:x}: NtAllocateVirtualMemoryEx rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *Base(in): 0x{:x} r8/region_sz_ptr: 0x{:x} *Size(in): 0x{:x} r9/type: 0x{:x} [rsp+28]/prot: 0x{:x} [rsp+38]/ext_count: 0x{:x}",
        WIN64_NTALLOCATEVIRTUALMEMORYEX,
        process_handle,
        base_ptr,
        pref_in,
        region_sz_ptr,
        size_in,
        alloc_type,
        protect,
        ext_count
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 || region_sz_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let mut size = match emu.maps.read_qword(region_sz_ptr) {
        Some(s) => s,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    size = align_up_page(size);
    size = clamp_backing_size(emu, size);

    let preferred_base = match emu.maps.read_qword(base_ptr) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    let mem_commit = (alloc_type & MEM_COMMIT) != 0;
    let mem_reserve = (alloc_type & MEM_RESERVE) != 0;
    if !mem_commit && !mem_reserve {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let permission = nt_page_protection_to_permission(protect);

    let base = if preferred_base == 0 {
        let allocation = if mem_reserve && size >= 0x10000 {
            alloc_64k_aligned(emu, size)
        } else {
            emu.maps.alloc(size)
        };
        match allocation {
            Some(a) => a,
            None => {
                emu.regs_mut().rax = STATUS_NO_MEMORY;
                return;
            }
        }
    } else {
        preferred_base
    };

    if !emu.maps.write_qword(base_ptr, base) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let name = format!("alloc_{:x}", base);
    match emu.maps.create_map(&name, base, size, permission) {
        Ok(_) => {}
        Err(_) => {
            if !emu.maps.is_mapped(base) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        }
    }

    patch_ldr_heap_list_sentinel(emu, base, size);

    if !emu.maps.write_qword(region_sz_ptr, size) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtFreeVirtualMemory` — x64: RCX, RDX, R8, R9 (all in registers).
pub fn nt_free_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let free_type = emu.regs().r9 as u32;

    let base_disp: String = if base_ptr == 0 {
        "—".to_string()
    } else if emu.maps.is_mapped(base_ptr) {
        format!("0x{:x}", emu.maps.read_qword(base_ptr).unwrap_or(0))
    } else {
        "? (unmapped rdx)".to_string()
    };
    let region_size_disp: String = if region_sz_ptr == 0 {
        "— (r8=0)".to_string()
    } else if emu.maps.is_mapped(region_sz_ptr) {
        format!("0x{:x}", emu.maps.read_qword(region_sz_ptr).unwrap_or(0))
    } else {
        "? (unmapped r8)".to_string()
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtFreeVirtualMemory rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *BaseAddress: {} r8/region_sz_ptr: 0x{:x} *RegionSize: {} r9/free_type: 0x{:x}",
        WIN64_NTFREEVIRTUALMEMORY,
        process_handle,
        base_ptr,
        base_disp,
        region_sz_ptr,
        region_size_disp,
        free_type
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = match emu.maps.read_qword(base_ptr) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if (free_type & MEM_RELEASE) != 0 && (free_type & MEM_DECOMMIT) != 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if (free_type & MEM_RELEASE) != 0 {
        // `Maps::dealloc` keys regions by allocation base. ntdll passes interior pointers,
        // exclusive ends (`addr == map.bottom`), etc.; use `alloc_region_base_for_free`.
        let alloc_base = emu.maps.alloc_region_base_for_free(base);
        let base_mapped = emu.maps.is_mapped(base);

        // SSDT-only: ntdll's heap segment manager calls `MEM_RELEASE` with a
        // non-zero RegionSize that targets only a sub-range of the original
        // reservation (real Windows requires size==0 for MEM_RELEASE). Our
        // single-map model can't shrink a reservation, so destroying the whole
        // map orphans subsequent COMMITs as scattered 4K maps — an 8-byte qword
        // write at the last few bytes of one page then straddles into the next
        // and hits "Writing qword to unmapped or non-writable region". When in
        // SSDT mode and the request looks partial, skip the dealloc; the
        // reservation stays and future commits in the same range fall through
        // with `is_mapped == true`, leaving ntdll's heap with one contiguous
        // backing map.
        let region_size = if region_sz_ptr != 0 {
            emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
        } else {
            0
        };
        let map_for_base = if base_mapped {
            emu.maps
                .get_mem_by_addr(base)
                .map(|m| (m.get_base(), m.size() as u64))
        } else if let Some(ab) = alloc_base {
            emu.maps
                .get_mem_by_addr(ab)
                .map(|m| (m.get_base(), m.size() as u64))
        } else {
            None
        };
        let is_partial_release = emu.cfg.emulate_winapi
            && region_size != 0
            && match map_for_base {
                Some((mb, msz)) => region_size < msz || base != mb,
                None => false,
            };

        let released = if is_partial_release {
            // Keep the reservation intact; report success.
            true
        } else if base_mapped {
            emu.maps.dealloc(base);
            true
        } else if let Some(ab) = alloc_base {
            emu.maps.dealloc(ab);
            true
        } else {
            false
        };
        if !released {
            // ntdll may call MEM_RELEASE on a range already torn down by a prior successful free or
            // on an address our single-map model treats as unmapped; real kernel often accepts the
            // no-op.
            if emu.cfg.emulate_winapi && (ALLOC64_MIN..ALLOC64_MAX).contains(&base) {
                // Windows writes back the page-aligned freed base (not zero) so callers can
                // inspect what was released. Do not zero *BaseAddress.
                emu.regs_mut().rax = STATUS_SUCCESS;
                return;
            }
            emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            return;
        }
        // Windows writes back the page-aligned freed base address and zeroes RegionSize.
        // Do NOT write 0 to *BaseAddress — ntdll reads it after MEM_RELEASE to derive the
        // effective heap start (freed_base + trim_amount). Leave *BaseAddress unchanged so the
        // caller still sees the address that was freed.
        if region_sz_ptr != 0 {
            let _ = emu.maps.write_qword(region_sz_ptr, 0);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if (free_type & MEM_DECOMMIT) != 0 {
        // ZwFreeVirtualMemory: with MEM_DECOMMIT, a NULL `RegionSize` (or *RegionSize == 0) means
        // decommit the whole allocation that contains `BaseAddress`. Our old stub treated that as
        // sz==0 → STATUS_INVALID_PARAMETER, which broke ntdll heap/Ldr scratch frees (then
        // STATUS_APP_INIT_FAILURE / terminate with 0xC000000d).
        let mut sz = if region_sz_ptr != 0 {
            emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
        } else {
            0
        };
        if !emu.maps.is_mapped(base) {
            emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            return;
        }
        if region_sz_ptr == 0 || sz == 0 {
            sz = emu
                .maps
                .get_mem_by_addr(base)
                .map(|m| {
                    let mb = m.get_base();
                    let top = mb.saturating_add(m.size() as u64);
                    top.saturating_sub(base)
                })
                .unwrap_or(0);
        }
        if sz == 0 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        let cap = sz.min(0x1000_0000) as usize;
        emu.maps.memset(base, 0, cap);
        if region_sz_ptr != 0 {
            let _ = emu.maps.write_qword(region_sz_ptr, cap as u64);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtProtectVirtualMemory` — x64: RCX..R9 + 5th (`OldProtect`) at `[rsp+0x28]`.
pub fn nt_protect_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let new_protect = emu.regs().r9 as u32;
    let rsp = emu.regs().rsp;
    let old_protect_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtProtectVirtualMemory h: 0x{:x} base_ptr: 0x{:x} new_prot: 0x{:x}",
        WIN64_NTPROTECTVIRTUALMEMORY,
        process_handle,
        base_ptr,
        new_protect
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 || region_sz_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.read_qword(base_ptr).unwrap_or(0);
    let _region_sz = emu.maps.read_qword(region_sz_ptr).unwrap_or(0);

    // Read the previous protection BEFORE changing it.
    let old_protect = if let Some(mem) = emu.maps.get_mem_by_addr(base) {
        permission_to_nt_page_protection(mem.permission())
    } else {
        0x04 // PAGE_READWRITE fallback
    };

    if old_protect_ptr != 0 {
        let _ = emu.maps.write_dword(old_protect_ptr, old_protect);
    }

    if let Some(mem) = emu.maps.get_mem_by_addr_mut(base) {
        let mut new_perm = nt_page_protection_to_permission(new_protect);
        // Preserve write permission on `.didat` sections. The loader maps them
        // RW (to apply delay-load patches via SEC_IMAGE COW we do not model)
        // and then calls NtProtectVirtualMemory(PAGE_READONLY) to mirror the
        // final state, but later code paths still write to those pages.
        if mem.get_name().ends_with(".didat") {
            new_perm = new_perm.add(Permission::WRITE);
        }
        mem.set_permission(new_perm);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtReadVirtualMemory` — x64: 5th arg (`NumberOfBytesRead`) at `[rsp+0x28]`.
pub fn nt_read_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let bytes_read_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtReadVirtualMemory h: 0x{:x} from: 0x{:x} to: 0x{:x} len: 0x{:x}",
        WIN64_NTREADVIRTUALMEMORY,
        process_handle,
        base,
        buffer,
        size
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if buffer == 0 || size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let sz = size.min(usize::MAX as u64) as usize;
    let data = match emu.maps.try_read_bytes(base, sz) {
        Some(s) => s.to_vec(),
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if !emu.maps.write_bytes(buffer, &data) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if bytes_read_ptr != 0 {
        let _ = emu.maps.write_qword(bytes_read_ptr, data.len() as u64);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtWriteVirtualMemory` — x64: 5th arg (`NumberOfBytesWritten`) at `[rsp+0x28]`.
pub fn nt_write_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let bytes_written_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtWriteVirtualMemory h: 0x{:x} to: 0x{:x} from: 0x{:x} len: 0x{:x}",
        WIN64_NTWRITEVIRTUALMEMORY,
        process_handle,
        base,
        buffer,
        size
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if buffer == 0 || size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let sz = size.min(usize::MAX as u64) as usize;
    let data = match emu.maps.try_read_bytes(buffer, sz) {
        Some(s) => s.to_vec(),
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if !emu.maps.write_bytes(base, &data) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if bytes_written_ptr != 0 {
        let _ = emu.maps.write_qword(bytes_written_ptr, data.len() as u64);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtUnmapViewOfSection` — no-op stub: returns SUCCESS without actually
/// destroying the mapping. ntdll calls this on temp views during PE
/// header probing and would re-map at a chosen base; collapsing it to a
/// no-op keeps the original mapping intact so subsequent reads of the PE
/// still succeed.
pub fn nt_unmap_view_of_section(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;

    // Resolve which map (if any) covers the base so the trace shows whether
    // the caller is unmapping one of our DLL views.
    let map_name = emu
        .maps
        .get_addr_name(base)
        .unwrap_or("<unmapped>")
        .to_string();
    log_orange!(
        emu,
        "syscall 0x{:x}: NtUnmapViewOfSection h: 0x{:x} base: 0x{:x} [{}] (stub → SUCCESS, keeping map intact)",
        WIN64_NTUNMAPVIEWOFSECTION,
        process_handle,
        base,
        map_name,
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtMapViewOfSection` — x64 10-arg syscall.
/// RCX=SectionHandle, RDX=ProcessHandle, R8=*BaseAddress (in/out),
/// R9=ZeroBits, [rsp+0x28]=CommitSize, [rsp+0x30]=SectionOffset,
/// [rsp+0x38]=*ViewSize, [rsp+0x40]=InheritDisposition,
/// [rsp+0x48]=AllocationType, [rsp+0x50]=Win32Protect.
///
/// For KnownDll section handles (tracked via NtOpenSection), loads the real
/// PE from the maps folder so ntdll's loader gets valid image content.
/// Otherwise allocates a fresh anonymous region.
pub fn nt_map_view_of_section(emu: &mut Emu) {
    let section_handle = emu.regs().rcx;
    let process_handle = emu.regs().rdx;
    let base_addr_ptr = emu.regs().r8;
    let rsp = emu.regs().rsp;
    let view_size_ptr = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);
    let protect = emu.maps.read_dword(rsp + 0x50).unwrap_or(4); // PAGE_READWRITE default

    // Read requested base (0 = any) and requested size.
    let requested_base = if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
        emu.maps.read_qword(base_addr_ptr).unwrap_or(0)
    } else {
        0
    };
    let view_size = if view_size_ptr != 0 && emu.maps.is_mapped(view_size_ptr) {
        emu.maps.read_qword(view_size_ptr).unwrap_or(0x1000)
    } else {
        0x1000
    };
    let size = if view_size == 0 {
        0x1000
    } else {
        (view_size + 0xfff) & !0xfff
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtMapViewOfSection base_ptr: 0x{:x} req_base: 0x{:x} size: 0x{:x} prot: 0x{:x}",
        WIN64_NTMAPVIEWOFSECTION,
        base_addr_ptr,
        requested_base,
        size,
        protect,
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    // If this handle corresponds to a KnownDll section (tracked by NtOpenSection),
    // load the real PE content so ntdll's loader gets valid image data.
    if let Some(dll_name) = emu.section_handles.get(&section_handle).cloned() {
        let dll_base = crate::api::windows::winapi64::kernel32::load_library(emu, &dll_name);
        if dll_base != 0 {
            log::trace!(
                "NtMapViewOfSection: KnownDll {} loaded at 0x{:x}",
                dll_name,
                dll_base
            );
            // Write the real PE base back to *BaseAddress.
            if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
                let _ = emu.maps.write_qword(base_addr_ptr, dll_base);
            }
            // Report SizeOfImage as the view size (PE opt header +0x50 on PE64).
            let size_of_image: u64 = {
                let pe_off = emu.maps.read_dword(dll_base + 0x3c).unwrap_or(0) as u64;
                if pe_off > 0 {
                    emu.maps
                        .read_dword(dll_base + pe_off + 0x50)
                        .unwrap_or(0x1000) as u64
                } else {
                    0x1000
                }
            };
            if view_size_ptr != 0 && emu.maps.is_mapped(view_size_ptr) {
                let _ = emu.maps.write_qword(view_size_ptr, size_of_image);
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
            return;
        }
    }

    let perm = nt_page_protection_to_permission(protect);

    // Use the requested base if it looks valid and is not already mapped,
    // otherwise let the allocator pick an address.
    let mapped_base = if requested_base >= 0x10000 && !emu.maps.is_mapped(requested_base) {
        // Try to create the map at the exact requested address.
        let name = format!("section_view_{:x}", requested_base);
        match emu.maps.create_map(&name, requested_base, size, perm) {
            Ok(_) => requested_base,
            Err(_) => {
                // Fall back to a lib64 allocation.
                let base = emu.maps.lib64_alloc(size).unwrap_or(0);
                if base != 0 {
                    let name2 = format!("section_view_{:x}", base);
                    let _ = emu.maps.create_map(&name2, base, size, perm);
                }
                base
            }
        }
    } else {
        let base = emu.maps.lib64_alloc(size).unwrap_or(0);
        if base != 0 {
            let name = format!("section_view_{:x}", base);
            emu.maps.create_map(&name, base, size, perm);
        }
        base
    };

    if mapped_base == 0 {
        emu.regs_mut().rax = STATUS_NO_MEMORY;
        return;
    }

    // Write back the mapped base address and actual size to caller.
    if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
        let _ = emu.maps.write_qword(base_addr_ptr, mapped_base);
    }
    if view_size_ptr != 0 && emu.maps.is_mapped(view_size_ptr) {
        let _ = emu.maps.write_qword(view_size_ptr, size);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAllocateUserPhysicalPagesEx` — syscall 0x76.
/// x64: RCX=`ProcessHandle`, RDX=`NumberOfPages` (PULONG_PTR in/out),
///      R8=`UserPfnArray` (PULONG_PTR), R9=`ExtendedParameters`,
///      `[rsp+0x28]`=`ExtendedParameterCount` (ULONG).
///
/// AWE physical-page allocation requires SE_LOCK_MEMORY_PRIVILEGE.
/// Return STATUS_PRIVILEGE_NOT_HELD so ntdll falls back to non-AWE heap.
pub fn nt_allocate_user_physical_pages_ex(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let num_pages_ptr = emu.regs().rdx;
    let _pfn_array = emu.regs().r8;
    let num_pages = emu.maps.read_qword(num_pages_ptr).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtAllocateUserPhysicalPagesEx h: 0x{:x} num_pages: {}",
        WIN64_NTALLOCATEUSERPHYSICALPAGESEX,
        process_handle,
        num_pages
    );

    // Return SUCCESS with 0 pages allocated (physical memory unavailable in emulator).
    // Writing 0 back to *NumberOfPages tells ntdll no pages were actually allocated,
    // so the segment heap falls back to non-AWE mode without treating it as fatal.
    if num_pages_ptr != 0 && emu.maps.is_mapped(num_pages_ptr) {
        let _ = emu.maps.write_qword(num_pages_ptr, 0);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtCreateSection` — syscall 0x4a.
/// RCX=SectionHandle (out), RDX=DesiredAccess, R8=ObjectAttributes,
/// R9=MaximumSize (PLARGE_INTEGER), [rsp+0x28]=SectionPageProtection,
/// [rsp+0x30]=AllocationAttributes, [rsp+0x38]=FileHandle.
///
/// Returns a fake handle. Section objects are used by the loader to map
/// DLL images; we don't need real semantics since mapping is handled
/// by NtMapViewOfSection stubs.
/// `NtOpenSection` — open a handle to an existing named section object.
///
/// x64: RCX=SectionHandle(out), RDX=DesiredAccess, R8=ObjectAttributes.
/// Writes a fake handle and returns STATUS_SUCCESS.
/// If the section name is a KnownDll path (e.g. `\KnownDlls\kernel32.dll`),
/// store `handle → dll_name` so NtMapViewOfSection can load real PE content.
pub fn nt_open_section(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let obj_attr = emu.regs().r8;

    // Parse ObjectAttributes → UNICODE_STRING → section name.
    let section_name = read_object_attributes_name(emu, obj_attr);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenSection handle_out: 0x{:x}, access: 0x{:x}, obj_attr: 0x{:x} name: {:?}",
        WIN64_NTOPENSECTION,
        handle_out,
        desired_access,
        obj_attr,
        section_name,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    // The loader can address KnownDll sections two ways:
    //   1. Absolute path:  ObjectName = `\KnownDlls\kernel32.dll`, RootDirectory = NULL.
    //   2. Relative open:  ObjectName = `kernel32.dll`, RootDirectory = handle from
    //      a previous `NtOpenDirectoryObject("\KnownDlls")`. ntdll's LdrpFindKnownDll
    //      uses this form, so we must resolve it too.
    let root_dir = read_object_attributes_root_directory(emu, obj_attr);
    let is_known_dll_dir = root_dir != 0 && emu.known_dll_dir_handles.contains(&root_dir);
    let dll_name = if is_known_dll_dir {
        if !section_name.is_empty() {
            Some(section_name.to_lowercase())
        } else {
            None
        }
    } else {
        extract_known_dll_name(&section_name)
    };

    // KnownDlls open with an empty/invalid section name. These are produced
    // when ntdll's RtlAnsiStringToUnicodeString / ApiSet resolver fails to
    // populate a destination buffer (yields `[zeros..., '.', 'D', 'L', 'L']`)
    // — typically for `ext-ms-*` dependencies of kernelbase that real Windows
    // redirects via the API-set schema. Returning STATUS_OBJECT_NAME_NOT_FOUND
    // here causes ntdll to fall through to disk lookup, which also fails, and
    // terminate the process with STATUS_DLL_NOT_FOUND.
    //
    // The former kernelbase fallback is disabled: an unbounded fake fallback
    // causes ntdll to re-init phantom modules (TLS callbacks, DllMain) in a
    // tight loop and exhaust the stack.
    if is_known_dll_dir && dll_name.is_none() {
        emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
        return;
    }

    // For KnownDll opens, only grant a handle if we actually have the file.
    // Otherwise the subsequent NtMapViewOfSection produces an empty section
    // and ntdll's LdrLoadDll bails with STATUS_INVALID_IMAGE_FORMAT.
    // Returning OBJECT_NAME_NOT_FOUND here lets the loader fall through to
    // the disk-file path, which fails cleanly with STATUS_DLL_NOT_FOUND.
    if let Some(ref name) = dll_name {
        let path = format!("{}{}", emu.cfg.maps_folder, name);
        if !std::path::Path::new(&path).exists() {
            log::trace!(
                "NtOpenSection: KnownDll {} not in maps_folder → OBJECT_NAME_NOT_FOUND",
                name
            );
            emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            return;
        }
    }

    let h = crate::syscall::windows::syscall64::sync::next_handle();
    let _ = emu.maps.write_qword(handle_out, h);

    if let Some(dll_name) = dll_name {
        log::trace!(
            "NtOpenSection: tracking KnownDll handle 0x{:x} -> {}",
            h,
            dll_name
        );
        emu.section_handles.insert(h, dll_name);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// Extract the DLL filename from a KnownDlls section path, e.g.:
/// `\KnownDlls\kernel32.dll`  → `kernel32.dll`
/// `\KnownDlls32\kernel32.dll` → `kernel32.dll`
/// Returns `None` if the path is not a KnownDlls path.
fn extract_known_dll_name(path: &str) -> Option<String> {
    // Case-insensitive match for \KnownDlls\ prefix.
    let lower = path.to_lowercase();
    let prefix = if lower.starts_with("\\knowndlls32\\") {
        "\\knowndlls32\\"
    } else if lower.starts_with("\\knowndlls\\") {
        "\\knowndlls\\"
    } else {
        return None;
    };
    let rest = &path[prefix.len()..];
    if rest.is_empty() {
        return None;
    }
    Some(rest.to_lowercase())
}

fn read_unicode_string(emu: &Emu, addr: u64) -> String {
    if addr == 0 || !emu.maps.is_mapped(addr) {
        return String::new();
    }
    // UNICODE_STRING { USHORT Length; USHORT MaximumLength; PWSTR Buffer; }.
    // `Length` is the size in BYTES of the string and is authoritative — the
    // buffer is NOT required to be null-terminated. Reading to a NUL (the old
    // behaviour) overran non-terminated strings into adjacent heap fill bytes
    // (0xABAB guard / 0xFEEE freed), appending garbage that broke name matches.
    let len = emu.maps.read_word(addr).unwrap_or(0);
    let buf = emu.maps.read_qword(addr + 8).unwrap_or(0);
    if buf == 0 || !emu.maps.is_mapped(buf) {
        return String::new();
    }
    emu.maps.read_wide_string_n(buf, (len / 2) as usize)
}

pub fn read_object_attributes_name(emu: &Emu, addr: u64) -> String {
    if addr == 0 || !emu.maps.is_mapped(addr) {
        return String::new();
    }
    // OBJECT_ATTRIBUTES64: Length(4)+pad(4)+RootDirectory(8)+ObjectName*(8)
    let object_name_ptr = emu.maps.read_qword(addr + 0x10).unwrap_or(0);
    read_unicode_string(emu, object_name_ptr)
}

pub(crate) fn read_object_attributes_root_directory(emu: &Emu, addr: u64) -> u64 {
    if addr == 0 || !emu.maps.is_mapped(addr) {
        return 0;
    }
    emu.maps.read_qword(addr + 0x08).unwrap_or(0)
}

pub fn nt_create_section(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;
    let max_size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let page_protection = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let alloc_attributes = emu.maps.read_dword(rsp + 0x30).unwrap_or(0);
    let file_handle = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateSection handle_out: 0x{:x}, access: 0x{:x}, max_size: 0x{:x}, prot: 0x{:x}, alloc: 0x{:x}, file: 0x{:x}",
        WIN64_NTCREATESECTION,
        handle_out,
        desired_access,
        max_size,
        page_protection,
        alloc_attributes,
        file_handle,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = crate::syscall::windows::syscall64::sync::next_handle();
    // Inherit the DLL backing from the file handle if we tracked one, so the
    // subsequent NtMapViewOfSection on this section loads the real PE.
    if file_handle != 0 {
        if let Some(dll_name) = emu.file_handles.get(&file_handle).cloned() {
            emu.section_handles.insert(h, dll_name);
        }
    }
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
