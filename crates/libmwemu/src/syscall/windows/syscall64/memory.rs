use crate::api::windows::common::memory as memory_api;
use crate::api::windows::common::memory::Call;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::windows::syscall64::sync::next_handle;
use crate::windows::loader64::load_library;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::syscalls::*;

use crate::windows::structures::MemoryBasicInformation64;
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

const STATUS_INVALID_IMAGE_FORMAT: u64 = 0xc000007b;

pub fn patch_ntdll_load_reason_checks(emu: &mut Emu) {
    if emu.validated_knowndlls.contains("__ntdll_patched__") {
        return;
    }
    emu.validated_knowndlls
        .insert("__ntdll_patched__".to_string());

    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let base = ntdll_map.get_base();

    // Patch `cmp [entry+10Ch], 9` immediates so they compare with 0
    // instead of LoadReasonPatchImage (9). Only JE sites: LoadReason=0
    // would fall through into "PatchImage" handling that sets
    // STATUS_DLL_INIT_FAILED. JNE sites correctly call the loading
    // function when LoadReason != 9 — the TpPostWork bypass below
    // keeps that path from spinning on the thread pool.
    // Excluded: 0x4f49 (jne), 0x1655f (jne_near), 0x740e4 (not a jcc)
    let rvas: &[u64] = &[
        0x4e3d, 0x4f57, 0x506e, 0x59da, 0x96ba, 0x127dd, 0x176d8, 0x3313b, 0x4c172,
    ];
    let mut count = 0u32;
    for &rva in rvas {
        let addr = base + rva;
        if emu.maps.is_mapped(addr) && emu.maps.read_byte(addr).unwrap_or(0) == 0x09 {
            emu.maps.write_byte(addr, 0x00);
            count += 1;
        }
    }
    if count > 0 {
        log::trace!(
            "NtMapViewOfSection: patched {} ntdll LoadReason checks (9→0)",
            count,
        );
    }

    // Bypass TpPostWork call in the parallel-loader submit path.
    // At RVA 0x6ECA1 there is `je short +0x1F` (0x74 0x1F) that skips
    // TpPostWork when the work-queue pointer is NULL. Change je→jmp so
    // the call is always skipped — we have no worker threads.
    let tp_bypass = base + 0x6ECA1;
    if emu.maps.is_mapped(tp_bypass) && emu.maps.read_byte(tp_bypass).unwrap_or(0) == 0x74 {
        emu.maps.write_byte(tp_bypass, 0xEB);
        log::trace!("ntdll: patched TpPostWork bypass at RVA 0x6ECA1 (je→jmp)");
    }
}

/// Read SizeOfImage from a PE file's optional header.
fn pe_size_of_image(raw: &[u8]) -> Option<u64> {
    if raw.len() < 0x40 {
        return None;
    }
    let pe_off = u32::from_le_bytes(raw[0x3c..0x40].try_into().ok()?) as usize;
    let magic_off = pe_off + 0x18;
    if raw.len() < magic_off + 2 {
        return None;
    }
    let magic = u16::from_le_bytes(raw[magic_off..magic_off + 2].try_into().ok()?);
    let soi_off = match magic {
        0x20b => pe_off + 0x50, // PE32+
        0x10b => pe_off + 0x50, // PE32
        _ => return None,
    };
    if raw.len() < soi_off + 4 {
        return None;
    }
    Some(u32::from_le_bytes(raw[soi_off..soi_off + 4].try_into().ok()?) as u64)
}

