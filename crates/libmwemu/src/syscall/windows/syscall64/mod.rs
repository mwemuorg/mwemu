use crate::emu::Emu;
use crate::windows::constants::*;

mod alpc;
pub(crate) mod memory;
mod nls;
mod process;
mod registry;
mod sync;
mod system;

/// Resolve a system DLL `basename` (lowercase, e.g. "kernelbase.dll") to a host
/// file inside the maps folder. When `--winver` is active and the file isn't
/// cached yet, fetch it from the symbol server on demand and cache it. Returns
/// `None` only when the file genuinely can't be obtained.
pub(crate) fn resolve_maps_dll(emu: &Emu, basename: &str) -> Option<std::path::PathBuf> {
    if basename.is_empty() {
        return None;
    }
    let p = std::path::Path::new(&emu.cfg.maps_folder).join(basename);
    if p.exists() {
        return Some(p);
    }
    // Lazy symbol-server fetch (only DLLs; data files were seeded at setup).
    if let Some(build) = emu.cfg.winver.clone() {
        if basename.ends_with(".dll") {
            let cache = std::path::Path::new(&emu.cfg.maps_folder);
            // syscall64 is the x64 Windows path.
            match crate::emu::winver::ensure_dll(
                cache,
                &build,
                basename,
                crate::emu::winver::MACHINE_AMD64,
            ) {
                Ok(path) => return Some(path),
                Err(e) => log::trace!("--winver: {} not fetched: {}", basename, e),
            }
        }
    }
    None
}

/// Build a translation table from the syscall numbers the currently-loaded
/// ntdll actually emits → the "canonical" numbers our dispatcher matches on
/// (the `WIN64_NT*` constants, which mirror an older Windows build).
///
/// We scan every `Nt*` export in `ntdll.pe` for the standard stub prologue
/// `4c 8b d1 b8 <imm32>` (mov r10, rcx; mov eax, imm32). For each match, if
/// the same name maps to a constant our dispatcher recognises, we record
/// the mapping. `gateway()` then translates the incoming `nr` via this map
/// so cross-build ntdlls Just Work without rewriting every `WIN64_*` const.
pub fn build_syscall_translation_table(emu: &mut Emu) {
    use crate::windows::constants::*;
    let ntdll_pe = match emu.maps.get_map_by_name("ntdll.pe") {
        Some(m) => m,
        None => return,
    };
    let base = ntdll_pe.get_base();
    let pe_off = match emu.maps.read_dword(base + 0x3c) {
        Some(v) => v as u64,
        None => return,
    };
    let exp_rva = emu.maps.read_dword(base + pe_off + 0x88).unwrap_or(0) as u64;
    if exp_rva == 0 {
        return;
    }
    let exp = base + exp_rva;
    let nname = emu.maps.read_dword(exp + 0x18).unwrap_or(0) as u64;
    let aon = base + emu.maps.read_dword(exp + 0x20).unwrap_or(0) as u64;
    let aono = base + emu.maps.read_dword(exp + 0x24).unwrap_or(0) as u64;

    // Canonical-name → canonical-number table (a subset matching the
    // syscalls our `gateway()` dispatches and the syscalls we observe
    // in LdrInit). Add entries here whenever you introduce a new handler.
    let canonical: &[(&str, u64)] = &[
        ("NtAccessCheck", WIN64_NTACCESSCHECK),
        ("NtAllocateVirtualMemory", WIN64_NTALLOCATEVIRTUALMEMORY),
        ("NtAllocateVirtualMemoryEx", WIN64_NTALLOCATEVIRTUALMEMORYEX),
        ("NtFreeVirtualMemory", WIN64_NTFREEVIRTUALMEMORY),
        ("NtProtectVirtualMemory", WIN64_NTPROTECTVIRTUALMEMORY),
        ("NtQueryVirtualMemory", WIN64_NTQUERYVIRTUALMEMORY),
        ("NtReadVirtualMemory", WIN64_NTREADVIRTUALMEMORY),
        ("NtWriteVirtualMemory", WIN64_NTWRITEVIRTUALMEMORY),
        ("NtCreateSection", WIN64_NTCREATESECTION),
        ("NtOpenSection", WIN64_NTOPENSECTION),
        ("NtMapViewOfSection", WIN64_NTMAPVIEWOFSECTION),
        ("NtUnmapViewOfSection", WIN64_NTUNMAPVIEWOFSECTION),
        ("NtQuerySystemInformation", WIN64_NTQUERYSYSTEMINFORMATION),
        ("NtQueryInformationProcess", WIN64_NTQUERYINFORMATIONPROCESS),
        ("NtQueryInformationThread", WIN64_NTQUERYINFORMATIONTHREAD),
        ("NtSetInformationProcess", WIN64_NTSETINFORMATIONPROCESS),
        ("NtSetInformationThread", WIN64_NTSETINFORMATIONTHREAD),
        ("NtOpenProcess", WIN64_NTOPENPROCESS),
        ("NtDuplicateObject", WIN64_NTDUPLICATEOBJECT),
        ("NtTerminateProcess", WIN64_NTTERMINATEPROCESS),
        ("NtQueryPerformanceCounter", WIN64_NTQUERYPERFORMANCECOUNTER),
        ("NtCreateEvent", WIN64_NTCREATEEVENT),
        ("NtSetEvent", WIN64_NTSETEVENT),
        ("NtClose", WIN64_NTCLOSE),
        ("NtOpenKey", WIN64_NTOPENKEY),
        ("NtQueryValueKey", WIN64_NTQUERYVALUEKEY),
        ("NtRaiseException", WIN64_NTRAISEEXCEPTION),
        ("NtContinue", WIN64_NTCONTINUE),
        ("NtQueryAttributesFile", WIN64_NTQUERYATTRIBUTESFILE),
        ("NtOpenFile", WIN64_NTOPENFILE),
        ("NtCreateFile", WIN64_NTCREATEFILE),
        ("NtDeviceIoControlFile", WIN64_NTDEVICEIOCONTROLFILE),
        ("NtCreateIoCompletion", WIN64_NTCREATEIOCOMPLETION),
        (
            "NtCreateWaitCompletionPacket",
            WIN64_NTCREATEWAITCOMPLETIONPACKET,
        ),
        ("NtCreateWorkerFactory", WIN64_NTCREATEWORKERFACTORY),
        (
            "NtSetInformationWorkerFactory",
            WIN64_NTSETINFORMATIONWORKERFACTORY,
        ),
        ("NtShutdownWorkerFactory", WIN64_NTSHUTDOWNWORKERFACTORY),
        ("NtQueryWnfStateData", WIN64_NTQUERYWNFSTATEDATA),
        ("NtOpenDirectoryObject", WIN64_NTOPENDIRECTORYOBJECT),
        ("NtOpenSymbolicLinkObject", WIN64_NTOPENSYMBOLICLINKOBJECT),
        ("NtQuerySymbolicLinkObject", WIN64_NTQUERYSYMBOLICLINKOBJECT),
        ("NtAlpcConnectPort", WIN64_NTALPCCONNECTPORT),
        ("NtAlpcSendWaitReceivePort", WIN64_NTALPCSENDWAITRECEIVEPORT),
        ("NtTraceEvent", WIN64_NTTRACEEVENT),
        ("NtOpenEvent", WIN64_NTOPENEVENT),
        ("NtWaitForSingleObject", WIN64_NTWAITFORSINGLEOBJECT),
        ("NtApphelpCacheControl", WIN64_NTAPPHELPCACHECONTROL),
        ("NtAreMappedFilesTheSame", WIN64_NTAREMAPPEDFILESTHESAME),
        (
            "NtQueryQuotaInformationFile",
            WIN64_NTQUERYQUOTAINFORMATIONFILE,
        ),
        (
            "NtQuerySystemEnvironmentValueEx",
            WIN64_NTQUERYSYSTEMENVIRONMENTVALUEEX,
        ),
        ("NtRaiseHardError", WIN64_NTRAISEHARDERROR),
        ("NtRaiseException", WIN64_NTRAISEEXCEPTION),
    ];
    let by_name: std::collections::HashMap<&'static str, u64> = canonical.iter().copied().collect();

    let mut map = std::collections::HashMap::new();
    let mut name_map = std::collections::HashMap::new();
    for i in 0..nname {
        let nrva = emu.maps.read_dword(aon + i * 4).unwrap_or(0) as u64;
        if nrva == 0 {
            continue;
        }
        let mut name = String::new();
        let mut k = 0u64;
        while k < 128 {
            let b = emu.maps.read_byte(base + nrva + k).unwrap_or(0);
            if b == 0 {
                break;
            }
            name.push(b as char);
            k += 1;
        }
        if !name.starts_with("Nt") {
            continue;
        }
        let ord = emu.maps.read_word(aono + i * 2).unwrap_or(0) as u64;
        let aof_rva = emu.maps.read_dword(exp + 0x1c).unwrap_or(0) as u64;
        let fn_rva = emu.maps.read_dword(base + aof_rva + ord * 4).unwrap_or(0) as u64;
        if fn_rva == 0 {
            continue;
        }
        // Stub prologue: 4c 8b d1 b8 <imm32>
        if emu.maps.read_byte(base + fn_rva).unwrap_or(0) != 0x4c {
            continue;
        }
        if emu.maps.read_byte(base + fn_rva + 1).unwrap_or(0) != 0x8b {
            continue;
        }
        if emu.maps.read_byte(base + fn_rva + 2).unwrap_or(0) != 0xd1 {
            continue;
        }
        if emu.maps.read_byte(base + fn_rva + 3).unwrap_or(0) != 0xb8 {
            continue;
        }
        let real_nr = emu.maps.read_dword(base + fn_rva + 4).unwrap_or(0) as u64;
        name_map.insert(real_nr, name.clone());
        if let Some(&canonical_nr) = by_name.get(name.as_str()) {
            if real_nr != canonical_nr {
                map.insert(real_nr, canonical_nr);
            }
        }
    }
    log::trace!(
        "syscall translation table built: {} entries (real_nr → canonical), {} names indexed (from loaded ntdll)",
        map.len(),
        name_map.len(),
    );
    emu.syscall_number_map = map;
    emu.syscall_name_by_real = name_map;
}

