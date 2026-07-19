use crate::emu;

/// Per Windows convention, when `lpProcName` has the high bit set, the
/// caller is passing an ordinal export, not a name pointer. The ordinal
/// value itself is the low word.
const ORDINAL_MASK: u64 = 0xFFFF_0000;

pub fn GetProcAddress(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetProcAddress cannot read the handle") as u64;
    let func_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetProcAddress cannot read the func name") as u64;

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let is_ordinal = (func_ptr & ORDINAL_MASK) != 0;

    let (resolved, display_module, display_name) = if is_ordinal {
        let ordinal = (func_ptr & 0xFFFF) as u32;
        resolve_ordinal_via_registry_or_scanner(emu, hndl, ordinal)
    } else {
        let func = emu.maps.read_string(func_ptr).to_lowercase();
        resolve_name_via_registry_or_scanner(emu, hndl, &func)
    };

    emu.regs_mut().rax = resolved;
    if resolved != 0 {
        log_red!(
            emu,
            "kernel32!GetProcAddress  `{}!{}` =0x{:x}",
            display_module,
            display_name,
            emu.regs().get_eax() as u32
        );
    } else {
        log::warn!("kernel32!GetProcAddress error searching 0x{:x}", func_ptr);
    }
}

fn resolve_name_via_registry_or_scanner(
    emu: &mut emu::Emu,
    hndl: u64,
    name: &str,
) -> (u64, String, String) {
    if let Some(module) = emu.export_indexes.get_by_base(hndl) {
        let addr = emu.export_indexes.resolve_name_by_base(hndl, name);
        if addr != 0 {
            let display = module_display_name_for(module, name).unwrap_or_else(|| name.to_string());
            return (addr, module.module_name.clone(), display);
        }
        return (0, module.module_name.clone(), name.to_string());
    }

    let addr =
        crate::api::windows::winapi32::kernel32::resolver::resolve_api_name_in_module(
            emu, "", name,
        );
    (addr, String::new(), name.to_string())
}

fn resolve_ordinal_via_registry_or_scanner(
    emu: &mut emu::Emu,
    hndl: u64,
    ordinal: u32,
) -> (u64, String, String) {
    if let Some(module) = emu.export_indexes.get_by_base(hndl) {
        let addr = emu.export_indexes.resolve_ordinal_by_base(hndl, ordinal);
        let display = module_display_name_for_ordinal(module, ordinal)
            .unwrap_or_else(|| format!("#{}", ordinal));
        if addr != 0 {
            return (addr, module.module_name.clone(), display);
        }
        return (0, module.module_name.clone(), display);
    }
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