/// Map a PE file as a section_view: allocate a region of SizeOfImage bytes,
/// copy headers and each section at its VirtualAddress offset. Returns the
/// base address of the new view, or None on failure.
fn map_pe_as_section_view(
    emu: &mut Emu,
    dll_name: &str,
    raw: &[u8],
    requested_base: u64,
) -> Option<u64> {
    let size_of_image = pe_size_of_image(raw)?;
    let aligned = (size_of_image + 0xfff) & !0xfff;

    let base = if requested_base >= 0x10000 && !emu.maps.is_mapped(requested_base) {
        let name = format!("section_view_{:x}", requested_base);
        match emu
            .maps
            .create_map(&name, requested_base, aligned, Permission::READ_WRITE)
        {
            Ok(_) => requested_base,
            Err(_) => {
                let b = emu.maps.lib64_alloc(aligned)?;
                let name2 = format!("section_view_{:x}", b);
                emu.maps
                    .create_map(&name2, b, aligned, Permission::READ_WRITE)
                    .ok()?;
                b
            }
        }
    } else {
        let b = emu.maps.lib64_alloc(aligned)?;
        let name = format!("section_view_{:x}", b);
        emu.maps
            .create_map(&name, b, aligned, Permission::READ_WRITE)
            .ok()?;
        b
    };

    // Copy PE headers.
    let pe_off = u32::from_le_bytes(raw.get(0x3c..0x40)?.try_into().ok()?) as usize;
    let magic_off = pe_off + 0x18;
    let magic = u16::from_le_bytes(raw.get(magic_off..magic_off + 2)?.try_into().ok()?);
    let (opt_hdr_size, section_table_off) = match magic {
        0x20b => (0xf0usize, pe_off + 0x18 + 0xf0), // PE32+ optional header = 240 bytes
        0x10b => (0xe0usize, pe_off + 0x18 + 0xe0), // PE32
        _ => return None,
    };
    let _ = opt_hdr_size;
    let size_of_headers_off = match magic {
        0x20b => pe_off + 0x54,
        0x10b => pe_off + 0x54,
        _ => return None,
    };
    let size_of_headers = u32::from_le_bytes(
        raw.get(size_of_headers_off..size_of_headers_off + 4)?
            .try_into()
            .ok()?,
    ) as usize;

    let hdr_len = size_of_headers.min(raw.len());
    for (i, &byte) in raw[..hdr_len].iter().enumerate() {
        emu.maps.write_byte(base + i as u64, byte);
    }

    // Copy sections at their virtual addresses.
    let num_sections_off = pe_off + 0x06;
    let num_sections = u16::from_le_bytes(
        raw.get(num_sections_off..num_sections_off + 2)?
            .try_into()
            .ok()?,
    ) as usize;
    for i in 0..num_sections {
        let entry = section_table_off + i * 40;
        if entry + 40 > raw.len() {
            break;
        }
        let virt_addr = u32::from_le_bytes(raw[entry + 12..entry + 16].try_into().ok()?) as u64;
        let raw_size = u32::from_le_bytes(raw[entry + 16..entry + 20].try_into().ok()?) as usize;
        let raw_ptr = u32::from_le_bytes(raw[entry + 20..entry + 24].try_into().ok()?) as usize;
        let copy_len = raw_size.min(raw.len().saturating_sub(raw_ptr));
        if copy_len > 0 && raw_ptr < raw.len() {
            for (j, &byte) in raw[raw_ptr..raw_ptr + copy_len].iter().enumerate() {
                emu.maps.write_byte(base + virt_addr + j as u64, byte);
            }
        }
    }

    Some(base)
}

