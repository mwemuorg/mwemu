use crate::emu;

/// Per Windows convention, when `lpProcName` has the high bit set (0xFFFF0000
/// mask), the caller is passing an ordinal export, not a name pointer. The
/// ordinal value itself is the low word.
const ORDINAL_MASK: u64 = 0xFFFF_0000_0000_0000;

pub fn GetProcAddress(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let func_ptr = emu.regs().rdx;
    // ORDINAL_MASK is the standard macro. If the caller passed an ordinal we
    // use the low word; otherwise the argument is a name pointer.
    let is_ordinal = (func_ptr & ORDINAL_MASK) != 0;

    let (resolved, display_module, display_name) = if is_ordinal {
        let ordinal = (func_ptr & 0xFFFF) as u32;
        resolve_ordinal_via_registry_or_scanner(emu, hndl, ordinal, func_ptr)
    } else {
        let func = emu.maps.read_string(func_ptr).to_lowercase();
        resolve_name_via_registry_or_scanner(emu, hndl, &func, func_ptr)
    };

    emu.regs_mut().rax = resolved;
    if emu.cfg.verbose >= 1 {
        if resolved != 0 {
            log_red!(
                emu,
                "kernel32!GetProcAddress  `{}!{}` =0x{:x}",
                display_module,
                display_name,
                emu.regs().rax
            );
        } else {
            log::warn!(
                "kernel32!GetProcAddress 0x{:x} (hndl 0x{:x}) unresolved",
                func_ptr,
                hndl
            );
        }
    }
    if resolved == 0 {
        log::warn!("kernel32!GetProcAddress error searching 0x{:x}", func_ptr);
    }
}

fn resolve_name_via_registry_or_scanner(
    emu: &mut emu::Emu,
    hndl: u64,
    name: &str,
    _func_ptr: u64,
) -> (u64, String, String) {
    // 1) Honor the supplied HMODULE through the registry.
    if let Some(module) = emu.export_indexes.get_by_base(hndl) {
        let addr = emu
            .export_indexes
            .resolve_name_by_base(hndl, name);
        if addr != 0 {
            // Recover the display name (preserved case) for the log.
            let display = module_display_name_for(module, name).unwrap_or_else(|| name.to_string());
            return (addr, module.module_name.clone(), display);
        }
        // The handle is known but this module does not export `name`. Honor
        // the handle — do not silently fall back to a global search.
        return (0, module.module_name.clone(), name.to_string());
    }

    // 2) Fallback to the global resolver + PEB scanner.
    let addr = crate::api::windows::winapi64::kernel32::resolver::resolve_api_name_in_module(
        emu, "", name,
    );
    // We don't know the module name for the scanner fallback; use a placeholder.
    (addr, String::new(), name.to_string())
}

fn resolve_ordinal_via_registry_or_scanner(
    emu: &mut emu::Emu,
    hndl: u64,
    ordinal: u32,
    _func_ptr: u64,
) -> (u64, String, String) {
    if let Some(module) = emu.export_indexes.get_by_base(hndl) {
        let addr = emu
            .export_indexes
            .resolve_ordinal_by_base(hndl, ordinal);
        let display = module_display_name_for_ordinal(module, ordinal)
            .unwrap_or_else(|| format!("#{}", ordinal));
        if addr != 0 {
            return (addr, module.module_name.clone(), display);
        }
        return (0, module.module_name.clone(), display);
    }

    // Fallback: use the global resolver which scans all modules by ordinal-ish
    // semantics. Ordinals are per-module so the global fallback is only an
    // approximation — return 0 if not indexed.
    (0, String::new(), format!("#{}", ordinal))
}

fn module_display_name_for(
    module: &crate::api::windows::export_index::ModuleExportIndex,
    name_lc: &str,
) -> Option<String> {
    let lc = name_lc.to_ascii_lowercase();
    if let Some(ord_idx) = module.by_name.get(&lc) {
        for (name, idx) in module.display_names_for_iter() {
            if idx == *ord_idx {
                return Some(name);
            }
        }
    }
    None
}

fn module_display_name_for_ordinal(
    module: &crate::api::windows::export_index::ModuleExportIndex,
    ordinal: u32,
) -> Option<String> {
    let idx = ordinal.checked_sub(module.export_base)?;
    for (name, ord_idx) in module.display_names_for_iter() {
        if ord_idx == idx {
            return Some(name);
        }
    }
    None
}