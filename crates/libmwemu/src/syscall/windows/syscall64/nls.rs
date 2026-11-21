//! NLS (National Language Support) syscalls.
//!
//! ntdll's `RtlInitNlsTables` and friends drive every ANSI↔Unicode conversion
//! during process init — including the byte→wide step that builds dependency
//! UNICODE_STRINGs in `LdrpFindKnownDll`. Without working NLS data, those
//! conversions return all-zero buffers and the loader terminates with
//! `STATUS_DLL_NOT_FOUND`.
//!
//! Real Windows hands ntdll real `.NLS` files (cross-referenced binary tables
//! covering codepage and unicode-casing data); we mirror that by reading the
//! files placed in `cfg.maps_folder` (`locale.nls`, `C_1252.NLS`, etc.) and
//! mapping them at the addresses ntdll expects.

use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

/// Default LCID we report — `0x407` is `de-DE`, but every Windows we've seen
/// here works with `0x409` (en-US).
/// matches the locale.nls we ship. The exact value doesn't affect ASCII DLL
/// loading.
const DEFAULT_LOCALE_ID: u32 = 0x0409;

const NLS_SECTION_TYPE_CODEPAGE: u32 = 11;

/// `NtInitializeNlsFiles(PVOID *BaseAddress, PLCID DefaultLocaleId, PLARGE_INTEGER DefaultCasingTableSize)`.
///
/// Loads `locale.nls` from `cfg.maps_folder` and writes its mapped base back
/// to the caller. ntdll uses this to find the Unicode case-mapping tables.
pub fn nt_initialize_nls_files(emu: &mut Emu) {
    let call = WinSyscall;
    let [base_address_out, locale_id_out, casing_size_out] = api_try!(emu, call.args(emu));
    if !emu.maps.validate_write_range(base_address_out, 8)
        || !emu.maps.validate_write_range(locale_id_out, 4)
        || (casing_size_out != 0 && !emu.maps.validate_write_range(casing_size_out, 8))
    {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }

    let path = emu.cfg.get_maps_folder("locale.nls");
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => {
            log::warn!(
                "NtInitializeNlsFiles: missing {} — falling back to STATUS_FILE_INVALID",
                path
            );
            emu.regs_mut().rax = STATUS_FILE_INVALID;
            return;
        }
    };

    let base = match map_nls_blob(emu, "locale.nls", &bytes) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_FILE_INVALID;
            return;
        }
    };

    emu.maps.write_qword(base_address_out, base);
    emu.maps.write_dword(locale_id_out, DEFAULT_LOCALE_ID);
    if casing_size_out != 0 {
        emu.maps.write_qword(casing_size_out, bytes.len() as u64);
    }

    log::trace!(
        "NtInitializeNlsFiles: locale.nls mapped at 0x{:x} ({} bytes), LCID = 0x{:x}",
        base,
        bytes.len(),
        DEFAULT_LOCALE_ID,
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtGetNlsSectionPtr(ULONG SectionType, ULONG SectionData, PVOID ContextData,
///                    PVOID *SectionPointer, PSIZE_T SectionSize)`.
///
/// `SectionType == 11` is the codepage-section variant; `SectionData` holds
/// the codepage number (1252, 437, …). Maps the corresponding `C_<n>.NLS`
/// from `cfg.maps_folder`.
pub fn nt_get_nls_section_ptr(emu: &mut Emu) {
    let call = WinSyscall;
    let [
        section_type,
        section_data,
        context,
        section_ptr_out,
        section_size_out,
    ] = api_try!(emu, call.args(emu));
    if context != 0 {
        return call.set_return(emu, STATUS_INVALID_PARAMETER);
    }
    if section_type as u32 != NLS_SECTION_TYPE_CODEPAGE {
        return call.set_return(emu, STATUS_NOT_SUPPORTED);
    }
    if !emu.maps.validate_write_range(section_ptr_out, 8)
        || (section_size_out != 0 && !emu.maps.validate_write_range(section_size_out, 8))
    {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let section_data = section_data as u32;
    let file_name = format!("C_{}.NLS", section_data);
    let path = emu.cfg.get_maps_folder(&file_name);
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => {
            log::warn!("NtGetNlsSectionPtr: missing {}", path);
            emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            return;
        }
    };

    let base = match map_nls_blob(emu, &file_name.to_lowercase(), &bytes) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            return;
        }
    };

    emu.maps.write_qword(section_ptr_out, base);
    if section_size_out != 0 {
        emu.maps
            .write_qword(section_size_out, page_align_up(bytes.len()) as u64);
    }

    log::trace!(
        "NtGetNlsSectionPtr: {} mapped at 0x{:x} ({} bytes)",
        file_name,
        base,
        bytes.len()
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// Allocate a page-aligned read-only mapping for an NLS blob and copy the
/// file contents into it. Returns the mapped base, or `None` on failure.
/// Reuses an existing map if one already exists for this name.
fn map_nls_blob(emu: &mut Emu, name: &str, bytes: &[u8]) -> Option<u64> {
    let map_name = format!("nls_{}", name);
    if let Some(existing) = emu.maps.get_map_by_name(&map_name) {
        return Some(existing.get_base());
    }

    let size = page_align_up(bytes.len()) as u64;
    let base = emu.maps.alloc(size)?;
    let mem = emu
        .maps
        .create_map(&map_name, base, size, Permission::READ_WRITE)
        .ok()?;
    mem.memcpy(bytes, bytes.len());
    // Drop write permission once populated to mirror real Windows
    // (the NLS section is read-only mapped).
    if let Some(m) = emu.maps.get_map_by_name_mut(&map_name) {
        m.set_permission(Permission::READ);
    }
    Some(base)
}

fn page_align_up(n: usize) -> usize {
    (n + 0xFFF) & !0xFFF
}