fn apply_pe_relocations(emu: &mut Emu, base: u64, raw: &[u8]) {
    let pe_off = match raw.get(0x3c..0x40) {
        Some(b) => u32::from_le_bytes(b.try_into().unwrap_or([0; 4])) as usize,
        None => return,
    };
    let magic_off = pe_off + 0x18;
    let magic = raw
        .get(magic_off..magic_off + 2)
        .map(|b| u16::from_le_bytes(b.try_into().unwrap_or([0; 2])))
        .unwrap_or(0);
    if magic != 0x20b {
        return;
    }

    let image_base = raw
        .get(pe_off + 0x30..pe_off + 0x38)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])))
        .unwrap_or(0);
    let delta = base.wrapping_sub(image_base);
    if delta == 0 {
        return;
    }

    let reloc_rva = raw
        .get(pe_off + 0xb0..pe_off + 0xb4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap_or([0; 4])) as usize)
        .unwrap_or(0);
    let reloc_size = raw
        .get(pe_off + 0xb4..pe_off + 0xb8)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap_or([0; 4])) as usize)
        .unwrap_or(0);
    if reloc_rva == 0 || reloc_size == 0 {
        return;
    }

    let mut offset = 0usize;
    while offset + 8 <= reloc_size {
        let block_rva = emu
            .maps
            .read_dword(base + reloc_rva as u64 + offset as u64)
            .unwrap_or(0) as u64;
        let block_size = emu
            .maps
            .read_dword(base + reloc_rva as u64 + offset as u64 + 4)
            .unwrap_or(0) as usize;
        if block_size < 8 {
            break;
        }
        let num_entries = (block_size - 8) / 2;
        for i in 0..num_entries {
            let entry_off = offset + 8 + i * 2;
            let entry = emu
                .maps
                .read_word(base + reloc_rva as u64 + entry_off as u64)
                .unwrap_or(0);
            let reloc_type = (entry >> 12) & 0xF;
            let reloc_offset = (entry & 0xFFF) as u64;
            let addr = base + block_rva + reloc_offset;
            match reloc_type {
                3 => {
                    let val = emu.maps.read_dword(addr).unwrap_or(0) as u64;
                    emu.maps.write_dword(addr, val.wrapping_add(delta) as u32);
                }
                10 => {
                    let val = emu.maps.read_qword(addr).unwrap_or(0);
                    emu.maps.write_qword(addr, val.wrapping_add(delta));
                }
                0 => {}
                _ => {}
            }
        }
        offset += block_size;
    }

    emu.maps.write_qword(base + pe_off as u64 + 0x30, base);
}