pub fn gateway(emu: &mut Emu) {
    let real_nr = emu.regs().rax;
    if !emu.call_syscall_hook(real_nr) {
        return;
    }
    let mut nr = real_nr;
    // Translate the syscall number coming from the loaded ntdll to the
    // canonical number our dispatcher matches against.
    if let Some(&translated) = emu.syscall_number_map.get(&nr) {
        nr = translated;
    } else if let Some(real_name) = emu.syscall_name_by_real.get(&real_nr) {
        // No translation. Verify the real-name from the loaded ntdll matches
        // what our static `what_syscall()` would say — if they differ, the
        // dispatcher would route this syscall to the wrong handler (e.g.
        // Win2022's 0x133 is `NtOpenSymbolicLinkObject` but our constant
        // `WIN64_NTOPENPROCESSTOKEN = 0x133` from an older build). Tag it
        // as unimplemented under its true name to avoid silent misroute.
        let canonical_name = what_syscall(real_nr);
        if &canonical_name != real_name {
            log_orange!(
                emu,
                "syscall 0x{:x}: {} (unimplemented; static table would have misrouted as {})",
                real_nr,
                real_name,
                canonical_name,
            );
            emu.regs_mut().rax = STATUS_NOT_IMPLEMENTED;
            return;
        }
    }
    match nr {
        WIN64_NTACCESSCHECK => process::nt_access_check(emu),
        WIN64_NTALLOCATEVIRTUALMEMORY => memory::nt_allocate_virtual_memory(emu),
        WIN64_NTALLOCATEVIRTUALMEMORYEX => memory::nt_allocate_virtual_memory_ex(emu),
        WIN64_NTFREEVIRTUALMEMORY => memory::nt_free_virtual_memory(emu),
        WIN64_NTPROTECTVIRTUALMEMORY => memory::nt_protect_virtual_memory(emu),
        WIN64_NTQUERYVIRTUALMEMORY => memory::nt_query_virtual_memory(emu),
        WIN64_NTREADVIRTUALMEMORY => memory::nt_read_virtual_memory(emu),
        WIN64_NTWRITEVIRTUALMEMORY => memory::nt_write_virtual_memory(emu),
        WIN64_NTCREATESECTION => memory::nt_create_section(emu),
        WIN64_NTOPENSECTION => memory::nt_open_section(emu),
        WIN64_NTMAPVIEWOFSECTION => memory::nt_map_view_of_section(emu),
        WIN64_NTUNMAPVIEWOFSECTION => memory::nt_unmap_view_of_section(emu),
        WIN64_NTQUERYSYSTEMINFORMATION => system::nt_query_system_information(emu),
        WIN64_NTQUERYINFORMATIONPROCESS => process::nt_query_information_process(emu),
        WIN64_NTQUERYINFORMATIONTHREAD => process::nt_query_information_thread(emu),
        WIN64_NTSETINFORMATIONPROCESS => process::nt_set_information_process(emu),
        WIN64_NTSETINFORMATIONTHREAD => process::nt_set_information_thread(emu),
        WIN64_NTOPENPROCESS => process::nt_open_process(emu),
        WIN64_NTDUPLICATEOBJECT => process::nt_duplicate_object(emu),
        WIN64_NTTERMINATEPROCESS => process::nt_terminate_process(emu),
        WIN64_NTQUERYPERFORMANCECOUNTER => process::nt_query_performance_counter(emu),
        WIN64_NTCREATEEVENT => sync::nt_create_event(emu),
        WIN64_NTSETEVENT => sync::nt_set_event(emu),
        WIN64_NTCLOSE => sync::nt_close(emu),
        WIN64_NTOPENKEY => registry::nt_open_key(emu),
        WIN64_NTQUERYOPENSUBKEYSEX => registry::nt_query_open_subkeys_ex(emu),
        WIN64_NTOPENKEYEX => registry::nt_open_key_ex(emu),
        WIN64_NTOPENKEYTRANSACTED => registry::nt_open_key_transacted(emu),
        WIN64_NTOPENKEYTRANSACTEDEX => registry::nt_open_key_transacted_ex(emu),
        WIN64_NTQUERYVALUEKEY => registry::nt_query_value_key(emu),
        WIN64_NTQUERYMULTIPLEVALUEKEY => registry::nt_query_multiple_value_key(emu),
        WIN64_NTOPENDIRECTORYOBJECT => registry::nt_open_directory_object(emu),
        WIN64_NTCREATEDIRECTORYOBJECT => registry::nt_create_directory_object(emu),
        WIN64_NTOPENSYMBOLICLINKOBJECT => registry::nt_open_symbolic_link_object(emu),
        WIN64_NTQUERYSYMBOLICLINKOBJECT => registry::nt_query_symbolic_link_object(emu),
        WIN64_NTMANAGEHOTPATCH => system::nt_manage_hot_patch(emu),
        WIN64_NTQUERYDEBUGFILTERSTATE => system::nt_query_debug_filter_state(emu),
        WIN64_NTTRACEEVENT => system::nt_trace_event(emu),
        WIN64_NTQUERYIOCOMPLETION => system::nt_query_io_completion(emu),
        WIN64_NTRAISEEXCEPTION => process::nt_raise_exception(emu),
        WIN64_NTRAISEHARDERROR => process::nt_raise_hard_error(emu),
        WIN64_NTWAITFORALERTBYTHREADID => sync::nt_wait_for_alert_by_thread_id(emu),
        WIN64_NTQUERYSECURITYATTRIBUTESTOKEN => process::nt_query_security_attributes_token(emu),
        WIN64_NTALLOCATEUSERPHYSICALPAGESEX => memory::nt_allocate_user_physical_pages_ex(emu),
        WIN64_NTQUERYINFORMATIONTRANSACTIONMANAGER => {
            system::nt_query_information_transaction_manager(emu)
        }
        WIN64_NTCREATETHREADEX => process::nt_create_thread_ex(emu),
        WIN64_NTCONTINUE => process::nt_continue(emu),
        WIN64_NTCREATETIMER2 => sync::nt_create_timer2(emu),
        WIN64_NTOPENEVENT => sync::nt_open_event(emu),
        WIN64_NTWAITFORSINGLEOBJECT => sync::nt_wait_for_single_object(emu),
        WIN64_NTALPCACCEPTCONNECTPORT => sync::nt_alpc_accept_connect_port(emu),
        WIN64_NTCREATEPROFILEEX => process::nt_create_profile_ex(emu),
        WIN64_NTALPCCANCELMESSAGE => alpc::nt_alpc_cancel_message(emu),
        WIN64_NTALPCCONNECTPORT => alpc::nt_alpc_connect_port(emu),
        WIN64_NTALPCCONNECTPORTEX => alpc::nt_alpc_connect_port_ex(emu),
        WIN64_NTALPCCREATEPORT => alpc::nt_alpc_create_port(emu),
        WIN64_NTALPCCREATEPORTSECTION => alpc::nt_alpc_create_port_section(emu),
        WIN64_NTALPCCREATERESOURCERESERVE => alpc::nt_alpc_create_resource_reserve(emu),
        WIN64_NTALPCCREATESECTIONVIEW => alpc::nt_alpc_create_section_view(emu),
        WIN64_NTALPCCREATESECURITYCONTEXT => alpc::nt_alpc_create_security_context(emu),
        WIN64_NTALPCDELETEPORTSECTION => alpc::nt_alpc_delete_port_section(emu),
        WIN64_NTALPCDELETERESOURCERESERVE => alpc::nt_alpc_delete_resource_reserve(emu),
        WIN64_NTALPCDELETESECTIONVIEW => alpc::nt_alpc_delete_section_view(emu),
        WIN64_NTALPCDELETESECURITYCONTEXT => alpc::nt_alpc_delete_security_context(emu),
        WIN64_NTALPCDISCONNECTPORT => alpc::nt_alpc_disconnect_port(emu),
        WIN64_NTALPCIMPERSONATECLIENTCONTAINEROFPORT => {
            alpc::nt_alpc_impersonate_client_container_of_port(emu)
        }
        WIN64_NTALPCIMPERSONATECLIENTOFPORT => alpc::nt_alpc_impersonate_client_of_port(emu),
        WIN64_NTALPCOPENSENDERPROCESS => alpc::nt_alpc_open_sender_process(emu),
        WIN64_NTALPCOPENSENDERTHREAD => alpc::nt_alpc_open_sender_thread(emu),
        WIN64_NTALPCQUERYINFORMATION => alpc::nt_alpc_query_information(emu),
        WIN64_NTALPCQUERYINFORMATIONMESSAGE => alpc::nt_alpc_query_information_message(emu),
        WIN64_NTALPCREVOKESECURITYCONTEXT => alpc::nt_alpc_revoke_security_context(emu),
        WIN64_NTALPCSENDWAITRECEIVEPORT => alpc::nt_alpc_send_wait_receive_port(emu),
        WIN64_NTALPCSETINFORMATION => alpc::nt_alpc_set_information(emu),
        // `NtInitializeNlsFiles` (0x108): map `locale.nls` so ntdll's
        // `RtlInitNlsTables` gets the real Unicode upcase/casing tables.
        // Without this, every byte→wide conversion in the loader yields zeros
        // and dependency lookups fail with `STATUS_DLL_NOT_FOUND`.
        0x108 => nls::nt_initialize_nls_files(emu),
        // `NtGetNlsSectionPtr` (0x102): map `C_<n>.NLS` for the requested
        // codepage (1252 ANSI, 437 OEM, etc.). Same purpose as above —
        // populates the byte→wide / wide→byte tables consumed by
        // `RtlAnsiStringToUnicodeString`.
        0x102 => nls::nt_get_nls_section_ptr(emu),
        // `NtQueryAttributesFile` (0x3d): ntdll's loader stats DLL files on disk
        // to validate the module cache after mapping them via KnownDlls. For real
        // DLLs (kernelbase.dll, kernel32.dll, …) returning OBJECT_NAME_NOT_FOUND
        // makes the loader treat the module as corrupt and call
        // `NtTerminateProcess(STATUS_DLL_NOT_FOUND)`. Resolve the NT path against
        // `cfg.maps_folder` and fill a FILE_BASIC_INFORMATION when the file
        // exists; only fall back to NOT_FOUND for genuinely missing files
        // (manifests, .local, policy probes).
        WIN64_NTQUERYATTRIBUTESFILE => {
            let obj_attr = emu.regs().rcx;
            let file_info_ptr = emu.regs().rdx;
            let nt_name = crate::syscall::windows::syscall64::memory::read_object_attributes_name(
                emu, obj_attr,
            );
            // Extract the filename (last path segment) from the NT path.
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            let resolved = resolve_maps_dll(emu, &basename);
            if let Some(path) = resolved {
                if file_info_ptr != 0 && emu.maps.is_mapped(file_info_ptr) {
                    // FILE_BASIC_INFORMATION: 4× LARGE_INTEGER + ULONG FileAttributes.
                    // Real timestamps would require unix→FILETIME conversion; the
                    // loader only checks them against its own cache, so a fixed
                    // recent value is good enough for any single emulation.
                    const FAKE_FILETIME: u64 = 0x01DA_0000_0000_0000; // ~2023
                    const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
                    let _ = emu.maps.write_qword(file_info_ptr, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x08, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x10, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x18, FAKE_FILETIME);
                    let _ = emu
                        .maps
                        .write_dword(file_info_ptr + 0x20, FILE_ATTRIBUTE_NORMAL);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQueryAttributesFile name={:?} → resolved {:?} (SUCCESS)",
                    WIN64_NTQUERYATTRIBUTESFILE,
                    nt_name,
                    path,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQueryAttributesFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    WIN64_NTQUERYATTRIBUTESFILE,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        // `NtAreMappedFilesTheSame` (0x8e on some Windows builds, 0x90 on others):
        // the loader calls this right after mapping a KnownDll section to check
        // whether the freshly section-mapped view and the on-disk file are the
        // same image. For the genuine System32 DLLs we serve via --iso they ARE
        // the same file, so the correct answer is STATUS_SUCCESS. Answering
        // STATUS_NOT_SAME_DEVICE makes ntdll reject the KnownDll mapping, fall
        // back to relocating the disk copy itself, and then fail re-validation
        // with STATUS_INVALID_IMAGE_FORMAT — which aborts process init.
        0x8e | 0x90 => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtAreMappedFilesTheSame (stub → SAME / SUCCESS)",
                nr,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtConnectPort` (0xa4): the CSR client (`CsrClientConnectToServer`,
        // called from kernelbase's DLL init) connects to the CSRSS API port.
        // There is no CSRSS to talk to, but the connect MUST succeed — returning
        // STATUS_NOT_IMPLEMENTED makes kernelbase's DllMain return FALSE and
        // ntdll aborts process init with STATUS_DLL_INIT_FAILED. We fake a
        // connected port: hand back a port handle and map the shared-memory view
        // the client passed in via its PORT_VIEW, filling ViewBase so the
        // client's `CsrPortHeap` / remote-delta arithmetic stays self-consistent.
        // PORT_VIEW layout: {Length@0, SectionHandle@8, SectionOffset@0x10,
        // ViewSize@0x18, ViewBase@0x20, ViewRemoteBase@0x28}.
        0xa4 => {
            let port_handle_ptr = emu.regs().rcx;
            let client_view_ptr = emu.regs().r9;
            let rsp = emu.regs().rsp;
            // arg6 (MaxMessageLength, PULONG out) sits at [rsp+0x30] in the
            // syscall stack frame; the slot holds the caller's pointer.
            let max_msg_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

            let h = crate::syscall::windows::syscall64::sync::next_handle();
            if port_handle_ptr != 0 && emu.maps.is_mapped(port_handle_ptr) {
                let _ = emu.maps.write_qword(port_handle_ptr, h);
            }

            let mut view_base = 0u64;
            if client_view_ptr != 0 && emu.maps.is_mapped(client_view_ptr) {
                let req = emu.maps.read_qword(client_view_ptr + 0x18).unwrap_or(0);
                // Clamp to a sane window; the client only needs a scratch heap
                // for CSR messages it will never actually send here.
                let size = if req == 0 || req > 0x0010_0000 {
                    0x0001_0000
                } else {
                    req
                };
                let base = emu.maps.lib64_alloc(size).unwrap_or(0);
                if base != 0 {
                    let perm = crate::maps::mem64::Permission::from_flags(true, true, false);
                    let _ =
                        emu.maps
                            .create_map(&format!("csr_port_view_{:x}", h), base, size, perm);
                    // ViewRemoteBase == ViewBase → remote delta 0, which is fine
                    // in our single address space (no separate CSR server view).
                    let _ = emu.maps.write_qword(client_view_ptr + 0x20, base);
                    let _ = emu.maps.write_qword(client_view_ptr + 0x28, base);
                    view_base = base;
                }
            }

            if max_msg_ptr != 0 && emu.maps.is_mapped(max_msg_ptr) {
                // LPC/ALPC default maximum message length.
                let _ = emu.maps.write_dword(max_msg_ptr, 0x148);
            }

            log_orange!(
                emu,
                "syscall 0x{:x}: NtConnectPort → fake CSR port handle 0x{:x}, view 0x{:x} (SUCCESS)",
                nr,
                h,
                view_base,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtQuerySystemEnvironmentValueEx` (0x16d): newer ntdll enumerates
        // UEFI variables during init. NOT_IMPLEMENTED and VARIABLE_NOT_FOUND
        // both cause a tight retry loop (~3K iterations). Return SUCCESS with
        // ValueLength=0 and ValueBuffer untouched so the caller treats it as
        // "the variable is empty" and moves on.
        0x16d => {
            // NT syscall ABI: rcx=VarName, rdx=VendorGuid, r8=ValueBuffer,
            // r9=ValueLengthPtr, [rsp+0x28]=Attributes (out, optional).
            let value_len_ptr = emu.regs().r9;
            if value_len_ptr != 0 && emu.maps.is_mapped(value_len_ptr) {
                let _ = emu.maps.write_dword(value_len_ptr, 0);
            }
            log_orange!(
                emu,
                "syscall 0x{:x}: NtQuerySystemEnvironmentValueEx (stub → SUCCESS, len=0)",
                nr
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtOpenProcessToken` (0x133): return SUCCESS with a fake handle to
        // bypass ntdll's bail path. (Empirically the loops downstream are
        // caused by other unstubbed token-using syscalls; we want to see
        // which ones surface next.)
        0x133 => {
            let token_handle_out = emu.regs().r8;
            if token_handle_out != 0 && emu.maps.is_mapped(token_handle_out) {
                let h = crate::syscall::windows::syscall64::sync::next_handle();
                let _ = emu.maps.write_qword(token_handle_out, h);
            }
            log_orange!(
                emu,
                "syscall 0x{:x}: NtOpenProcessToken (stub → SUCCESS, fake handle)",
                nr
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtQuerySystemInformationEx` (0x16e): newer ntdll uses this during
        // `LdrpInitializeProcess`. Returning STATUS_SUCCESS with `ReturnLength=0`
        // is enough to keep init going — the caller treats "no data" as "no
        // extra info". We do NOT zero the output buffer: callers sometimes
        // pass wildly inflated `out_len` values and a bulk memset overruns
        // the destination map, corrupting adjacent regions.
        0x16e => {
            let rsp = emu.regs().rsp;
            let info_class = emu.regs().rcx;
            let in_buf = emu.regs().rdx;
            let sysinfo = emu.regs().r9;
            let sysinfo_len = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as u32;
            let ret_len_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

            // class 0x6b = SystemLogicalProcessorAndGroupInformation. ntdll's
            // parallel loader queries this (RelationGroup) very early: it calls
            // once with a NULL/short buffer expecting STATUS_INFO_LENGTH_MISMATCH
            // plus the required size, allocates, and retries. Answering SUCCESS
            // the first time breaks that pattern and ntdll dereferences a NULL
            // result buffer. We model one processor group with one active core.
            let relationship = if in_buf != 0 {
                emu.maps.read_dword(in_buf).unwrap_or(0xffff_ffff)
            } else {
                0xffff_ffff
            };
            if info_class == 0x6b && relationship == 4 {
                // SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX (RelationGroup): a
                // 0x20-byte header + one 0x30-byte PROCESSOR_GROUP_INFO = 0x50.
                const NEED: u32 = 0x50;
                if ret_len_ptr != 0 && emu.maps.is_mapped(ret_len_ptr) {
                    let _ = emu.maps.write_dword(ret_len_ptr, NEED);
                }
                if sysinfo == 0 || sysinfo_len < NEED || !emu.maps.is_mapped(sysinfo) {
                    log_orange!(
                        emu,
                        "syscall 0x{:x}: NtQuerySystemInformationEx SystemLogicalProcessorAndGroupInformation (need 0x{:x}, got 0x{:x}) → STATUS_INFO_LENGTH_MISMATCH",
                        nr,
                        NEED,
                        sysinfo_len,
                    );
                    emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                } else {
                    for off in 0..NEED as u64 {
                        let _ = emu.maps.write_byte(sysinfo + off, 0);
                    }
                    let _ = emu.maps.write_dword(sysinfo, 4); // Relationship = RelationGroup
                    let _ = emu.maps.write_dword(sysinfo + 0x04, NEED); // Size
                    let _ = emu.maps.write_word(sysinfo + 0x08, 1); // MaximumGroupCount
                    let _ = emu.maps.write_word(sysinfo + 0x0a, 1); // ActiveGroupCount
                    let _ = emu.maps.write_byte(sysinfo + 0x20, 1); // GroupInfo[0].MaximumProcessorCount
                    let _ = emu.maps.write_byte(sysinfo + 0x21, 1); // GroupInfo[0].ActiveProcessorCount
                    let _ = emu.maps.write_qword(sysinfo + 0x48, 1); // GroupInfo[0].ActiveProcessorMask
                    log_orange!(
                        emu,
                        "syscall 0x{:x}: NtQuerySystemInformationEx SystemLogicalProcessorAndGroupInformation → 1 group/1 cpu",
                        nr,
                    );
                    emu.regs_mut().rax = STATUS_SUCCESS;
                }
            } else {
                // Generic fallback: SUCCESS with ReturnLength=0. We do NOT zero the
                // output buffer — callers sometimes pass wildly inflated `out_len`
                // and a bulk memset would overrun the destination map.
                if ret_len_ptr != 0 && emu.maps.is_mapped(ret_len_ptr) {
                    let _ = emu.maps.write_dword(ret_len_ptr, 0);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQuerySystemInformationEx class=0x{:x} (stub → SUCCESS, len=0)",
                    nr,
                    info_class,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
        }
        // `NtQueryQuotaInformationFile` (0x166): kernelbase's TLS-callback /
        // process-init path queries this for each loaded module. Returning
        // STATUS_NOT_IMPLEMENTED here causes ntdll to retry indefinitely
        // (we observed ~500K calls eating through the entire stack reserve).
        // STATUS_NOT_SUPPORTED tells the caller "this volume doesn't track
        // quotas" and is treated as a terminal, non-retryable answer.
        0x166 => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtQueryQuotaInformationFile (stub → NOT_SUPPORTED)",
                nr,
            );
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }
        // `NtOpenFile` (0x33): ntdll's loader opens KnownDll files on disk to
        // verify them after mapping (timestamps, share-mode locks, …). For real
        // DLLs returning OBJECT_NAME_NOT_FOUND makes the loader treat them as
        // missing and terminate with STATUS_DLL_NOT_FOUND. Resolve the NT path
        // against `cfg.maps_folder` and hand out a fake handle when the file
        // exists; only return NOT_FOUND for genuinely absent paths.
        0x33 => {
            let file_handle_out = emu.regs().rcx;
            let obj_attr = emu.regs().r8;
            let io_status_block = emu.regs().r9;
            let nt_name = crate::syscall::windows::syscall64::memory::read_object_attributes_name(
                emu, obj_attr,
            );
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            let exists = resolve_maps_dll(emu, &basename).is_some();
            if exists {
                let h = crate::syscall::windows::syscall64::sync::next_handle();
                if file_handle_out != 0 && emu.maps.is_mapped(file_handle_out) {
                    let _ = emu.maps.write_qword(file_handle_out, h);
                }
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (NTSTATUS, 4) + pad(4) + Information (ULONG_PTR, 8).
                    // Information = FILE_OPENED (1).
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 1);
                }
                // Track the handle → basename so a follow-up NtCreateSection(file: h)
                // can mark the new section as backed by this DLL and NtMapViewOfSection
                // will load the real PE.
                emu.file_handles.insert(h, basename.clone());
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtOpenFile name={:?} → handle 0x{:x} (SUCCESS)",
                    nr,
                    nt_name,
                    h,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtOpenFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    nr,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        // `NtCreateFile` (0x55): ntdll's loader opens DLL files on disk with
        // NtCreateFile (FILE_OPEN disposition) to back a section before mapping.
        // Same arg layout as NtOpenFile for the handles we care about:
        // FileHandle=rcx, ObjectAttributes=r8, IoStatusBlock=r9. Resolve the NT
        // path against `cfg.maps_folder`; hand out a fake handle when the file
        // exists and only return OBJECT_NAME_NOT_FOUND for genuinely absent paths.
        WIN64_NTCREATEFILE => {
            let file_handle_out = emu.regs().rcx;
            let obj_attr = emu.regs().r8;
            let io_status_block = emu.regs().r9;
            let nt_name = crate::syscall::windows::syscall64::memory::read_object_attributes_name(
                emu, obj_attr,
            );
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            // The console device (`\Device\ConDrv\…`) is opened during process
            // init (kernel32 `ConsoleAllocate`/`SetUpConsoleHandle`). It is not a
            // file in `maps_folder`, so resolving it by basename would fail and
            // make the console DLL init return STATUS_DLL_INIT_FAILED. Hand out a
            // fake handle so process startup proceeds.
            //
            // After opening `\Device\ConDrv\Server`, kernel32 opens the console
            // sub-objects *relative* to that handle: `NtCreateFile("\Reference",
            // RootDirectory = ConDrv handle)`, then `\Connect`, `\Input`,
            // `\Output`. These carry no "condrv" in their ObjectName, so detect
            // them by their RootDirectory being a handle we already tagged as a
            // console object.
            let root_dir =
                crate::syscall::windows::syscall64::memory::read_object_attributes_root_directory(
                    emu, obj_attr,
                );
            let is_console = nt_name.to_lowercase().contains("condrv")
                || (root_dir != 0 && emu.console_handles.contains(&root_dir));
            let exists = is_console || resolve_maps_dll(emu, &basename).is_some();
            if exists {
                let h = crate::syscall::windows::syscall64::sync::next_handle();
                if file_handle_out != 0 && emu.maps.is_mapped(file_handle_out) {
                    let _ = emu.maps.write_qword(file_handle_out, h);
                }
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (4) + pad(4) + Information (8).
                    // Information = FILE_OPENED (1).
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 1);
                }
                // Don't register console pseudo-handles as DLL-backed files; only
                // real on-disk files feed the NtCreateSection → NtMapViewOfSection
                // PE-loading path. Track them in `console_handles` instead so the
                // relative sub-object opens above resolve and NtDeviceIoControlFile
                // can answer the CONDRV IOCTLs on them.
                if is_console {
                    emu.console_handles.insert(h);
                } else {
                    emu.file_handles.insert(h, basename.clone());
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtCreateFile name={:?} → handle 0x{:x} (SUCCESS)",
                    nr,
                    nt_name,
                    h,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtCreateFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    nr,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        // `NtDeviceIoControlFile` (0x7): kernel32's console client driver issues
        // the CONDRV IOCTLs (BIND_HANDLE / SET_SERVER_INFORMATION / GET_HANDLE …)
        // against the console object handles opened above during process init
        // (`ConsoleClientConnectToServer`). Without an answer the console DLL
        // init returns FALSE → ntdll raises DLL_INIT_FAILED → NtTerminateProcess.
        // We don't model the real console protocol; for a console handle, report
        // success with an empty (zeroed) output so the client treats the console
        // as connected and continues. Non-console handles keep the previous
        // unimplemented behaviour.
        WIN64_NTDEVICEIOCONTROLFILE => {
            let file_handle = emu.regs().rcx;
            let rsp = emu.regs().rsp;
            let io_status_block = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
            let io_control_code = emu.maps.read_qword(rsp + 0x30).unwrap_or(0) as u32;
            let output_buffer = emu.maps.read_qword(rsp + 0x48).unwrap_or(0);
            let output_len = emu.maps.read_qword(rsp + 0x50).unwrap_or(0) as u32;
            if emu.console_handles.contains(&file_handle) {
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (4) + pad(4) + Information (8).
                    // Report 0 bytes transferred — we don't synthesize a CONDRV
                    // reply payload, and writing into the (possibly stack-based)
                    // output buffer risks clobbering live locals / stack cookies.
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 0u64);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtDeviceIoControlFile console handle 0x{:x} ioctl 0x{:x} out_buf 0x{:x} out_len 0x{:x} (stub → SUCCESS)",
                    nr,
                    file_handle,
                    io_control_code,
                    output_buffer,
                    output_len,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtDeviceIoControlFile handle 0x{:x} ioctl 0x{:x} (unimplemented)",
                    nr,
                    file_handle,
                    io_control_code,
                );
                emu.regs_mut().rax = STATUS_NOT_IMPLEMENTED;
            }
        }
        // Thread-pool object creation during process init: the pool manager
        // (`TpAllocPool` / `TppWorkerThread`) creates an I/O completion port,
        // a worker factory bound to it, and wait-completion packets. Returning
        // STATUS_NOT_IMPLEMENTED for any of them makes the manager retry forever
        // (an observable ~280K-instruction loop that never converges under
        // SSDT — IoCompletion → WorkerFactory → WaitCompletionPacket → cleanup →
        // repeat). Hand out a fake handle so the pool initializes and init
        // proceeds. All three return the OUT PHANDLE in rcx.
        WIN64_NTCREATEIOCOMPLETION
        | WIN64_NTCREATEWAITCOMPLETIONPACKET
        | WIN64_NTCREATEWORKERFACTORY => {
            let handle_out = emu.regs().rcx;
            if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            } else {
                let h = crate::syscall::windows::syscall64::sync::next_handle();
                let _ = emu.maps.write_qword(handle_out, h);
                log_orange!(
                    emu,
                    "syscall 0x{:x}: {} out: 0x{:x} → handle 0x{:x} (stub → SUCCESS)",
                    nr,
                    what_syscall(real_nr),
                    handle_out,
                    h,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
        }
        // Thread-pool worker-factory configuration / teardown. After creating a
        // worker factory the pool manager configures it
        // (`NtSetInformationWorkerFactory`: min/max threads, binding info) and,
        // on a probe path, tears it down (`NtShutdownWorkerFactory`). Neither
        // returns data we must synthesize; STATUS_SUCCESS lets the pool setup
        // run to completion instead of looping on STATUS_NOT_IMPLEMENTED.
        WIN64_NTSETINFORMATIONWORKERFACTORY | WIN64_NTSHUTDOWNWORKERFACTORY => {
            log_orange!(
                emu,
                "syscall 0x{:x}: {} (stub → SUCCESS)",
                nr,
                what_syscall(real_nr),
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtQueryWnfStateData` (0x170): the loader / kernelbase poll WNF
        // (Windows Notification Facility) state names during init (feature
        // flags, telemetry opt-in, etc.). Returning STATUS_NOT_IMPLEMENTED left
        // them retrying. Report an empty, stable state (ChangeStamp=0, 0 bytes)
        // with SUCCESS so callers treat the state as "present but empty/default"
        // and move on.
        // Args: rcx=StateName*, rdx=TypeId, r8=ExplicitScope, r9=ChangeStamp*(out),
        //       [rsp+0x28]=Buffer(out), [rsp+0x30]=BufferSize*(in/out).
        WIN64_NTQUERYWNFSTATEDATA => {
            let state_name_ptr = emu.regs().rcx;
            let change_stamp_ptr = emu.regs().r9;
            let rsp = emu.regs().rsp;
            let buffer = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
            let bufsize_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);
            let state_name = if state_name_ptr != 0 {
                emu.maps.read_qword(state_name_ptr).unwrap_or(0)
            } else {
                0
            };
            let in_size = if bufsize_ptr != 0 && emu.maps.is_mapped(bufsize_ptr) {
                emu.maps.read_dword(bufsize_ptr).unwrap_or(0)
            } else {
                0
            };
            if change_stamp_ptr != 0 && emu.maps.is_mapped(change_stamp_ptr) {
                let _ = emu.maps.write_dword(change_stamp_ptr, 0);
            }
            if bufsize_ptr != 0 && emu.maps.is_mapped(bufsize_ptr) {
                let _ = emu.maps.write_dword(bufsize_ptr, 0);
            }
            log_orange!(
                emu,
                "syscall 0x{:x}: NtQueryWnfStateData state=0x{:x} buf=0x{:x} in_size=0x{:x} (stub → SUCCESS, empty)",
                nr,
                state_name,
                buffer,
                in_size,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtSetWnfProcessNotificationEvent` (0x1c5): Win11's ntdll registers a
        // per-process event the kernel signals on any subscribed WNF state
        // change (`RtlpInitializeWnf` / the thread-pool WNF subscription path).
        // Returning STATUS_NOT_IMPLEMENTED made the caller treat the
        // subscription as failed and busy-retry the whole dance (query WNF →
        // create event → create wait-completion-packet → set-notify → tear
        // down) forever — the WorkerFactory churn visible during init. Win2022's
        // ntdll doesn't take this path, which is why only Win11 hangs here.
        // Report SUCCESS so the subscription "registers" and init continues; we
        // never signal the event, which is fine — no subscribed state changes.
        WIN64_NTSETWNFPROCESSNOTIFICATIONEVENT => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtSetWnfProcessNotificationEvent (stub → SUCCESS)",
                real_nr,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtSubscribeWnfStateChange` (0x1cd) / `NtUnsubscribeWnfStateChange`
        // (0x1e0): Win11's `RtlSubscribeWnfStateChangeNotification` (reached
        // from `RtlRegisterFeatureConfigurationChangeNotification` during init)
        // subscribes to the feature-configuration-change WNF state. The outer
        // registration retries the whole subscribe in a tight loop until it
        // returns success, so STATUS_NOT_IMPLEMENTED hung here. Report SUCCESS
        // (and hand back a non-zero SubscriptionId for subscribe so the caller
        // doesn't treat 0 as "not subscribed"); we never deliver a change
        // notification, which is correct — the state never changes.
        WIN64_NTSUBSCRIBEWNFSTATECHANGE => {
            // rcx=StateName*, rdx=ChangeStamp, r8=EventFilter, r9=SubscriptionId*(out)
            let subscription_id_ptr = emu.regs().r9;
            if subscription_id_ptr != 0 && emu.maps.is_mapped(subscription_id_ptr) {
                let _ = emu.maps.write_qword(subscription_id_ptr, 1);
            }
            log_orange!(
                emu,
                "syscall 0x{:x}: NtSubscribeWnfStateChange (stub → SUCCESS)",
                real_nr,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        WIN64_NTUNSUBSCRIBEWNFSTATECHANGE => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtUnsubscribeWnfStateChange (stub → SUCCESS)",
                real_nr,
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtApphelpCacheControl` (0x4c): kernel32/kernelbase consult the
        // app-compat shim cache during process start (`BasepCheckBadapp`,
        // `BasepCheckAppCompat`, etc.). STATUS_NOT_SUPPORTED tells callers
        // "no shim infrastructure available" so they skip compat-fixup work
        // and continue with the normal load
        // `handle_NtApphelpCacheControl`.
        WIN64_NTAPPHELPCACHECONTROL => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtApphelpCacheControl (stub → NOT_SUPPORTED)",
                nr,
            );
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }
        // `NtOpenThreadToken` (0x24): kernel32/kernelbase open a thread's primary
        // token during `BasepCheckAppCompat`, `BasepCheckWebBlockedFileType`,
        // and several user32/uxtheme init paths. Returning STATUS_NOT_IMPLEMENTED
        // makes callers treat the failure as "this thread has no token attached"
        // and retry against `NtOpenProcessToken` — but the retry doesn't always
        // happen (e.g. user32!CreateWindowExW init), and the unhandled fail
        // propagates back as a DllMain `BOOL FALSE`, which makes ntdll roll
        // back the user32 load and `LoadLibraryA` returns 0. STATUS_NO_TOKEN
        // is the documented "no impersonation token on this thread" answer
        // that callers handle gracefully (skip the token-based check and
        // continue init).
        0x24 => {
            let thread_handle = emu.regs().rcx;
            let access = emu.regs().rdx;
            let open_as_self = emu.regs().r8 as u8;
            let handle_out = emu.regs().r9;
            log_orange!(
                emu,
                "syscall 0x{:x}: NtOpenThreadToken thread: 0x{:x} access: 0x{:x} self: {} out: 0x{:x} (stub → NO_TOKEN)",
                real_nr,
                thread_handle,
                access,
                open_as_self,
                handle_out,
            );
            // Zero the out-handle so callers don't accidentally re-use a
            // stale pointer value.
            if handle_out != 0 && emu.maps.is_mapped(handle_out) {
                let _ = emu.maps.write_qword(handle_out, 0);
            }
            emu.regs_mut().rax = STATUS_NO_TOKEN;
        }
        _ => {
            // Prefer the syscall name we extracted from the loaded ntdll
            // (matches the actual build), falling back to the static table
            // for older builds where the dispatch covers without translation.
            let name = emu
                .syscall_name_by_real
                .get(&real_nr)
                .cloned()
                .unwrap_or_else(|| what_syscall(real_nr));
            log_orange!(emu, "syscall 0x{:x}: {} (unimplemented)", real_nr, name,);
            // Return STATUS_NOT_IMPLEMENTED so callers using `test eax,eax; js` take
            // the error path instead of processing an uninitialized output buffer.
            emu.regs_mut().rax = STATUS_NOT_IMPLEMENTED;
        }
    }
}

/// Maps x64 NT syscall number to the `Nt…` export name (PascalCase).
pub fn what_syscall(sys: u64) -> String {
    match sys {
        0x0 => "NtAccessCheck".to_string(),
        0x1 => "NtWorkerFactoryWorkerReady".to_string(),
        0x2 => "NtAcceptConnectPort".to_string(),
        0x3 => "NtMapUserPhysicalPagesScatter".to_string(),
        0x4 => "NtWaitForSingleObject".to_string(),
        0x5 => "NtCallbackReturn".to_string(),
        0x6 => "NtReadFile".to_string(),
        0x7 => "NtDeviceIoControlFile".to_string(),
        0x8 => "NtWriteFile".to_string(),
        0x9 => "NtRemoveIoCompletion".to_string(),
        0xa => "NtReleaseSemaphore".to_string(),
        0xb => "NtReplyWaitReceivePort".to_string(),
        0xc => "NtReplyPort".to_string(),
        0xd => "NtSetInformationThread".to_string(),
        0xe => "NtSetEvent".to_string(),
        0xf => "NtClose".to_string(),
        0x10 => "NtQueryObject".to_string(),
        0x11 => "NtQueryInformationFile".to_string(),
        0x12 => "NtOpenKey".to_string(),
        0x13 => "NtEnumerateValueKey".to_string(),
        0x14 => "NtFindAtom".to_string(),
        0x15 => "NtQueryDefaultLocale".to_string(),
        0x16 => "NtQueryKey".to_string(),
        0x17 => "NtQueryValueKey".to_string(),
        0x18 => "NtAllocateVirtualMemory".to_string(),
        0x19 => "NtQueryInformationProcess".to_string(),
        0x1a => "NtWaitForMultipleObjects32".to_string(),
        0x1b => "NtWriteFileGather".to_string(),
        0x1c => "NtSetInformationProcess".to_string(),
        0x1d => "NtCreateKey".to_string(),
        0x1e => "NtFreeVirtualMemory".to_string(),
        0x1f => "NtImpersonateClientOfPort".to_string(),
        0x20 => "NtReleaseMutant".to_string(),
        0x21 => "NtQueryInformationToken".to_string(),
        0x22 => "NtRequestWaitReplyPort".to_string(),
        0x23 => "NtQueryVirtualMemory".to_string(),
        0x24 => "NtOpenThreadToken".to_string(),
        0x25 => "NtQueryInformationThread".to_string(),
        0x26 => "NtOpenProcess".to_string(),
        0x27 => "NtSetInformationFile".to_string(),
        0x28 => "NtMapViewOfSection".to_string(),
        0x29 => "NtAccessCheckAndAuditAlarm".to_string(),
        0x2a => "NtUnmapViewOfSection".to_string(),
        0x2b => "NtReplyWaitReceivePortEx".to_string(),
        0x2c => "NtTerminateProcess".to_string(),
        0x2d => "NtSetEventBoostPriority".to_string(),
        0x2e => "NtReadFileScatter".to_string(),
        0x2f => "NtOpenThreadTokenEx".to_string(),
        0x30 => "NtOpenProcessTokenEx".to_string(),
        0x31 => "NtQueryPerformanceCounter".to_string(),
        0x32 => "NtEnumerateKey".to_string(),
        0x33 => "NtOpenFile".to_string(),
        0x34 => "NtDelayExecution".to_string(),
        0x35 => "NtQueryDirectoryFile".to_string(),
        0x36 => "NtQuerySystemInformation".to_string(),
        0x37 => "NtOpenSection".to_string(),
        0x38 => "NtQueryTimer".to_string(),
        0x39 => "NtFsControlFile".to_string(),
        0x3a => "NtWriteVirtualMemory".to_string(),
        0x3b => "NtCloseObjectAuditAlarm".to_string(),
        0x3c => "NtDuplicateObject".to_string(),
        0x3d => "NtQueryAttributesFile".to_string(),
        0x3e => "NtClearEvent".to_string(),
        0x3f => "NtReadVirtualMemory".to_string(),
        0x40 => "NtOpenEvent".to_string(),
        0x41 => "NtAdjustPrivilegesToken".to_string(),
        0x42 => "NtDuplicateToken".to_string(),
        0x43 => "NtContinue".to_string(),
        0x44 => "NtQueryDefaultUILanguage".to_string(),
        0x45 => "NtQueueApcThread".to_string(),
        0x46 => "NtYieldExecution".to_string(),
        0x47 => "NtAddAtom".to_string(),
        0x48 => "NtCreateEvent".to_string(),
        0x49 => "NtQueryVolumeInformationFile".to_string(),
        0x4a => "NtCreateSection".to_string(),
        0x4b => "NtFlushBuffersFile".to_string(),
        0x4c => "NtApphelpCacheControl".to_string(),
        0x4d => "NtCreateProcessEx".to_string(),
        0x4e => "NtCreateThread".to_string(),
        0x4f => "NtIsProcessInJob".to_string(),
        0x50 => "NtProtectVirtualMemory".to_string(),
        0x51 => "NtQuerySection".to_string(),
        0x52 => "NtResumeThread".to_string(),
        0x53 => "NtTerminateThread".to_string(),
        0x54 => "NtReadRequestData".to_string(),
        0x55 => "NtCreateFile".to_string(),
        0x56 => "NtQueryEvent".to_string(),
        0x57 => "NtWriteRequestData".to_string(),
        0x58 => "NtOpenDirectoryObject".to_string(),
        0x59 => "NtAccessCheckByTypeAndAuditAlarm".to_string(),
        0x5a => "NtQuerySystemTime".to_string(),
        0x5b => "NtWaitForMultipleObjects".to_string(),
        0x5c => "NtSetInformationObject".to_string(),
        0x5d => "NtCancelIoFile".to_string(),
        0x5e => "NtTraceEvent".to_string(),
        0x5f => "NtPowerInformation".to_string(),
        0x60 => "NtSetValueKey".to_string(),
        0x61 => "NtCancelTimer".to_string(),
        0x62 => "NtSetTimer".to_string(),
        0x63 => "NtAccessCheckByType".to_string(),
        0x64 => "NtAccessCheckByTypeResultList".to_string(),
        0x65 => "NtAccessCheckByTypeResultListAndAuditAlarm".to_string(),
        0x66 => "NtAccessCheckByTypeResultListAndAuditAlarmByHandle".to_string(),
        0x67 => "NtAcquireCrossVmMutant".to_string(),
        0x68 => "NtAcquireProcessActivityReference".to_string(),
        0x69 => "NtAddAtomEx".to_string(),
        0x6a => "NtAddBootEntry".to_string(),
        0x6b => "NtAddDriverEntry".to_string(),
        0x6c => "NtAdjustGroupsToken".to_string(),
        0x6d => "NtAdjustTokenClaimsAndDeviceGroups".to_string(),
        0x6e => "NtAlertResumeThread".to_string(),
        0x6f => "NtAlertThread".to_string(),
        0x70 => "NtAlertThreadByThreadId".to_string(),
        0x71 => "NtAllocateLocallyUniqueId".to_string(),
        0x72 => "NtAllocateReserveObject".to_string(),
        0x73 => "NtAllocateUserPhysicalPages".to_string(),
        0x74 => "NtAllocateUserPhysicalPagesEx".to_string(),
        0x75 => "NtAllocateUuids".to_string(),
        0x76 => "NtAllocateVirtualMemoryEx".to_string(),
        0x77 => "NtAlpcAcceptConnectPort".to_string(),
        0x78 => "NtAlpcCancelMessage".to_string(),
        0x79 => "NtAlpcConnectPort".to_string(),
        0x7a => "NtAlpcConnectPortEx".to_string(),
        0x7b => "NtAlpcCreatePort".to_string(),
        0x7c => "NtAlpcCreatePortSection".to_string(),
        0x7d => "NtAlpcCreateResourceReserve".to_string(),
        0x7e => "NtAlpcCreateSectionView".to_string(),
        0x7f => "NtAlpcCreateSecurityContext".to_string(),
        0x80 => "NtAlpcDeletePortSection".to_string(),
        0x81 => "NtAlpcDeleteResourceReserve".to_string(),
        0x82 => "NtAlpcDeleteSectionView".to_string(),
        0x83 => "NtAlpcDeleteSecurityContext".to_string(),
        0x84 => "NtAlpcDisconnectPort".to_string(),
        0x85 => "NtAlpcImpersonateClientContainerOfPort".to_string(),
        0x86 => "NtAlpcImpersonateClientOfPort".to_string(),
        0x87 => "NtAlpcOpenSenderProcess".to_string(),
        0x88 => "NtAlpcOpenSenderThread".to_string(),
        0x89 => "NtAlpcQueryInformation".to_string(),
        0x8a => "NtAlpcQueryInformationMessage".to_string(),
        0x8b => "NtAlpcRevokeSecurityContext".to_string(),
        0x8c => "NtAlpcSendWaitReceivePort".to_string(),
        0x8d => "NtAlpcSetInformation".to_string(),
        0x8e => "NtAreMappedFilesTheSame".to_string(),
        0x8f => "NtAssignProcessToJobObject".to_string(),
        0x90 => "NtAreMappedFilesTheSame".to_string(),
        0x91 => "NtAssignProcessToJobObject".to_string(),
        0x92 => "NtAssociateWaitCompletionPacket".to_string(),
        0x93 => "NtCallEnclave".to_string(),
        0x94 => "NtCancelIoFileEx".to_string(),
        0x95 => "NtCancelSynchronousIoFile".to_string(),
        0x96 => "NtCancelTimer2".to_string(),
        0x97 => "NtCancelWaitCompletionPacket".to_string(),
        0x98 => "NtChangeProcessState".to_string(),
        0x99 => "NtChangeThreadState".to_string(),
        0x9a => "NtCommitComplete".to_string(),
        0x9b => "NtCommitEnlistment".to_string(),
        0x9c => "NtCommitRegistryTransaction".to_string(),
        0x9d => "NtCommitTransaction".to_string(),
        0x9e => "NtCompactKeys".to_string(),
        0x9f => "NtCompareObjects".to_string(),
        0xa0 => "NtCompareSigningLevels".to_string(),
        0xa1 => "NtCompareTokens".to_string(),
        0xa2 => "NtCompleteConnectPort".to_string(),
        0xa3 => "NtCompressKey".to_string(),
        0xa4 => "NtConnectPort".to_string(),
        0xa5 => "NtContinueEx".to_string(),
        0xa6 => "NtConvertBetweenAuxiliaryCounterAndPerformanceCounter".to_string(),
        0xa7 => "NtCopyFileChunk".to_string(),
        0xa8 => "NtCreateCpuPartition".to_string(),
        0xa9 => "NtCreateCrossVmEvent".to_string(),
        0xaa => "NtCreateCrossVmMutant".to_string(),
        0xab => "NtCreateDebugObject".to_string(),
        0xac => "NtCreateDirectoryObject".to_string(),
        0xad => "NtCreateDirectoryObjectEx".to_string(),
        0xae => "NtCreateEnclave".to_string(),
        0xaf => "NtCreateEnlistment".to_string(),
        0xb0 => "NtCreateEventPair".to_string(),
        0xb1 => "NtCreateIRTimer".to_string(),
        0xb2 => "NtCreateIoCompletion".to_string(),
        0xb3 => "NtCreateIoRing".to_string(),
        0xb4 => "NtCreateJobObject".to_string(),
        0xb5 => "NtCreateJobSet".to_string(),
        0xb6 => "NtCreateKeyTransacted".to_string(),
        0xb7 => "NtCreateKeyedEvent".to_string(),
        0xb8 => "NtCreateLowBoxToken".to_string(),
        0xb9 => "NtCreateMailslotFile".to_string(),
        0xba => "NtCreateMutant".to_string(),
        0xbb => "NtCreateNamedPipeFile".to_string(),
        0xbc => "NtCreatePagingFile".to_string(),
        0xbd => "NtCreatePartition".to_string(),
        0xbe => "NtCreatePort".to_string(),
        0xbf => "NtCreatePrivateNamespace".to_string(),
        0xc0 => "NtCreateProcess".to_string(),
        0xc1 => "NtCreateProcessStateChange".to_string(),
        0xc2 => "NtCreateProfile".to_string(),
        0xc3 => "NtCreateProfileEx".to_string(),
        0xc4 => "NtCreateRegistryTransaction".to_string(),
        0xc5 => "NtCreateResourceManager".to_string(),
        0xc6 => "NtCreateSectionEx".to_string(),
        0xc7 => "NtCreateSemaphore".to_string(),
        0xc8 => "NtCreateSymbolicLinkObject".to_string(),
        0xc9 => "NtCreateThreadEx".to_string(),
        0xca => "NtCreateThreadStateChange".to_string(),
        0xcb => "NtCreateTimer".to_string(),
        0xcc => "NtCreateTimer2".to_string(),
        0xcd => "NtCreateToken".to_string(),
        0xce => "NtCreateTokenEx".to_string(),
        0xcf => "NtCreateTransaction".to_string(),
        0xd0 => "NtCreateTransactionManager".to_string(),
        0xd1 => "NtCreateUserProcess".to_string(),
        0xd2 => "NtCreateWaitCompletionPacket".to_string(),
        0xd3 => "NtCreateWaitablePort".to_string(),
        0xd4 => "NtCreateWnfStateName".to_string(),
        0xd5 => "NtCreateWorkerFactory".to_string(),
        0xd6 => "NtDebugActiveProcess".to_string(),
        0xd7 => "NtDebugContinue".to_string(),
        0xd8 => "NtDeleteAtom".to_string(),
        0xd9 => "NtDeleteBootEntry".to_string(),
        0xda => "NtDeleteDriverEntry".to_string(),
        0xdb => "NtDeleteFile".to_string(),
        0xdc => "NtDeleteKey".to_string(),
        0xdd => "NtDeleteObjectAuditAlarm".to_string(),
        0xde => "NtDeletePrivateNamespace".to_string(),
        0xdf => "NtDeleteValueKey".to_string(),
        0xe0 => "NtDeleteWnfStateData".to_string(),
        0xe1 => "NtDeleteWnfStateName".to_string(),
        0xe2 => "NtDirectGraphicsCall".to_string(),
        0xe3 => "NtDisableLastKnownGood".to_string(),
        0xe4 => "NtDisplayString".to_string(),
        0xe5 => "NtDrawText".to_string(),
        0xe6 => "NtEnableLastKnownGood".to_string(),
        0xe7 => "NtEnumerateBootEntries".to_string(),
        0xe8 => "NtEnumerateDriverEntries".to_string(),
        0xe9 => "NtEnumerateSystemEnvironmentValuesEx".to_string(),
        0xea => "NtEnumerateTransactionObject".to_string(),
        0xeb => "NtExtendSection".to_string(),
        0xec => "NtFilterBootOption".to_string(),
        0xed => "NtFilterToken".to_string(),
        0xee => "NtFilterTokenEx".to_string(),
        0xef => "NtFlushBuffersFileEx".to_string(),
        0xf0 => "NtFlushInstallUILanguage".to_string(),
        0xf1 => "NtFlushInstructionCache".to_string(),
        0xf2 => "NtFlushKey".to_string(),
        0xf3 => "NtFlushProcessWriteBuffers".to_string(),
        0xf4 => "NtFlushVirtualMemory".to_string(),
        0xf5 => "NtFlushWriteBuffer".to_string(),
        0xf6 => "NtFreeUserPhysicalPages".to_string(),
        0xf7 => "NtFreezeRegistry".to_string(),
        0xf8 => "NtFreezeTransactions".to_string(),
        0xf9 => "NtGetCachedSigningLevel".to_string(),
        0xfa => "NtGetCompleteWnfStateSubscription".to_string(),
        0xfb => "NtGetContextThread".to_string(),
        0xfc => "NtGetCurrentProcessorNumber".to_string(),
        0xfd => "NtGetCurrentProcessorNumberEx".to_string(),
        0xfe => "NtGetDevicePowerState".to_string(),
        0xff => "NtGetMUIRegistryInfo".to_string(),
        0x100 => "NtGetNextProcess".to_string(),
        0x101 => "NtGetNextThread".to_string(),
        0x102 => "NtGetNlsSectionPtr".to_string(),
        0x103 => "NtGetNotificationResourceManager".to_string(),
        0x104 => "NtGetWriteWatch".to_string(),
        0x105 => "NtImpersonateAnonymousToken".to_string(),
        0x106 => "NtImpersonateThread".to_string(),
        0x107 => "NtInitializeEnclave".to_string(),
        0x108 => "NtInitializeNlsFiles".to_string(),
        0x109 => "NtInitializeRegistry".to_string(),
        0x10a => "NtInitiatePowerAction".to_string(),
        0x10b => "NtIsSystemResumeAutomatic".to_string(),
        0x10c => "NtIsUILanguageComitted".to_string(),
        0x10d => "NtListenPort".to_string(),
        0x10e => "NtLoadDriver".to_string(),
        0x10f => "NtLoadEnclaveData".to_string(),
        0x110 => "NtLoadKey".to_string(),
        0x111 => "NtLoadKey2".to_string(),
        0x112 => "NtLoadKey3".to_string(),
        0x113 => "NtLoadKeyEx".to_string(),
        0x114 => "NtLockFile".to_string(),
        0x115 => "NtLockProductActivationKeys".to_string(),
        0x116 => "NtLockRegistryKey".to_string(),
        0x117 => "NtLockVirtualMemory".to_string(),
        0x118 => "NtMakePermanentObject".to_string(),
        0x119 => "NtMakeTemporaryObject".to_string(),
        0x11a => "NtManageHotPatch".to_string(),
        0x11b => "NtManagePartition".to_string(),
        0x11c => "NtMapCMFModule".to_string(),
        0x11d => "NtMapUserPhysicalPages".to_string(),
        0x11e => "NtMapViewOfSectionEx".to_string(),
        0x11f => "NtModifyBootEntry".to_string(),
        0x120 => "NtModifyDriverEntry".to_string(),
        0x121 => "NtNotifyChangeDirectoryFile".to_string(),
        0x122 => "NtNotifyChangeDirectoryFileEx".to_string(),
        0x123 => "NtNotifyChangeKey".to_string(),
        0x124 => "NtNotifyChangeMultipleKeys".to_string(),
        0x125 => "NtNotifyChangeSession".to_string(),
        0x126 => "NtOpenCpuPartition".to_string(),
        0x127 => "NtOpenEnlistment".to_string(),
        0x128 => "NtOpenEventPair".to_string(),
        0x129 => "NtOpenIoCompletion".to_string(),
        0x12a => "NtOpenJobObject".to_string(),
        0x12b => "NtOpenKeyEx".to_string(),
        0x12c => "NtOpenKeyTransacted".to_string(),
        0x12d => "NtOpenKeyTransactedEx".to_string(),
        0x12e => "NtOpenKeyedEvent".to_string(),
        0x12f => "NtOpenMutant".to_string(),
        0x130 => "NtOpenObjectAuditAlarm".to_string(),
        0x131 => "NtOpenPartition".to_string(),
        0x132 => "NtOpenPrivateNamespace".to_string(),
        0x133 => "NtOpenProcessToken".to_string(),
        0x134 => "NtOpenRegistryTransaction".to_string(),
        0x135 => "NtOpenResourceManager".to_string(),
        0x136 => "NtOpenSemaphore".to_string(),
        0x137 => "NtOpenSession".to_string(),
        0x138 => "NtOpenSymbolicLinkObject".to_string(),
        0x139 => "NtOpenThread".to_string(),
        0x13a => "NtOpenTimer".to_string(),
        0x13b => "NtOpenTransaction".to_string(),
        0x13c => "NtOpenTransactionManager".to_string(),
        0x13d => "NtPlugPlayControl".to_string(),
        0x13e => "NtPrePrepareComplete".to_string(),
        0x13f => "NtPrePrepareEnlistment".to_string(),
        0x140 => "NtPrepareComplete".to_string(),
        0x141 => "NtPrepareEnlistment".to_string(),
        0x142 => "NtPrivilegeCheck".to_string(),
        0x143 => "NtPrivilegeObjectAuditAlarm".to_string(),
        0x144 => "NtPrivilegedServiceAuditAlarm".to_string(),
        0x145 => "NtPropagationComplete".to_string(),
        0x146 => "NtPropagationFailed".to_string(),
        0x147 => "NtPssCaptureVaSpaceBulk".to_string(),
        0x148 => "NtPulseEvent".to_string(),
        0x149 => "NtQueryAuxiliaryCounterFrequency".to_string(),
        0x14a => "NtQueryBootEntryOrder".to_string(),
        0x14b => "NtQueryBootOptions".to_string(),
        0x14c => "NtQueryDebugFilterState".to_string(),
        0x14d => "NtQueryDirectoryFileEx".to_string(),
        0x14e => "NtQueryDirectoryObject".to_string(),
        0x14f => "NtQueryDriverEntryOrder".to_string(),
        0x150 => "NtQueryEaFile".to_string(),
        0x151 => "NtQueryFullAttributesFile".to_string(),
        0x152 => "NtQueryInformationAtom".to_string(),
        0x153 => "NtQueryInformationByName".to_string(),
        0x154 => "NtQueryInformationCpuPartition".to_string(),
        0x155 => "NtQueryInformationEnlistment".to_string(),
        0x156 => "NtQueryInformationJobObject".to_string(),
        0x157 => "NtQueryInformationPort".to_string(),
        0x158 => "NtQueryInformationResourceManager".to_string(),
        0x159 => "NtQueryInformationTransaction".to_string(),
        0x15a => "NtQueryInformationTransactionManager".to_string(),
        0x15b => "NtQueryInformationWorkerFactory".to_string(),
        0x15c => "NtQueryInstallUILanguage".to_string(),
        0x15d => "NtQueryIntervalProfile".to_string(),
        0x15e => "NtQueryIoCompletion".to_string(),
        0x15f => "NtQueryIoRingCapabilities".to_string(),
        0x160 => "NtQueryLicenseValue".to_string(),
        0x161 => "NtQueryMultipleValueKey".to_string(),
        0x162 => "NtQueryMutant".to_string(),
        0x163 => "NtQueryOpenSubKeys".to_string(),
        0x164 => "NtQueryOpenSubKeysEx".to_string(),
        0x165 => "NtQueryPortInformationProcess".to_string(),
        0x166 => "NtQueryQuotaInformationFile".to_string(),
        0x167 => "NtQuerySecurityAttributesToken".to_string(),
        0x168 => "NtQuerySecurityObject".to_string(),
        0x169 => "NtQuerySecurityPolicy".to_string(),
        0x16a => "NtQuerySemaphore".to_string(),
        0x16b => "NtQuerySymbolicLinkObject".to_string(),
        0x16c => "NtQuerySystemEnvironmentValue".to_string(),
        0x16d => "NtQuerySystemEnvironmentValueEx".to_string(),
        0x16e => "NtQuerySystemInformationEx".to_string(),
        0x16f => "NtQueryTimerResolution".to_string(),
        0x170 => "NtQueryWnfStateData".to_string(),
        0x171 => "NtQueryWnfStateNameInformation".to_string(),
        0x172 => "NtQueueApcThreadEx".to_string(),
        0x173 => "NtQueueApcThreadEx2".to_string(),
        0x174 => "NtRaiseException".to_string(),
        0x175 => "NtRaiseHardError".to_string(),
        0x176 => "NtReadOnlyEnlistment".to_string(),
        0x177 => "NtReadVirtualMemoryEx".to_string(),
        0x178 => "NtRecoverEnlistment".to_string(),
        0x179 => "NtRecoverResourceManager".to_string(),
        0x17a => "NtRecoverTransactionManager".to_string(),
        0x17b => "NtRegisterProtocolAddressInformation".to_string(),
        0x17c => "NtRegisterThreadTerminatePort".to_string(),
        0x17d => "NtReleaseKeyedEvent".to_string(),
        0x17e => "NtReleaseWorkerFactoryWorker".to_string(),
        0x17f => "NtRemoveIoCompletionEx".to_string(),
        0x180 => "NtRemoveProcessDebug".to_string(),
        0x181 => "NtRenameKey".to_string(),
        0x182 => "NtRenameTransactionManager".to_string(),
        0x183 => "NtReplaceKey".to_string(),
        0x184 => "NtReplacePartitionUnit".to_string(),
        0x185 => "NtReplyWaitReplyPort".to_string(),
        0x186 => "NtRequestPort".to_string(),
        0x187 => "NtResetEvent".to_string(),
        0x188 => "NtResetWriteWatch".to_string(),
        0x189 => "NtRestoreKey".to_string(),
        0x18a => "NtResumeProcess".to_string(),
        0x18b => "NtRevertContainerImpersonation".to_string(),
        0x18c => "NtRollbackComplete".to_string(),
        0x18d => "NtRollbackEnlistment".to_string(),
        0x18e => "NtRollbackRegistryTransaction".to_string(),
        0x18f => "NtRollbackTransaction".to_string(),
        0x190 => "NtRollforwardTransactionManager".to_string(),
        0x191 => "NtSaveKey".to_string(),
        0x192 => "NtSaveKeyEx".to_string(),
        0x193 => "NtSaveMergedKeys".to_string(),
        0x194 => "NtSecureConnectPort".to_string(),
        0x195 => "NtSerializeBoot".to_string(),
        0x196 => "NtSetBootEntryOrder".to_string(),
        0x197 => "NtSetBootOptions".to_string(),
        0x198 => "NtSetCachedSigningLevel".to_string(),
        0x199 => "NtSetCachedSigningLevel2".to_string(),
        0x19a => "NtSetContextThread".to_string(),
        0x19b => "NtSetDebugFilterState".to_string(),
        0x19c => "NtSetDefaultHardErrorPort".to_string(),
        0x19d => "NtSetDefaultLocale".to_string(),
        0x19e => "NtSetDefaultUILanguage".to_string(),
        0x19f => "NtSetDriverEntryOrder".to_string(),
        0x1a0 => "NtSetEaFile".to_string(),
        0x1a1 => "NtSetEventEx".to_string(),
        0x1a2 => "NtSetHighEventPair".to_string(),
        0x1a3 => "NtSetHighWaitLowEventPair".to_string(),
        0x1a4 => "NtSetIRTimer".to_string(),
        0x1a5 => "NtSetInformationCpuPartition".to_string(),
        0x1a6 => "NtSetInformationDebugObject".to_string(),
        0x1a7 => "NtSetInformationEnlistment".to_string(),
        0x1a8 => "NtSetInformationIoRing".to_string(),
        0x1a9 => "NtSetInformationJobObject".to_string(),
        0x1aa => "NtSetInformationKey".to_string(),
        0x1ab => "NtSetInformationResourceManager".to_string(),
        0x1ac => "NtSetInformationSymbolicLink".to_string(),
        0x1ad => "NtSetInformationToken".to_string(),
        0x1ae => "NtSetInformationTransaction".to_string(),
        0x1af => "NtSetInformationTransactionManager".to_string(),
        0x1b0 => "NtSetInformationVirtualMemory".to_string(),
        0x1b1 => "NtSetInformationWorkerFactory".to_string(),
        0x1b2 => "NtSetIntervalProfile".to_string(),
        0x1b3 => "NtSetIoCompletion".to_string(),
        0x1b4 => "NtSetIoCompletionEx".to_string(),
        0x1b5 => "NtSetLdtEntries".to_string(),
        0x1b6 => "NtSetLowEventPair".to_string(),
        0x1b7 => "NtSetLowWaitHighEventPair".to_string(),
        0x1b8 => "NtSetQuotaInformationFile".to_string(),
        0x1b9 => "NtSetSecurityObject".to_string(),
        0x1ba => "NtSetSystemEnvironmentValue".to_string(),
        0x1bb => "NtSetSystemEnvironmentValueEx".to_string(),
        0x1bc => "NtSetSystemInformation".to_string(),
        0x1bd => "NtSetSystemPowerState".to_string(),
        0x1be => "NtSetSystemTime".to_string(),
        0x1bf => "NtSetThreadExecutionState".to_string(),
        0x1c0 => "NtSetTimer2".to_string(),
        0x1c1 => "NtSetTimerEx".to_string(),
        0x1c2 => "NtSetTimerResolution".to_string(),
        0x1c3 => "NtSetUuidSeed".to_string(),
        0x1c4 => "NtSetVolumeInformationFile".to_string(),
        0x1c5 => "NtSetWnfProcessNotificationEvent".to_string(),
        0x1c6 => "NtShutdownSystem".to_string(),
        0x1c7 => "NtShutdownWorkerFactory".to_string(),
        0x1c8 => "NtSignalAndWaitForSingleObject".to_string(),
        0x1c9 => "NtSinglePhaseReject".to_string(),
        0x1ca => "NtStartProfile".to_string(),
        0x1cb => "NtStopProfile".to_string(),
        0x1cc => "NtSubmitIoRing".to_string(),
        0x1cd => "NtSubscribeWnfStateChange".to_string(),
        0x1ce => "NtSuspendProcess".to_string(),
        0x1cf => "NtSuspendThread".to_string(),
        0x1d0 => "NtSystemDebugControl".to_string(),
        0x1d1 => "NtTerminateEnclave".to_string(),
        0x1d2 => "NtTerminateJobObject".to_string(),
        0x1d3 => "NtTestAlert".to_string(),
        0x1d4 => "NtThawRegistry".to_string(),
        0x1d5 => "NtThawTransactions".to_string(),
        0x1d6 => "NtTraceControl".to_string(),
        0x1d7 => "NtTranslateFilePath".to_string(),
        0x1d8 => "NtUmsThreadYield".to_string(),
        0x1d9 => "NtUnloadDriver".to_string(),
        0x1da => "NtUnloadKey".to_string(),
        0x1db => "NtUnloadKey2".to_string(),
        0x1dc => "NtUnloadKeyEx".to_string(),
        0x1dd => "NtUnlockFile".to_string(),
        0x1de => "NtUnlockVirtualMemory".to_string(),
        0x1df => "NtUnmapViewOfSectionEx".to_string(),
        0x1e0 => "NtUnsubscribeWnfStateChange".to_string(),
        0x1e1 => "NtUpdateWnfStateData".to_string(),
        0x1e2 => "NtVdmControl".to_string(),
        0x1e3 => "NtWaitForAlertByThreadId".to_string(),
        0x1e4 => "NtWaitForDebugEvent".to_string(),
        0x1e5 => "NtWaitForKeyedEvent".to_string(),
        0x1e6 => "NtWaitForWorkViaWorkerFactory".to_string(),
        0x1e7 => "NtWaitHighEventPair".to_string(),
        0x1e8 => "NtWaitLowEventPair".to_string(),
        _ => format!("NtSyscall_unknown(0x{:x})", sys),
    }
}
