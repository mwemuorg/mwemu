use crate::emu::Emu;
use crate::maps::virtual_memory::MemoryKind;
use crate::winapi::common::memory::{Call, writable_pointer, write_pointer};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::*;

use crate::windows::peb::{peb32, peb64};

pub(crate) fn is_api_set_contract(module: &str) -> bool {
    let module = module.trim().to_ascii_lowercase();
    module.starts_with("api-ms-win-") || module.starts_with("ext-ms-")
}

/// DLLs implementing an API set contract, the host first. ntdll's ApiSet
/// schema (a per-build binary blob) is the authoritative mapping, but the
/// prefixes are stable across builds.
pub(crate) fn api_set_hosts(contract: &str) -> &'static [&'static str] {
    let contract = contract.trim().to_ascii_lowercase();
    let prefix = |p: &str| contract.starts_with(p);
    if prefix("api-ms-win-crt-") {
        &["ucrtbase.dll", "msvcp_win.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-core-rtlsupport-") || prefix("api-ms-win-core-apiquery-") {
        // RtlCompareMemory, RtlUnwind, RtlCaptureContext, ApiSetQueryApiSetPresence
        &["ntdll.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-shcore-") {
        &["shcore.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-eventing-") {
        // ETW user-mode helpers live in ntdll; advapi32 for legacy callers
        &["ntdll.dll", "kernelbase.dll", "advapi32.dll"]
    } else if prefix("api-ms-win-security-") {
        &["sechost.dll", "advapi32.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-service-") {
        &["sechost.dll", "kernelbase.dll"]
    } else {
        &["kernelbase.dll", "kernel32.dll"]
    }
}

pub(crate) fn normalize_library_name(libname: &str, allow_exe_suffix: bool) -> Option<String> {
    // Guest paths use Windows separators even when the host is Unix.
    // Only the basename is looked up inside the host maps folder.
    let mut dll = libname
        .trim()
        .rsplit(['\\', '/'])
        .next()?
        .to_ascii_lowercase();
    if dll.is_empty() || dll.contains(':') {
        return None;
    }

    if allow_exe_suffix {
        if !dll.ends_with(".dll") && !dll.ends_with(".exe") {
            dll.push_str(".dll");
        }
    } else if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    Some(dll)
}

pub(crate) fn module_name_matches(flink_mod: &str, want: &str) -> bool {
    normalize_module_name(flink_mod) == normalize_module_name(want)
}

fn normalize_module_name(module_name: &str) -> String {
    let module_name = module_name.trim().to_ascii_lowercase();
    let module_name = module_name
        .rsplit_once('\\')
        .map(|(_, name)| name)
        .unwrap_or(&module_name);
    let module_name = module_name
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(module_name);
    module_name
        .strip_suffix(".dll")
        .unwrap_or(module_name)
        .to_string()
}

fn module_base(emu: &mut Emu, name_ptr: u64, wide: bool, from_address: bool) -> Option<u64> {
    if name_ptr == 0 {
        return (emu.base != 0).then_some(emu.base);
    }
    if from_address {
        return emu
            .maps
            .allocation_at(name_ptr)
            .filter(|allocation| allocation.kind == MemoryKind::Image)
            .map(|allocation| allocation.base);
    }
    let name = if wide {
        emu.maps.read_wide_string(name_ptr)
    } else {
        emu.maps.read_string(name_ptr)
    };
    let name = normalize_library_name(&name, true)?;
    if let Some(module) = emu.export_indexes.get_by_name(&name) {
        return Some(module.base);
    }
    let map_name = format!("{}.pe", name.strip_suffix(".dll").unwrap_or(&name));
    if let Some(base) = emu
        .maps
        .get_mem2(&name)
        .map(|map| map.get_base())
        .or_else(|| emu.maps.get_mem2(&map_name).map(|map| map.get_base()))
    {
        return Some(base);
    }
    emu.maps.get_mem2("peb")?;
    if emu.cfg.arch.is_64bits() {
        peb64::get_module_base(&name, emu)
    } else {
        peb32::get_module_base(&name, emu)
    }
}

pub(crate) fn get_module_handle(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = Call::Api;
    let flags = if extended { call.arg(emu, 0) as u32 } else { 0 };
    let name = call.arg(emu, usize::from(extended));
    let output = if extended { call.arg(emu, 2) } else { 0 };
    let allowed = GET_MODULE_HANDLE_EX_FLAG_PIN
        | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
        | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS;
    let incompatible = GET_MODULE_HANDLE_EX_FLAG_PIN | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
    let valid_output = !extended || (output != 0 && writable_pointer(emu, output));
    let result = if flags & !allowed != 0 || flags & incompatible == incompatible || !valid_output {
        Err(ERROR_INVALID_PARAMETER)
    } else {
        module_base(
            emu,
            name,
            wide,
            flags & GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS != 0,
        )
        .ok_or(ERROR_MOD_NOT_FOUND)
    };
    let base = match result {
        Ok(base) => base,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    if extended && valid_output {
        write_pointer(emu, output, base);
    }
    log_red!(
        emu,
        "{} name/address:0x{:x} flags:0x{:x} => 0x{:x}",
        api,
        name,
        flags,
        base
    );
    call.finish(
        emu,
        if extended { (base != 0) as u64 } else { base },
        if extended { 3 } else { 1 },
    );
}

/// Guest path of a loaded module; NULL identifies the main image.
pub fn module_path(emu: &Emu, handle: u64) -> Option<String> {
    if handle == 0 || handle == emu.base {
        return Some(main_module_path(emu));
    }
    emu.export_indexes
        .get_by_base(handle)
        .map(|module| format!("C:\\Windows\\System32\\{}", module.module_name))
}

pub fn main_module_path(emu: &Emu) -> String {
    format!("C:\\{}", emu.cfg.exe_name)
}

pub(crate) fn get_module_file_name(emu: &mut Emu, api: &str, wide: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let output = call.arg(emu, 1);
    let capacity = call.arg(emu, 2) as u32 as usize;
    let path = module_path(emu, handle);
    log_red!(
        emu,
        "{} module:0x{:x} output:0x{:x} capacity:{} => {:?}",
        api,
        handle,
        output,
        capacity,
        path
    );
    let result = path.ok_or(ERROR_MOD_NOT_FOUND).and_then(|path| {
        if capacity == 0 {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        let units: Vec<u16> = if wide {
            path.encode_utf16().collect()
        } else {
            path.bytes().map(u16::from).collect()
        };
        let count = units.len().min(capacity - 1);
        let mut bytes = Vec::with_capacity(
            (count + 1)
                * if wide {
                    size_of::<u16>()
                } else {
                    size_of::<u8>()
                },
        );
        for unit in units[..count].iter().copied().chain([0]) {
            if wide {
                bytes.extend_from_slice(&unit.to_le_bytes());
            } else {
                bytes.push(unit as u8);
            }
        }
        if output == 0 || !emu.maps.write_bytes(output, &bytes) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if units.len() >= capacity {
            emu.set_last_error(ERROR_INSUFFICIENT_BUFFER);
            Ok(capacity as u64)
        } else {
            Ok(count as u64)
        }
    });
    let result = match result {
        Ok(length) => length,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.finish(emu, result, 3);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let handler: fn(&mut Emu, &str) = match api {
        "GetModuleHandleA" => |emu, name| get_module_handle(emu, name, false, false),
        "GetModuleHandleW" => |emu, name| get_module_handle(emu, name, true, false),
        "GetModuleHandleExA" => |emu, name| get_module_handle(emu, name, false, true),
        "GetModuleHandleExW" => |emu, name| get_module_handle(emu, name, true, true),
        "GetModuleFileNameA" => |emu, name| get_module_file_name(emu, name, false),
        "GetModuleFileNameW" => |emu, name| get_module_file_name(emu, name, true),
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}