fn permission_to_nt_page_protection(perm: Permission) -> u32 {
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

/// `NtQueryVirtualMemory` — x64 register/stack layout matches the ntdll syscall stub
/// (RCX..R9 + 5th/6th on stack at `rsp+0x28` / `rsp+0x30`).
pub fn nt_query_virtual_memory(emu: &mut Emu) {
    if emu.regs().r8 == 0 {
        memory_api::dispatch_nt("NtQueryVirtualMemory", emu, Call::Syscall);
        return;
    }

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

    const MEMORY_MAPPED_FILENAME_INFORMATION: u64 = 2;
    if memory_information_class == MEMORY_MAPPED_FILENAME_INFORMATION {
        let map_name = emu
            .maps
            .get_addr_name(base_address)
            .unwrap_or("")
            .to_string();
        let stem = map_name
            .strip_suffix(".pe")
            .or_else(|| map_name.strip_suffix(".text"))
            .unwrap_or(&map_name);
        let nt_path = if !stem.is_empty() {
            format!("\\Device\\HarddiskVolume2\\Windows\\System32\\{}.dll", stem)
        } else {
            "\\Device\\HarddiskVolume2\\Windows\\System32\\unknown.dll".to_string()
        };
        let wide: Vec<u16> = nt_path.encode_utf16().collect();
        let byte_len = (wide.len() * 2) as u16;
        let needed = 8u64 + byte_len as u64;
        if memory_information_length < needed {
            if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
                emu.maps.write_qword(return_length_ptr, needed);
            }
            emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
            return;
        }
        emu.maps.write_word(memory_information, byte_len);
        emu.maps.write_word(memory_information + 2, byte_len);
        emu.maps.write_dword(memory_information + 4, 0);
        let buf_ptr = memory_information + 8;
        emu.maps.write_qword(memory_information + 4, buf_ptr);
        for (i, &ch) in wide.iter().enumerate() {
            emu.maps.write_word(buf_ptr + (i * 2) as u64, ch);
        }
        if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
            emu.maps.write_qword(return_length_ptr, needed);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    const MEMORY_REGION_INFORMATION: u64 = 4;
    if memory_information_class == MEMORY_REGION_INFORMATION {
        if memory_information_length < 0x50 {
            emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
            return;
        }
        let page_base = base_address & !0xFFF;
        let (alloc_base, alloc_prot, region_type, region_size, commit_size) =
            if emu.maps.is_mapped(base_address) {
                let name = emu
                    .maps
                    .get_addr_name(base_address)
                    .unwrap_or("")
                    .to_string();
                let is_image = name.ends_with(".pe") || name.ends_with(".text");
                let rtype: u32 = if is_image { 4 } else { 1 };
                (page_base, 0x02u32, rtype, 0x1000u64, 0x1000u64)
            } else {
                (0u64, 0u32, 1u32, 0x10000u64, 0u64)
            };
        emu.maps.write_qword(memory_information, alloc_base);
        emu.maps.write_dword(memory_information + 8, alloc_prot);
        emu.maps.write_dword(memory_information + 0x0C, region_type);
        emu.maps.write_qword(memory_information + 0x10, region_size);
        emu.maps.write_qword(memory_information + 0x18, commit_size);
        emu.maps.write_qword(memory_information + 0x20, 0);
        emu.maps.write_qword(memory_information + 0x28, 0);
        if return_length_ptr != 0 && emu.maps.is_mapped(return_length_ptr) {
            emu.maps.write_qword(return_length_ptr, 0x50);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
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
        let mem = emu.maps.get_mem_by_addr(base_address).unwrap();
        let base = base_address & !0xfff;
        let (_, region_size, permission) = mem
            .permission_runs(base, mem.get_bottom() - base)
            .into_iter()
            .next()
            .unwrap();
        let protect = permission_to_nt_page_protection(permission);
        MemoryBasicInformation64 {
            base_address: base,
            allocation_base: mem.get_base(),
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
    memory_api::dispatch_nt("NtAllocateVirtualMemory", emu, Call::Syscall);
}

/// `NtAllocateVirtualMemoryEx` — extended version with MEM_EXTENDED_PARAMETER support.
/// Parameters: RCX ProcessHandle, RDX *BaseAddress, R8 *RegionSize, R9 AllocationType,
/// [RSP+28h] Protect, [RSP+30h] ExtendedParameters, [RSP+38h] ExtendedParameterCount.
pub fn nt_allocate_virtual_memory_ex(emu: &mut Emu) {
    memory_api::dispatch_nt("NtAllocateVirtualMemoryEx", emu, Call::Syscall);
}

/// `NtFreeVirtualMemory` — x64: RCX, RDX, R8, R9 (all in registers).
pub fn nt_free_virtual_memory(emu: &mut Emu) {
    memory_api::dispatch_nt("NtFreeVirtualMemory", emu, Call::Syscall);
}

/// `NtProtectVirtualMemory` — x64: RCX..R9 + 5th (`OldProtect`) at `[rsp+0x28]`.
pub fn nt_protect_virtual_memory(emu: &mut Emu) {
    memory_api::dispatch_nt("NtProtectVirtualMemory", emu, Call::Syscall);
}

/// `NtReadVirtualMemory` — x64: 5th arg (`NumberOfBytesRead`) at `[rsp+0x28]`.
pub fn nt_read_virtual_memory(emu: &mut Emu) {
    memory_api::dispatch_nt("NtReadVirtualMemory", emu, Call::Syscall);
}

/// `NtWriteVirtualMemory` — x64: 5th arg (`NumberOfBytesWritten`) at `[rsp+0x28]`.
pub fn nt_write_virtual_memory(emu: &mut Emu) {
    memory_api::dispatch_nt("NtWriteVirtualMemory", emu, Call::Syscall);
}

/// `NtUnmapViewOfSection` — unmaps anonymous section views created by
/// `NtMapViewOfSection`. DLL image maps (loaded via `load_library`) are
/// kept intact because their PE sections, LDR entries, and export indexes
/// must survive; only `section_view_*` regions are actually deallocated.
pub fn nt_unmap_view_of_section(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;

    let map_name = emu
        .maps
        .get_addr_name(base)
        .unwrap_or("<unmapped>")
        .to_string();

    if !memory_api::current_process(process_handle) {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection h: 0x{:x} base: 0x{:x} [{}] → ACCESS_DENIED",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    if map_name.starts_with("section_view_") {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection h: 0x{:x} base: 0x{:x} [{}] → unmapped",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
        emu.maps.dealloc(base);
        emu.section_view_dll.remove(&base);
    } else {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection h: 0x{:x} base: 0x{:x} [{}] → kept (DLL image)",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAreMappedFilesTheSame` — compares two mapped addresses to determine
/// whether they belong to the same underlying file.
///
/// Resolution order:
/// 1. section_view_dll tracking (populated by NtMapViewOfSection)
/// 2. Map name stem comparison (e.g. `kernelbase.pe` and `kernelbase.text`
///    share stem `kernelbase`)
///
/// Returns STATUS_SUCCESS when same, STATUS_NOT_SAME_DEVICE otherwise.
pub fn nt_are_mapped_files_the_same(emu: &mut Emu) {
    let addr1 = emu.regs().rcx;
    let addr2 = emu.regs().rdx;

    let dll1 = emu.section_view_dll.get(&addr1).cloned();
    let dll2 = emu.section_view_dll.get(&addr2).cloned();

    let name1 = emu.maps.get_addr_name(addr1).unwrap_or("").to_string();
    let name2 = emu.maps.get_addr_name(addr2).unwrap_or("").to_string();

    fn map_stem(name: &str) -> &str {
        let leaf = name.rsplit('/').next().unwrap_or(name);
        leaf.split('.').next().unwrap_or(leaf)
    }

    fn dll_stem(dll: &str) -> &str {
        dll.strip_suffix(".dll").unwrap_or(dll)
    }

    let same = match (&dll1, &dll2) {
        (Some(a), Some(b)) => dll_stem(a) == dll_stem(b),
        (Some(a), None) => dll_stem(a) == map_stem(&name2),
        (None, Some(b)) => map_stem(&name1) == dll_stem(b),
        (None, None) => {
            let s1 = map_stem(&name1);
            let s2 = map_stem(&name2);
            !s1.is_empty() && !s2.is_empty() && s1 == s2
        }
    };

    if same {
        if let Some(ref d) = dll1 {
            emu.validated_knowndlls.insert(d.to_lowercase());
        }
        if let Some(ref d) = dll2 {
            emu.validated_knowndlls.insert(d.to_lowercase());
        }
        log_orange!(
            emu,
            "syscall 0x{:x}: NtAreMappedFilesTheSame(0x{:x} [{}], 0x{:x} [{}]) → SAME",
            WIN64_NTAREMAPPEDFILESTHESAME,
            addr1,
            dll1.as_deref().unwrap_or(&name1),
            addr2,
            dll2.as_deref().unwrap_or(&name2),
        );
        emu.regs_mut().rax = STATUS_SUCCESS;

        // Mark every LDR entry's LoadReason (+0x10C) as 9 so the parallel
        // loader's retry loop sees "already loaded" and stops iterating.
        mark_all_ldr_entries_ready(emu);
    } else {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtAreMappedFilesTheSame(0x{:x} [{}], 0x{:x} [{}]) → NOT_SAME",
            WIN64_NTAREMAPPEDFILESTHESAME,
            addr1,
            dll1.as_deref().unwrap_or(&name1),
            addr2,
            dll2.as_deref().unwrap_or(&name2),
        );
        emu.regs_mut().rax = STATUS_NOT_SAME_DEVICE;
    }
}

/// Walk PEB → Ldr → InLoadOrderModuleList and set [entry+0x10C] = 9 for
/// every LDR_DATA_TABLE_ENTRY, including entries the parallel loader created.
fn mark_all_ldr_entries_ready(emu: &mut Emu) {
    let peb = emu.maps.get_mem("peb").get_base();
    let ldr = emu.maps.read_qword(peb + 0x18).unwrap_or(0);
    if ldr == 0 {
        return;
    }
    let sentinel = ldr + 0x10; // InLoadOrderModuleList head
    let mut cur = emu.maps.read_qword(sentinel).unwrap_or(0);
    let mut count = 0u32;
    while cur != 0 && cur != sentinel && count < 64 {
        let prev = emu.maps.read_dword(cur + 0x10C).unwrap_or(0);
        if prev != 9 {
            emu.maps.write_dword(cur + 0x10C, 9);
        }
        cur = emu.maps.read_qword(cur).unwrap_or(0); // Flink
        count += 1;
    }
}

/// `NtMapViewOfSection` — x64 10-arg syscall.
/// RCX=SectionHandle, RDX=ProcessHandle, R8=*BaseAddress (in/out),
/// R9=ZeroBits, [rsp+0x28]=CommitSize, [rsp+0x30]=SectionOffset,
/// [rsp+0x38]=*ViewSize, [rsp+0x40]=InheritDisposition,
/// [rsp+0x48]=AllocationType, [rsp+0x50]=Win32Protect.
///
/// For KnownDll section handles (tracked via NtOpenSection), loads the real
/// PE from the maps folder so ntdll's loader gets valid image content.
/// Other section handles use the shared mapping model.
pub fn nt_map_view_of_section(emu: &mut Emu) {
    if !emu.section_handles.contains_key(&Call::Syscall.arg(emu, 0)) {
        memory_api::dispatch_nt("NtMapViewOfSection", emu, Call::Syscall);
        return;
    }
    let section_handle = emu.regs().rcx;
    let process_handle = emu.regs().rdx;
    let base_addr_ptr = emu.regs().r8;
    let rsp = emu.regs().rsp;
    let view_size_ptr = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);
    let protect = emu.maps.read_dword(rsp + 0x50).unwrap_or(PAGE_READONLY);

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
        "syscall 0x{:x}: NtMapViewOfSection sh: 0x{:x} base_ptr: 0x{:x} req_base: 0x{:x} size: 0x{:x} prot: 0x{:x}",
        WIN64_NTMAPVIEWOFSECTION,
        section_handle,
        base_addr_ptr,
        requested_base,
        size,
        protect,
    );

    if section_handle == 0 {
        if let Some(dll_name) = emu.last_closed_section_dll.take() {
            let stem = dll_name.strip_suffix(".dll").unwrap_or(&dll_name);
            let pe_name = format!("{}.pe", stem);
            if let Some(map) = emu.maps.get_map_by_name(&pe_name) {
                let dll_base = map.get_base();
                log::trace!(
                    "NtMapViewOfSection: NULL handle → clean view of {} at 0x{:x}",
                    dll_name,
                    dll_base,
                );
                if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
                    let _ = emu.maps.write_qword(base_addr_ptr, dll_base);
                }
                let soi: u64 = {
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
                    let _ = emu.maps.write_qword(view_size_ptr, soi);
                }
                emu.regs_mut().rax = STATUS_SUCCESS;
                return;
            }
        }
        if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
            let _ = emu.maps.write_qword(base_addr_ptr, 0);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if !memory_api::current_process(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    // --- KnownDll path (NtOpenSection) -----------------------------------------
    // load_library makes the DLL permanently resident.  We return the DLL base
    // directly; the subsequent NtUnmapViewOfSection is a no-op for DLL images
    // (they don't carry a "section_view_" prefix).
    if let Some(dll_name) = emu.section_handles.get(&section_handle).cloned() {
        let dll_base = load_library(emu, &dll_name);
        if dll_base != 0 {
            patch_ntdll_load_reason_checks(emu);
            log::trace!(
                "NtMapViewOfSection: KnownDll {} loaded at 0x{:x}",
                dll_name,
                dll_base,
            );
            if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
                let _ = emu.maps.write_qword(base_addr_ptr, dll_base);
            }
            let soi: u64 = {
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
                let _ = emu.maps.write_qword(view_size_ptr, soi);
            }
            emu.regs_mut().rax = STATUS_SUCCESS;
            return;
        }
    }

    // --- File-backed section path (NtCreateSection from file handle) ----------
    if let Some(dll_name) = emu.file_section_names.get(&section_handle).cloned() {
        patch_ntdll_load_reason_checks(emu);
        let stem = dll_name
            .strip_suffix(".dll")
            .unwrap_or(&dll_name)
            .to_string();
        let pe_map_name = format!("{}.pe", stem);
        if let Some(existing) = emu.maps.get_map_by_name(&pe_map_name) {
            let dll_base = existing.get_base();
            let soi: u64 = {
                let pe_off = emu.maps.read_dword(dll_base + 0x3c).unwrap_or(0) as u64;
                if pe_off > 0 {
                    emu.maps
                        .read_dword(dll_base + pe_off + 0x50)
                        .unwrap_or(0x1000) as u64
                } else {
                    0x1000
                }
            };
            let view_addr = emu.maps.lib64_alloc(soi).unwrap_or(0);
            if view_addr != 0 {
                let view_name = format!("section_view_{:x}", view_addr);
                let _ = emu
                    .maps
                    .create_map(&view_name, view_addr, soi, Permission::READ_WRITE);
                for off in (0..soi).step_by(8) {
                    let val = emu.maps.read_qword(dll_base + off).unwrap_or(0);
                    let _ = emu.maps.write_qword(view_addr + off, val);
                }
                emu.section_view_dll.insert(view_addr, dll_name.to_string());
                log_orange!(
                    emu,
                    "NtMapViewOfSection: file-backed {} → temp copy at 0x{:x} (0x{:x} bytes)",
                    stem,
                    view_addr,
                    soi,
                );
                if base_addr_ptr != 0 && emu.maps.is_mapped(base_addr_ptr) {
                    let _ = emu.maps.write_qword(base_addr_ptr, view_addr);
                }
                if view_size_ptr != 0 && emu.maps.is_mapped(view_size_ptr) {
                    let _ = emu.maps.write_qword(view_size_ptr, soi);
                }
                emu.regs_mut().rax = STATUS_SUCCESS;
                return;
            }
        }
    }

    let perm = memory_api::permissions(protect).unwrap_or(Permission::READ_WRITE);

    let mapped_base = if requested_base >= 0x10000 && !emu.maps.is_mapped(requested_base) {
        let name = format!("section_view_{:x}", requested_base);
        match emu.maps.create_map(&name, requested_base, size, perm) {
            Ok(_) => requested_base,
            Err(_) => {
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
        let path = emu.cfg.get_maps_folder(name);
        // KnownDlls contains filenames, not guest paths or host drive prefixes.
        if name.contains(['\\', '/', ':'])
            || matches!(name.as_str(), "." | "..")
            || !std::path::Path::new(&path).exists()
        {
            log::trace!(
                "NtOpenSection: KnownDll {} not in maps_folder → OBJECT_NAME_NOT_FOUND",
                name
            );
            emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            return;
        }
    }

    let h = next_handle();
    let _ = emu.maps.write_qword(handle_out, h);

    // Track as KnownDll if either explicitly a KnownDll path, or the name
    // matches an already-loaded DLL (.pe map exists).  The hot-patch code
    // opens sections by bare name (e.g. "KERNELBASE.dll") without the
    // \KnownDlls\ prefix; returning the loaded .pe base for these avoids a
    // zeroed-memory mapping that fails the MZ check later.
    let effective_dll = dll_name.or_else(|| {
        let lc = section_name.to_lowercase();
        if lc.ends_with(".dll") {
            let stem = lc.strip_suffix(".dll").unwrap_or(&lc);
            let pe_name = format!("{}.pe", stem);
            if emu.maps.get_map_by_name(&pe_name).is_some() {
                return Some(lc);
            }
        }
        None
    });

    if let Some(dll_name) = effective_dll {
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
    if !emu.file_handles.contains_key(&Call::Syscall.arg(emu, 6)) {
        memory_api::dispatch_nt("NtCreateSection", emu, Call::Syscall);
        return;
    }
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

    let h = next_handle();
    // Inherit the DLL name into file_section_names so NtMapViewOfSection can
    // create a section_view with real PE content.
    if file_handle != 0
        && let Some(dll_name) = emu.file_handles.get(&file_handle).cloned()
    {
        emu.file_section_names.insert(h, dll_name);
    }
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
