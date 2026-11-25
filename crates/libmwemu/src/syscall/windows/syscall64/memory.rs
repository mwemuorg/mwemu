use super::{alpc, registry};
use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::memory as memory_api;
use crate::emu::Emu;
use crate::emu::loaders::image_regions;
use crate::emu::object_handle::SectionHandle;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, MemoryKind, PAGE_SIZE, align_up};
use crate::syscall::windows::syscall64::resolve_maps_dll;
use crate::syscall::windows::syscall64::sync::next_handle;
use crate::winapi::common::process::require_memory_access;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::PROCESS_VM_OPERATION;
use crate::windows::constants::syscalls::*;
use goblin::pe::optional_header::{MAGIC_32, MAGIC_64};
use goblin::pe::{PE, header::COFF_MACHINE_X86_64};
use rs_header::pe::pe64::PE64;
use std::fs;

use crate::windows::structures::{MemoryBasicInformation64, UnicodeString64};
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

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
        MAGIC_64 => pe_off + 0x50, // PE32+
        MAGIC_32 => pe_off + 0x50, // PE32
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
        MAGIC_64 => (0xf0usize, pe_off + 0x18 + 0xf0), // PE32+ optional header = 240 bytes
        MAGIC_32 => (0xe0usize, pe_off + 0x18 + 0xe0), // PE32
        _ => return None,
    };
    let _ = opt_hdr_size;
    let size_of_headers_off = match magic {
        MAGIC_64 => pe_off + 0x54,
        MAGIC_32 => pe_off + 0x54,
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
    if magic != MAGIC_64 {
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
    let call = WinSyscall;
    if emu.regs().r8 == 0 {
        memory_api::dispatch_nt("NtQueryVirtualMemory", emu, call);
        return;
    }

    let process_handle = emu.regs().rcx;
    let base_address = emu.regs().rdx;
    let memory_information_class = emu.regs().r8;
    let memory_information = emu.regs().r9;
    let memory_information_length = api_try!(emu, call.arg(emu, 4));
    let return_length_ptr = api_try!(emu, call.arg(emu, 5));

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryVirtualMemory process=0x{:x}, base=0x{:x}, class=0x{:x}, out=0x{:x}, len=0x{:x}, ret_len_ptr=0x{:x}",
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

    if memory_information_class == MEMORY_INFORMATION_CLASS_MEMORY_MAPPED_FILENAME_INFORMATION {
        let base = emu
            .maps
            .allocation_at(base_address)
            .map(|allocation| allocation.base)
            .or_else(|| {
                emu.maps
                    .get_mem_by_addr(base_address)
                    .map(|region| region.get_base())
            });
        let filename = base
            .and_then(|base| emu.section_view_dll.get(&base))
            .cloned()
            .or_else(|| {
                let (image_base, _) = emu.maps.find_pe_image_info(base_address)?;
                let stem = emu.maps.get_addr_name(image_base)?.strip_suffix(".pe")?;
                Some(format!("{stem}.dll"))
            });
        let Some(filename) = filename else {
            return call.set_return(emu, STATUS_NOT_MAPPED_VIEW);
        };
        let path = format!(r"\Device\HarddiskVolume2\Windows\System32\{filename}");
        let mut bytes: Vec<u8> = path
            .encode_utf16()
            .chain([0])
            .flat_map(u16::to_le_bytes)
            .collect();
        let needed = UnicodeString64::size() + bytes.len() as u64;
        if return_length_ptr != 0 && !emu.maps.write_qword(return_length_ptr, needed) {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        }
        if memory_information_length < UnicodeString64::size() {
            return call.set_return(emu, STATUS_INFO_LENGTH_MISMATCH);
        }
        let capacity = memory_information_length.min(needed) - UnicodeString64::size();
        let maximum_length = (capacity & !1) as u16;
        if maximum_length < 2 {
            return call.set_return(emu, STATUS_BUFFER_OVERFLOW);
        }
        bytes.truncate(maximum_length as usize);
        let length = maximum_length - 2;
        bytes[length as usize..].fill(0);
        if !emu.maps.validate_write_range(
            memory_information,
            UnicodeString64::size() + maximum_length as u64,
        ) {
            return call.set_return(emu, STATUS_ACCESS_VIOLATION);
        }
        let buffer = memory_information + UnicodeString64::size();
        UnicodeString64 {
            length,
            maximum_length,
            padding: 0,
            buffer,
        }
        .save(memory_information, &mut emu.maps);
        emu.maps.write_bytes(buffer, &bytes);
        return call.set_return(
            emu,
            if memory_information_length < needed {
                STATUS_BUFFER_OVERFLOW
            } else {
                STATUS_SUCCESS
            },
        );
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
    let call = WinSyscall;
    memory_api::dispatch_nt("NtAllocateVirtualMemory", emu, call);
}

/// `NtAllocateVirtualMemoryEx` — extended version with MEM_EXTENDED_PARAMETER support.
/// Parameters: RCX ProcessHandle, RDX *BaseAddress, R8 *RegionSize, R9 AllocationType,
/// [RSP+28h] Protect, [RSP+30h] ExtendedParameters, [RSP+38h] ExtendedParameterCount.
pub fn nt_allocate_virtual_memory_ex(emu: &mut Emu) {
    let call = WinSyscall;
    memory_api::dispatch_nt("NtAllocateVirtualMemoryEx", emu, call);
}

/// `NtFreeVirtualMemory` — x64: RCX, RDX, R8, R9 (all in registers).
pub fn nt_free_virtual_memory(emu: &mut Emu) {
    let call = WinSyscall;
    memory_api::dispatch_nt("NtFreeVirtualMemory", emu, call);
}

/// `NtProtectVirtualMemory` — x64: RCX..R9 + 5th (`OldProtect`) at `[rsp+0x28]`.
pub fn nt_protect_virtual_memory(emu: &mut Emu) {
    let call = WinSyscall;
    memory_api::dispatch_nt("NtProtectVirtualMemory", emu, call);
}

/// `NtReadVirtualMemory` — x64: 5th arg (`NumberOfBytesRead`) at `[rsp+0x28]`.
pub fn nt_read_virtual_memory(emu: &mut Emu) {
    let call = WinSyscall;
    memory_api::dispatch_nt("NtReadVirtualMemory", emu, call);
}

/// `NtWriteVirtualMemory` — x64: 5th arg (`NumberOfBytesWritten`) at `[rsp+0x28]`.
pub fn nt_write_virtual_memory(emu: &mut Emu) {
    let call = WinSyscall;
    memory_api::dispatch_nt("NtWriteVirtualMemory", emu, call);
}

/// `NtUnmapViewOfSection` — unmaps anonymous section views created by
/// `NtMapViewOfSection`. Resident system DLL images are
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

    if let Err(status) = require_memory_access(emu, process_handle, PROCESS_VM_OPERATION) {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection handle=0x{:x} base=0x{:x} [{}] rejected",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
        emu.regs_mut().rax = status;
        return;
    }

    if emu
        .maps
        .allocation_at(base)
        .is_some_and(|region| region.kind == MemoryKind::Mapped)
    {
        memory_api::dispatch_nt("NtUnmapViewOfSection", emu, WinSyscall);
        return;
    }
    if map_name.starts_with("section_view_") {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection handle=0x{:x} base=0x{:x} [{}] → unmapped",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
        if let Some(allocation) = emu.maps.allocation_at(base)
            && allocation.kind == MemoryKind::Image
        {
            let start = allocation.base;
            let size = allocation.size;
            emu.maps.unmap_range(start, size);
            emu.section_view_dll.remove(&start);
        } else {
            emu.maps.dealloc(base);
            emu.section_view_dll.remove(&base);
        }
    } else {
        log_orange!(
            emu,
            "syscall 0x{:x}: NtUnmapViewOfSection handle=0x{:x} base=0x{:x} [{}] → kept (DLL image)",
            WIN64_NTUNMAPVIEWOFSECTION,
            process_handle,
            base,
            map_name,
        );
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

fn mapped_dll(emu: &Emu, address: u64) -> Option<String> {
    let map = emu.maps.get_mem_by_addr(address)?;
    let base = emu
        .maps
        .allocation_at(address)
        .map_or(map.get_base(), |image| image.base);
    if let Some(name) = emu.section_view_dll.get(&base) {
        return Some(name.clone());
    }
    // Loaded images have a header map and separately named sections.
    let (stem, _) = map.get_name().rsplit_once('.')?;
    emu.maps.get_map_by_name(&format!("{stem}.pe"))?;
    Some(format!("{stem}.dll"))
}

pub fn nt_are_mapped_files_the_same(emu: &mut Emu) {
    let call = WinSyscall;
    let [image, view] = api_try!(emu, call.args(emu));
    let same = mapped_dll(emu, image)
        .zip(mapped_dll(emu, view))
        .is_some_and(|(left, right)| left.eq_ignore_ascii_case(&right));
    let status = if same {
        STATUS_SUCCESS
    } else {
        STATUS_NOT_SAME_DEVICE
    };
    log_orange!(
        emu,
        "NtAreMappedFilesTheSame image={image:#x} view={view:#x} => {status:#x}"
    );
    call.set_return(emu, status);
}

fn section_view_base(emu: &mut Emu, size: u64, requested_base: u64) -> Result<u64, u64> {
    Ok(if requested_base == 0 {
        emu.maps.lib64_alloc(size).ok_or(STATUS_NO_MEMORY)?
    } else {
        if !requested_base.is_multiple_of(ALLOCATION_GRANULARITY) {
            return Err(STATUS_MAPPED_ALIGNMENT);
        }
        if requested_base
            .checked_add(size)
            .is_none_or(|end| end > USER64_ADDRESS_LIMIT)
            || emu.maps.overlaps(requested_base, size)
        {
            return Err(STATUS_CONFLICTING_ADDRESSES);
        }
        requested_base
    })
}

/// Build a minimal stub PE image for a virtual SYSTEM32 DLL.
/// The image has a single .text section whose DllMain returns TRUE so the
/// loader's LdrpInitializeNode succeeds without executing real DLL code.
fn map_stub_dll(emu: &mut Emu, dll: &str, requested_base: u64) -> Result<(u64, u64), u64> {
    const STUB_IMAGE_SIZE: u64 = 0x2000; // 2 pages: headers + .text
    let size = align_up(STUB_IMAGE_SIZE, PAGE_SIZE).ok_or(STATUS_INVALID_IMAGE_FORMAT)?;
    let base = section_view_base(emu, size, requested_base)?;
    let name = format!("section_view_{base:x}");
    emu.maps
        .create_map(&name, base, size, Permission::READ_WRITE_EXECUTE)
        .map_err(|_| STATUS_NO_MEMORY)?;

    // --- DOS header ---
    emu.maps.write_word(base, 0x5A4D); // e_magic = "MZ"
    emu.maps.write_dword(base + 0x3C, 0x80); // e_lfanew

    // --- PE signature ---
    let pe = base + 0x80;
    emu.maps.write_dword(pe, 0x0000_4550); // "PE\0\0"

    // --- COFF header (20 bytes) ---
    let coff = pe + 4;
    emu.maps.write_word(coff, 0x8664); // Machine = AMD64
    emu.maps.write_word(coff + 2, 1); // NumberOfSections
    emu.maps.write_word(coff + 16, 0xF0); // SizeOfOptionalHeader
    emu.maps.write_word(coff + 18, 0x2022); // Characteristics = DLL | EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

    // --- Optional header 64 (240 = 0xF0 bytes) ---
    let opt = coff + 20;
    emu.maps.write_word(opt, 0x020B); // Magic = PE32+
    // AddressOfEntryPoint: points to DllMain stub in .text at RVA 0x1000
    emu.maps.write_dword(opt + 16, 0x1000);
    emu.maps.write_qword(opt + 24, base); // ImageBase
    emu.maps.write_dword(opt + 32, 0x1000); // SectionAlignment
    emu.maps.write_dword(opt + 36, 0x200); // FileAlignment
    emu.maps.write_dword(opt + 56, STUB_IMAGE_SIZE as u32); // SizeOfImage
    emu.maps.write_dword(opt + 60, 0x200); // SizeOfHeaders
    emu.maps.write_dword(opt + 64, 0); // CheckSum
    emu.maps.write_word(opt + 68, 2); // Subsystem = GUI
    emu.maps.write_word(opt + 70, 0x160); // DllCharacteristics = DYNAMIC_BASE | NX_COMPAT | HIGH_ENTROPY_VA
    emu.maps.write_dword(opt + 108, 16); // NumberOfRvaAndSizes

    // --- Section header (.text) ---
    let sect = opt + 0xF0;
    // Name: ".text\0\0\0"
    emu.maps.write_dword(sect, 0x7865_742E); // ".tex"
    emu.maps.write_byte(sect + 4, b't');
    emu.maps.write_dword(sect + 8, 0x10); // VirtualSize
    emu.maps.write_dword(sect + 12, 0x1000); // VirtualAddress (RVA)
    emu.maps.write_dword(sect + 16, 0x200); // SizeOfRawData
    emu.maps.write_dword(sect + 20, 0x200); // PointerToRawData
    emu.maps.write_dword(sect + 36, 0x6000_0020); // Characteristics = CODE | EXECUTE | READ

    // --- DllMain stub at RVA 0x1000 ---
    // BOOL WINAPI DllMain(HINSTANCE, DWORD, LPVOID) { return TRUE; }
    // x64:  mov eax, 1  /  ret
    let entry = base + 0x1000;
    emu.maps.write_byte(entry, 0xB8); // mov eax, ...
    emu.maps.write_dword(entry + 1, 1); // ... 1
    emu.maps.write_byte(entry + 5, 0xC3); // ret

    log::trace!("map_stub_dll: mapped virtual DLL {dll:?} at 0x{base:x} size=0x{size:x}",);

    emu.section_view_dll.insert(base, dll.to_string());
    Ok((base, size))
}

fn map_file_section(emu: &mut Emu, dll: &str, requested_base: u64) -> Result<(u64, u64), u64> {
    const DOS_LFANEW_OFFSET: u64 = 0x3c;
    const NT_SIZE_OF_IMAGE_OFFSET: u64 = 0x50;

    let stem = dll.strip_suffix(".dll").unwrap_or(dll);
    let Some(source) = emu
        .maps
        .get_map_by_name(&format!("{stem}.pe"))
        .map(|map| map.get_base())
    else {
        // NtOpenFile records the basename in the guest DLL cache. The image can be
        // mapped as a file section before any module by this name has been loaded.
        if dll.is_empty() || dll.contains(['/', '\\', ':']) || !dll.ends_with(".dll") {
            return Err(STATUS_INVALID_IMAGE_FORMAT);
        }
        let raw = match resolve_maps_dll(emu, dll) {
            Some(path) => fs::read(path).map_err(|_| STATUS_INVALID_IMAGE_FORMAT)?,
            None if emu.virtual_system32_dlls.contains(dll) => {
                // Virtual SYSTEM32 stub: the DLL is not in maps_folder but
                // was opened successfully (NtOpenFile returned SUCCESS for a
                // virtual SYSTEM32 DLL). Build a minimal PE image whose
                // DllMain immediately returns TRUE so the loader proceeds.
                return map_stub_dll(emu, dll, requested_base);
            }
            None => return Err(STATUS_INVALID_IMAGE_FORMAT),
        };
        let pe = PE::parse(&raw).map_err(|_| STATUS_INVALID_IMAGE_FORMAT)?;
        if !pe.is_64 || pe.header.coff_header.machine != COFF_MACHINE_X86_64 {
            return Err(STATUS_INVALID_IMAGE_FORMAT);
        }
        let image = PE64::parse(dll, &raw);
        let layout = image.image_layout();
        let size = layout.mapped_size();
        if size == 0 || size > emu.maps.max_alloc_size {
            return Err(STATUS_INVALID_IMAGE_FORMAT);
        }
        let requested_base = if image.can_relocate(&raw) {
            requested_base
        } else if requested_base == 0 || requested_base == image.opt.image_base {
            image.opt.image_base
        } else {
            return Err(STATUS_CONFLICTING_ADDRESSES);
        };
        let base = section_view_base(emu, size, requested_base)?;
        let regions = image_regions(&format!("section_view_{base:x}"), &raw, layout);
        emu.maps
            .map_image(base, size, &regions)
            .map_err(|_| STATUS_INVALID_IMAGE_FORMAT)?;
        image.apply_relocations(&raw, emu, base);
        emu.section_view_dll.insert(base, dll.to_string());
        return Ok((base, size));
    };
    let header = source
        .checked_add(DOS_LFANEW_OFFSET)
        .ok_or(STATUS_INVALID_IMAGE_FORMAT)?;
    let pe_offset = emu
        .maps
        .read_dword(header)
        .ok_or(STATUS_INVALID_IMAGE_FORMAT)? as u64;
    let size_address = source
        .checked_add(pe_offset)
        .and_then(|base| base.checked_add(NT_SIZE_OF_IMAGE_OFFSET))
        .ok_or(STATUS_INVALID_IMAGE_FORMAT)?;
    let image_size = emu
        .maps
        .read_dword(size_address)
        .filter(|&size| size != 0)
        .ok_or(STATUS_INVALID_IMAGE_FORMAT)? as u64;
    let size = align_up(image_size, PAGE_SIZE)
        .filter(|&size| size <= emu.maps.max_alloc_size)
        .ok_or(STATUS_NO_MEMORY)?;
    let base = section_view_base(emu, size, requested_base)?;
    if source.checked_add(image_size).is_none() {
        return Err(STATUS_INVALID_IMAGE_FORMAT);
    }
    let name = format!("section_view_{base:x}");
    emu.maps
        .create_map(&name, base, size, Permission::READ_WRITE)
        .map_err(|_| STATUS_NO_MEMORY)?;
    // Copy mapped image bytes; gaps between image sections remain zero-filled.
    for offset in 0..image_size {
        let byte = emu.maps.read_byte(source + offset).unwrap_or(0);
        emu.maps.write_byte(base + offset, byte);
    }
    emu.section_view_dll.insert(base, dll.to_string());
    Ok((base, size))
}

// The guest loader binds imports and publishes KnownDlls in its own PEB lists.
fn map_known_dll(emu: &mut Emu, dll: &str) -> Result<(u64, u64), u64> {
    let stem = dll
        .strip_suffix(".dll")
        .ok_or(STATUS_INVALID_IMAGE_FORMAT)?;
    if stem.is_empty() || stem.contains(['/', '\\', ':']) {
        return Err(STATUS_INVALID_IMAGE_FORMAT);
    }
    if let Some(header) = emu.maps.get_map_by_name(&format!("{stem}.pe")) {
        let image = emu
            .maps
            .allocation_at(header.get_base())
            .filter(|image| image.kind == MemoryKind::Image)
            .ok_or(STATUS_INVALID_IMAGE_FORMAT)?;
        return Ok((image.base, image.size));
    }
    let path = emu.cfg.get_maps_folder(dll);
    let mapped = emu.map_dll_pe64(&path).map_err(|error| {
        log::trace!("NtMapViewOfSection: cannot map {dll}: {error}");
        STATUS_INVALID_IMAGE_FORMAT
    })?;
    Ok((mapped.base, mapped.image.image_layout().mapped_size()))
}

pub fn nt_map_view_of_section(emu: &mut Emu) {
    let call = WinSyscall;
    let section_handle = api_try!(emu, call.arg(emu, 0));
    if !emu.section_handles.contains_key(&section_handle)
        && !emu.file_section_names.contains_key(&section_handle)
    {
        memory_api::dispatch_nt("NtMapViewOfSection", emu, call);
        return;
    }
    let process_handle = emu.regs().rdx;
    let base_addr_ptr = emu.regs().r8;
    let view_size_ptr = api_try!(emu, call.arg(emu, 6));
    let protect = api_try!(emu, call.arg_u32(emu, 9));

    if !memory_api::writable_pointer(emu, base_addr_ptr)
        || !memory_api::writable_pointer(emu, view_size_ptr)
    {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let Some(requested_base) = emu.maps.read_qword(base_addr_ptr) else {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    };
    let Some(view_size) = emu.maps.read_qword(view_size_ptr) else {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    };
    let Some(size) = align_up(view_size.max(PAGE_SIZE), PAGE_SIZE)
        .filter(|&size| size <= emu.maps.max_alloc_size)
    else {
        emu.regs_mut().rax = STATUS_NO_MEMORY;
        return;
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtMapViewOfSection sh=0x{:x} base_ptr=0x{:x} req_base=0x{:x} size=0x{:x} prot=0x{:x}",
        WIN64_NTMAPVIEWOFSECTION,
        section_handle,
        base_addr_ptr,
        requested_base,
        size,
        protect,
    );

    if let Err(status) = require_memory_access(emu, process_handle, PROCESS_VM_OPERATION) {
        emu.regs_mut().rax = status;
        return;
    }

    let view = if let Some(dll) = emu.section_handles.get(&section_handle).cloned() {
        map_known_dll(emu, &dll)
    } else if let Some(dll) = emu.file_section_names.get(&section_handle).cloned() {
        map_file_section(emu, &dll, requested_base)
    } else {
        Err(STATUS_INVALID_IMAGE_FORMAT)
    };
    match view {
        Ok((base, size)) => {
            emu.maps.write_qword(base_addr_ptr, base);
            emu.maps.write_qword(view_size_ptr, size);
            call.set_return(emu, STATUS_SUCCESS);
        }
        Err(status) => call.set_return(emu, status),
    }
}

pub fn nt_open_section(emu: &mut Emu) {
    let call = WinSyscall;
    let [handle_out, desired_access, obj_attr] = api_try!(emu, call.args(emu));
    let (root_dir, section_name, _) = match registry::object_attributes(emu, obj_attr) {
        Ok(attributes) => attributes,
        Err(status) => return call.set_return(emu, status),
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenSection handle_out=0x{:x}, access=0x{:x}, obj_attr=0x{:x} name={:?}",
        WIN64_NTOPENSECTION,
        handle_out,
        desired_access,
        obj_attr,
        section_name,
    );

    if !memory_api::writable_pointer(emu, handle_out) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if root_dir == 0 && alpc::session_object(emu, &section_name, "SharedSection") {
        if desired_access != SECTION_MAP_READ as u64 {
            return call.set_return(emu, STATUS_ACCESS_DENIED);
        }
        let name = section_name.to_ascii_lowercase();
        let section = if let Some(section) = emu.handle_management.named_section(&name) {
            section.clone()
        } else if let Some(map) = emu.maps.get_map_by_name("peb_ro_shared") {
            let bytes = map.get_bytes().into_owned();
            SectionHandle {
                name,
                max_size: bytes.len() as u64,
                protection: PAGE_READONLY,
                storage: emu.maps.create_shared_storage(bytes),
                access: Some(Permission::READ),
            }
        } else {
            return call.set_return(emu, STATUS_OBJECT_NAME_NOT_FOUND);
        };
        let handle = emu.handle_management.insert_section_handle(section);
        emu.maps.write_qword(handle_out, handle as u64);
        return call.set_return(emu, STATUS_SUCCESS);
    }

    // The loader can address KnownDll sections two ways:
    //   1. Absolute path:  ObjectName = `\KnownDlls\kernel32.dll`, RootDirectory = NULL.
    //   2. Relative open:  ObjectName = `kernel32.dll`, RootDirectory = handle from
    //      a previous `NtOpenDirectoryObject("\KnownDlls")`. ntdll's LdrpFindKnownDll
    //      uses this form, so we must resolve it too.
    let is_known_dll_dir = if root_dir == 0 {
        false
    } else {
        match registry::known_dll_directory(emu, root_dir) {
            Ok(_) => true,
            Err(status) => {
                WinSyscall.set_return(emu, status);
                return;
            }
        }
    };
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

    let Some(dll_name) = effective_dll else {
        return call.set_return(emu, STATUS_OBJECT_NAME_NOT_FOUND);
    };
    let handle = next_handle();
    emu.maps.write_qword(handle_out, handle);
    emu.section_handles.insert(handle, dll_name);
    WinSyscall.set_return(emu, STATUS_SUCCESS);
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
    let call = WinSyscall;
    if !emu
        .file_handles
        .contains_key(&api_try!(emu, call.arg(emu, 6)))
    {
        memory_api::dispatch_nt("NtCreateSection", emu, call);
        return;
    }
    let [
        handle_out,
        desired_access,
        object_attributes,
        max_size,
        page_protection,
        alloc_attributes,
        file_handle,
    ] = api_try!(emu, call.args(emu));
    if alloc_attributes as u32 != SEC_IMAGE || object_attributes != 0 || max_size != 0 {
        return policy::unsupported_with_abi(emu, "ntdll", "NtCreateSection", emu.pc(), call);
    }
    if memory_api::permissions(page_protection as u32).is_none() {
        return call.set_return(emu, STATUS_INVALID_PAGE_PROTECTION);
    }

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateSection handle_out=0x{:x}, access=0x{:x}, max_size=0x{:x}, prot=0x{:x}, alloc=0x{:x}, file=0x{:x}",
        WIN64_NTCREATESECTION,
        handle_out,
        desired_access,
        max_size,
        page_protection,
        alloc_attributes,
        file_handle,
    );

    if !emu.maps.validate_write_range(handle_out, 8) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let Some(dll_name) = emu.file_handles.get(&file_handle).cloned() else {
        return call.set_return(emu, STATUS_INVALID_HANDLE);
    };
    let handle = next_handle();
    emu.file_section_names.insert(handle, dll_name);
    emu.maps.write_qword(handle_out, handle);
    call.set_return(emu, STATUS_SUCCESS);
}
