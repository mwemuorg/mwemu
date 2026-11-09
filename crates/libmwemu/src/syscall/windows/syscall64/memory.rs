use crate::api::windows::common::memory as memory_api;
use crate::api::windows::common::memory::Call;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::windows::syscall64::sync::next_handle;
use crate::winapi::winapi64::kernel32::load_library;
use crate::windows::constants::*;
use crate::windows::structures::MemoryBasicInformation64;
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

const STATUS_INVALID_IMAGE_FORMAT: u64 = 0xc000007b;

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

/// Unmap the complete view containing the supplied address.
pub fn nt_unmap_view_of_section(emu: &mut Emu) {
    memory_api::dispatch_nt("NtUnmapViewOfSection", emu, Call::Syscall);
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

    if !memory_api::current_process(process_handle) {
        emu.regs_mut().rax = STATUS_INVALID_HANDLE;
        return;
    }
    if !emu.maps.validate_write_range(base_addr_ptr, 8)
        || !emu.maps.validate_write_range(view_size_ptr, 8)
    {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // If this handle corresponds to a KnownDll section (tracked by NtOpenSection),
    // load the real PE content so ntdll's loader gets valid image data.
    if let Some(dll_name) = emu.section_handles.get(&section_handle).cloned() {
        let dll_base = load_library(emu, &dll_name);
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

    emu.regs_mut().rax = STATUS_INVALID_IMAGE_FORMAT;
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

    let h = next_handle();
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
    // Inherit the DLL backing from the file handle if we tracked one, so the
    // subsequent NtMapViewOfSection on this section loads the real PE.
    if file_handle != 0
        && let Some(dll_name) = emu.file_handles.get(&file_handle).cloned()
    {
        emu.section_handles.insert(h, dll_name);
    }
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
