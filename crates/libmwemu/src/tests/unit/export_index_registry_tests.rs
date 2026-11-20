use rs_header::pe::PeLoader;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::api::windows::export_index::{
    ExportIndexRegistry, IndexedExport, MAX_FORWARDER_DEPTH, ModuleExportIndex,
    normalize_module_name,
};
use crate::maps::mem64::Permission;
use crate::tests::helpers::{call_winapi_export, prepare_winapi};
use crate::windows::constants::errors::{ERROR_MOD_NOT_FOUND, ERROR_PROC_NOT_FOUND};
use crate::windows::peb::{peb32, peb64};

const FIRST_BASE: u64 = 0x1_0000;
const SECOND_BASE: u64 = 0x5_0000;
const REPLACEMENT_BASE: u64 = 0x9_0000;
const FIRST_RVA: u32 = 0x1500;
const SECOND_RVA: u32 = 0x1600;
const EXPORT_BASE: u32 = 5;
const MAX_ORDINAL: u16 = u16::MAX;
const FIRST_NAME_POINTER: u64 = MAX_ORDINAL as u64 + 1;
const NAME_BUFFER_SIZE: u64 = 0x100;
const GUEST_MODULES: [(bool, u64); 2] = [(false, 0x7100_0000), (true, 0x1_7100_0000)];

fn module_index(
    module: &str,
    base: u64,
    export_base: u32,
    entries: &[(&str, ExportTarget)],
) -> ModuleExportIndex {
    let parsed = ExportIndexData {
        export_base,
        number_of_functions: entries.len() as u32,
        ordinal_targets: entries
            .iter()
            .map(|(_, target)| Some(target.clone()))
            .collect(),
        named_exports: entries
            .iter()
            .enumerate()
            .map(|(i, (name, _))| NamedExport {
                name: (*name).into(),
                ordinal_index: i as u32,
            })
            .collect(),
    };
    ModuleExportIndex::from_parsed(module.into(), normalize_module_name(module), base, &parsed)
}

fn direct_module(module: &str, base: u64) -> ModuleExportIndex {
    module_index(
        module,
        base,
        EXPORT_BASE,
        &[
            ("First", ExportTarget::Direct { rva: FIRST_RVA }),
            ("Second", ExportTarget::Direct { rva: SECOND_RVA }),
        ],
    )
}

#[test]
fn registration_indexes_module_name_base_and_export_address() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(direct_module("First.DLL", FIRST_BASE));

    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    for name in ["first", "FIRST.DLL", r"C:\Windows\first.dll"] {
        assert_eq!(registry.get_by_name(name).unwrap().base, FIRST_BASE);
    }
    assert_eq!(
        registry.get_by_base(FIRST_BASE).unwrap().module_name,
        "First.DLL"
    );
    assert!(registry.get_by_name("missing.dll").is_none());
    assert!(registry.get_by_base(SECOND_BASE).is_none());
    assert_eq!(
        registry.resolve_address(FIRST_BASE + FIRST_RVA as u64),
        Some(("first".into(), "First".into()))
    );
    assert_eq!(registry.resolve_address(SECOND_BASE), None);
}

#[test]
fn replacement_removes_old_base_and_preserves_lookup_order() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(direct_module("first.dll", FIRST_BASE));
    registry.register(direct_module("second.dll", SECOND_BASE));
    registry.register(direct_module("FIRST.DLL", REPLACEMENT_BASE));

    assert_eq!(registry.len(), 2);
    assert!(registry.get_by_base(FIRST_BASE).is_none());
    assert!(registry.get_by_base(REPLACEMENT_BASE).is_some());
    assert_eq!(
        registry
            .iter_ordered()
            .map(|m| m.normalized_name.as_str())
            .collect::<Vec<_>>(),
        ["first", "second"]
    );
    assert_eq!(
        registry.resolve_name_global("First"),
        REPLACEMENT_BASE + FIRST_RVA as u64
    );
    assert_eq!(
        registry.resolve_address(FIRST_BASE + FIRST_RVA as u64),
        None
    );
}

#[test]
fn removal_clears_all_indices_and_preserves_other_modules() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(direct_module("first.dll", FIRST_BASE));
    registry.register(direct_module("second.dll", SECOND_BASE));

    assert!(registry.remove(r"C:\Windows\FIRST.DLL"));
    assert!(!registry.remove("first.dll"));
    assert!(registry.get_by_name("first").is_none());
    assert!(registry.get_by_base(FIRST_BASE).is_none());
    assert_eq!(
        registry.resolve_address(FIRST_BASE + FIRST_RVA as u64),
        None
    );
    assert_eq!(registry.resolve_name_in_module("first", "First"), 0);
    assert_eq!(
        registry.resolve_name_global("First"),
        SECOND_BASE + FIRST_RVA as u64
    );
    assert_eq!(
        registry
            .iter_ordered()
            .map(|m| m.normalized_name.as_str())
            .collect::<Vec<_>>(),
        ["second"]
    );
}

#[test]
fn module_name_normalization_handles_paths_case_and_whitespace() {
    for name in [
        "KERNEL32.DLL",
        r"C:\Windows\kernel32.dll",
        "/usr/lib/kernel32.dll",
        "kernel32",
        "  kernel32.DLL  ",
    ] {
        assert_eq!(normalize_module_name(name), "kernel32", "{name}");
    }
}

#[test]
fn registry_lookup_normalizes_export_names() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(direct_module("first.dll", FIRST_BASE));
    for name in ["First", "first", "FIRST"] {
        assert_eq!(
            registry.resolve_name_in_module("first", name),
            FIRST_BASE + FIRST_RVA as u64
        );
    }
}

#[test]
fn rebase_updates_direct_addresses_and_preserves_names_and_forwarders() {
    let mut index = module_index(
        "first.dll",
        FIRST_BASE,
        EXPORT_BASE,
        &[
            ("MixedCase", ExportTarget::Direct { rva: FIRST_RVA }),
            (
                "Forwarded",
                ExportTarget::Forwarder {
                    value: "second.First".into(),
                },
            ),
        ],
    );
    index.rebase(REPLACEMENT_BASE);
    index.rebase(REPLACEMENT_BASE);
    let address = REPLACEMENT_BASE + FIRST_RVA as u64;

    assert_eq!(index.base, REPLACEMENT_BASE);
    assert_eq!(
        index
            .resolve_name("mixedcase")
            .and_then(IndexedExport::direct_address),
        Some(address)
    );
    assert_eq!(
        index
            .resolve_ordinal(EXPORT_BASE)
            .and_then(IndexedExport::direct_address),
        Some(address)
    );
    assert_eq!(index.resolve_address(FIRST_BASE + FIRST_RVA as u64), None);
    assert_eq!(index.resolve_address(address), Some("MixedCase"));
    assert_eq!(
        index.resolve_name("forwarded"),
        Some(&IndexedExport::Forwarder {
            value: "second.First".into()
        })
    );
}

#[test]
fn indexed_export_helpers_distinguish_direct_and_forwarded_targets() {
    let direct = IndexedExport::Direct {
        address: FIRST_BASE,
    };
    let forwarded = IndexedExport::Forwarder {
        value: "second.First".into(),
    };

    assert!(direct.is_direct());
    assert_eq!(direct.direct_address(), Some(FIRST_BASE));
    assert!(!forwarded.is_direct());
    assert_eq!(forwarded.direct_address(), None);
}

#[test]
fn empty_registry_has_no_resolvable_exports() {
    let registry = ExportIndexRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.iter_ordered().count(), 0);
    assert_eq!(registry.resolve_name_in_module("missing", "First"), 0);
    assert_eq!(registry.resolve_name_global("First"), 0);
    assert_eq!(registry.resolve_name_by_base(FIRST_BASE, "First"), 0);
    assert_eq!(
        registry.resolve_ordinal_in_module("missing", EXPORT_BASE),
        0
    );
    assert_eq!(registry.resolve_ordinal_by_base(FIRST_BASE, EXPORT_BASE), 0);
    assert_eq!(registry.resolve_address(FIRST_BASE), None);
}

#[test]
fn module_lookups_do_not_borrow_names_or_ordinals_from_other_modules() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(direct_module("first.dll", FIRST_BASE));
    registry.register(module_index(
        "second.dll",
        SECOND_BASE,
        EXPORT_BASE + 2,
        &[("Other", ExportTarget::Direct { rva: FIRST_RVA })],
    ));
    let address = FIRST_BASE + FIRST_RVA as u64;

    assert_eq!(registry.resolve_name_global("First"), address);
    assert_eq!(
        registry.resolve_ordinal_in_module("first", EXPORT_BASE),
        address
    );
    assert_eq!(registry.resolve_name_in_module("second", "First"), 0);
    assert_eq!(registry.resolve_name_by_base(SECOND_BASE, "First"), 0);
    assert_eq!(registry.resolve_ordinal_in_module("second", EXPORT_BASE), 0);
    assert_eq!(
        registry.resolve_ordinal_by_base(SECOND_BASE, EXPORT_BASE),
        0
    );
    assert_eq!(
        registry.resolve_ordinal_in_module("missing", EXPORT_BASE),
        0
    );
    assert_eq!(
        registry.resolve_ordinal_by_base(REPLACEMENT_BASE, EXPORT_BASE),
        0
    );
    assert_eq!(registry.resolve_name_by_base(REPLACEMENT_BASE, "First"), 0);
    assert_eq!(registry.resolve_name_global("Missing"), 0);
}

#[test]
fn ordinal_lookup_uses_export_base_and_rejects_holes_and_bounds() {
    let mut registry = ExportIndexRegistry::new();
    let mut index = module_index(
        "first.dll",
        FIRST_BASE,
        EXPORT_BASE,
        &[
            ("First", ExportTarget::Direct { rva: FIRST_RVA }),
            ("Hole", ExportTarget::Direct { rva: FIRST_RVA }),
            ("Second", ExportTarget::Direct { rva: SECOND_RVA }),
        ],
    );
    index.by_ordinal[1] = None;
    registry.register(index);
    for (ordinal, expected) in [
        (EXPORT_BASE - 1, 0),
        (EXPORT_BASE, FIRST_BASE + FIRST_RVA as u64),
        (EXPORT_BASE + 1, 0),
        (EXPORT_BASE + 2, FIRST_BASE + SECOND_RVA as u64),
        (EXPORT_BASE + 3, 0),
    ] {
        assert_eq!(
            registry.resolve_ordinal_in_module(r"C:\Windows\FIRST.DLL", ordinal),
            expected,
            "ordinal {ordinal}"
        );
        assert_eq!(
            registry.resolve_ordinal_by_base(FIRST_BASE, ordinal),
            expected,
            "ordinal {ordinal}"
        );
    }
    assert_eq!(registry.resolve_name_in_module("first", "Hole"), 0);
}

#[test]
fn global_lookup_uses_registration_order_and_skips_unresolved_forwarders() {
    let mut registry = ExportIndexRegistry::new();
    registry.register(module_index(
        "forwarded.dll",
        REPLACEMENT_BASE,
        EXPORT_BASE,
        &[(
            "First",
            ExportTarget::Forwarder {
                value: "missing.First".into(),
            },
        )],
    ));
    registry.register(direct_module("first.dll", FIRST_BASE));
    registry.register(direct_module("second.dll", SECOND_BASE));

    assert_eq!(
        registry.resolve_name_global("First"),
        FIRST_BASE + FIRST_RVA as u64
    );
}

#[test]
fn forwarders_resolve_named_and_ordinal_targets() {
    for target in ["second.First".to_string(), format!("second.#{EXPORT_BASE}")] {
        let mut registry = ExportIndexRegistry::new();
        registry.register(module_index(
            "first.dll",
            FIRST_BASE,
            EXPORT_BASE,
            &[(
                "Forwarded",
                ExportTarget::Forwarder {
                    value: target.clone(),
                },
            )],
        ));
        registry.register(direct_module("second.dll", SECOND_BASE));
        let expected = SECOND_BASE + FIRST_RVA as u64;

        assert_eq!(
            registry.resolve_name_in_module("first", "Forwarded"),
            expected,
            "{target}"
        );
        assert_eq!(
            registry.resolve_name_by_base(FIRST_BASE, "Forwarded"),
            expected,
            "{target}"
        );
        assert_eq!(
            registry.resolve_name_global("Forwarded"),
            expected,
            "{target}"
        );
        assert_eq!(
            registry.resolve_ordinal_in_module("first", EXPORT_BASE),
            expected,
            "{target}"
        );
        assert_eq!(
            registry.resolve_ordinal_by_base(FIRST_BASE, EXPORT_BASE),
            expected,
            "{target}"
        );
    }
}

#[test]
fn forwarder_cycles_return_zero() {
    for (case, first_target, second_target) in [
        ("named", "second.Entry".into(), "first.Entry".into()),
        (
            "ordinal",
            format!("second.#{EXPORT_BASE}"),
            format!("first.#{EXPORT_BASE}"),
        ),
        (
            "mixed",
            "second.Entry".into(),
            format!("first.#{EXPORT_BASE}"),
        ),
    ] {
        let mut registry = ExportIndexRegistry::new();
        for (module, base, target) in [
            ("first.dll", FIRST_BASE, first_target),
            ("second.dll", SECOND_BASE, second_target),
        ] {
            registry.register(module_index(
                module,
                base,
                EXPORT_BASE,
                &[("Entry", ExportTarget::Forwarder { value: target })],
            ));
        }

        assert_eq!(
            registry.resolve_name_in_module("first", "Entry"),
            0,
            "{case}"
        );
        assert_eq!(
            registry.resolve_name_by_base(FIRST_BASE, "Entry"),
            0,
            "{case}"
        );
        assert_eq!(registry.resolve_name_global("Entry"), 0, "{case}");
        assert_eq!(
            registry.resolve_ordinal_in_module("first", EXPORT_BASE),
            0,
            "{case}"
        );
        assert_eq!(
            registry.resolve_ordinal_by_base(FIRST_BASE, EXPORT_BASE),
            0,
            "{case}"
        );
    }
}

#[test]
fn named_forwarder_chain_stops_at_the_depth_limit() {
    for (module_count, resolves) in [
        (MAX_FORWARDER_DEPTH, true),
        (MAX_FORWARDER_DEPTH + 1, false),
    ] {
        let mut registry = ExportIndexRegistry::new();
        for i in 0..module_count {
            let target = if i + 1 == module_count {
                ExportTarget::Direct { rva: FIRST_RVA }
            } else {
                ExportTarget::Forwarder {
                    value: format!("chain{}.Entry", i + 1),
                }
            };
            registry.register(module_index(
                &format!("chain{i}.dll"),
                FIRST_BASE * (i as u64 + 1),
                EXPORT_BASE,
                &[("Entry", target)],
            ));
        }
        let expected = if resolves {
            FIRST_BASE * module_count as u64 + FIRST_RVA as u64
        } else {
            0
        };

        assert_eq!(
            registry.resolve_name_in_module("chain0", "Entry"),
            expected,
            "{module_count} modules"
        );
    }
}

#[test]
fn missing_or_invalid_forwarder_targets_return_zero() {
    for target in [
        "missing.First",
        "second.Missing",
        "second.#0",
        "second.#invalid",
        "malformed",
    ] {
        let mut registry = ExportIndexRegistry::new();
        registry.register(module_index(
            "first.dll",
            FIRST_BASE,
            EXPORT_BASE,
            &[(
                "Forwarded",
                ExportTarget::Forwarder {
                    value: target.into(),
                },
            )],
        ));
        registry.register(direct_module("second.dll", SECOND_BASE));

        assert_eq!(
            registry.resolve_name_in_module("first", "Forwarded"),
            0,
            "{target}"
        );
        assert_eq!(
            registry.resolve_ordinal_in_module("first", EXPORT_BASE),
            0,
            "{target}"
        );
    }
}

#[test]
fn pe_loader_resolves_ordinals_within_the_requested_module() {
    for (is64, base) in GUEST_MODULES {
        let (mut emu, _) = prepare_winapi(is64);
        emu.export_indexes
            .register(direct_module("first.dll", base));
        emu.export_indexes.register(module_index(
            "second.dll",
            SECOND_BASE,
            EXPORT_BASE + 2,
            &[("Other", ExportTarget::Direct { rva: FIRST_RVA })],
        ));
        for (module, ordinal, expected) in [
            ("first.dll", EXPORT_BASE, base + FIRST_RVA as u64),
            ("first.dll", EXPORT_BASE + 1, base + SECOND_RVA as u64),
            ("first.dll", EXPORT_BASE + 2, 0),
            ("second.dll", EXPORT_BASE, 0),
            ("missing.dll", EXPORT_BASE, 0),
        ] {
            assert_eq!(
                emu.resolve_api_ordinal_in_module(module, ordinal as u16),
                expected,
                "is64={is64}, {module}, ordinal {ordinal}"
            );
        }
    }
}

#[test]
fn synthetic_dll_unwind_exports_match_the_guest_architecture() -> std::io::Result<()> {
    let maps = tempfile::tempdir()?;
    for is64 in [false, true] {
        let (mut emu, name) = prepare_winapi(is64);
        emu.cfg.maps_folder = maps.path().display().to_string();
        emu.cfg.winver = None;
        if is64 {
            peb64::init_peb(&mut emu);
        } else {
            peb32::init_peb(&mut emu);
        }

        for (dll, export, available) in [
            ("kernel32.dll", "GetProcAddress", true),
            ("kernel32.dll", "RtlLookupFunctionEntry", is64),
            ("ntdll.dll", "RtlCaptureContext", true),
            ("ntdll.dll", "RtlLookupFunctionEntry", is64),
            ("ntdll.dll", "RtlVirtualUnwind", is64),
            ("ntdll.dll", "RtlAddFunctionTable", is64),
        ] {
            let base = emu.link_library(dll);
            assert_ne!(base, 0, "x64={is64}, {dll}");
            emu.maps.write_string(name, export);
            let address =
                call_winapi_export(&mut emu, "kernelbase.dll", "GetProcAddress", &[base, name]);
            assert_eq!(address != 0, available, "x64={is64}, {dll}!{export}");
            if !available {
                assert_eq!(emu.get_last_error(), ERROR_PROC_NOT_FOUND);
            }
        }
    }
    Ok(())
}

#[test]
fn get_proc_address_resolves_names_and_ordinals_on_both_architectures() {
    for (is64, base) in GUEST_MODULES {
        let (mut emu, name) = prepare_winapi(is64);
        emu.export_indexes
            .register(direct_module("first.dll", base));
        for (export, ordinal, rva) in [
            ("First", EXPORT_BASE, FIRST_RVA),
            ("Second", EXPORT_BASE + 1, SECOND_RVA),
        ] {
            emu.maps.write_string(name, export);
            for argument in [name, ordinal as u64] {
                assert_eq!(
                    call_winapi_export(
                        &mut emu,
                        "kernel32.dll",
                        "GetProcAddress",
                        &[base, argument]
                    ),
                    base + rva as u64,
                    "is64={is64}, {export}, argument={argument:#x}"
                );
            }
        }
    }
}

#[test]
fn get_proc_address_distinguishes_the_last_ordinal_from_the_first_name_pointer() {
    for (is64, base) in GUEST_MODULES {
        let (mut emu, _) = prepare_winapi(is64);
        emu.export_indexes.register(module_index(
            "first.dll",
            base,
            MAX_ORDINAL as u32,
            &[("LastOrdinal", ExportTarget::Direct { rva: FIRST_RVA })],
        ));
        emu.maps
            .create_map(
                "export_name",
                FIRST_NAME_POINTER,
                NAME_BUFFER_SIZE,
                Permission::READ_WRITE,
            )
            .unwrap();
        emu.maps.write_string(FIRST_NAME_POINTER, "LastOrdinal");
        for argument in [MAX_ORDINAL as u64, FIRST_NAME_POINTER] {
            assert_eq!(
                call_winapi_export(
                    &mut emu,
                    "kernel32.dll",
                    "GetProcAddress",
                    &[base, argument]
                ),
                base + FIRST_RVA as u64,
                "is64={is64}, argument={argument:#x}"
            );
        }
    }
}

#[test]
fn get_proc_address_does_not_borrow_exports_from_another_registered_module() {
    for (is64, base) in GUEST_MODULES {
        let (mut emu, name) = prepare_winapi(is64);
        emu.export_indexes
            .register(direct_module("first.dll", base));
        emu.export_indexes.register(module_index(
            "second.dll",
            SECOND_BASE,
            EXPORT_BASE + 2,
            &[("Other", ExportTarget::Direct { rva: FIRST_RVA })],
        ));
        emu.maps.write_string(name, "First");
        for argument in [name, EXPORT_BASE as u64] {
            assert_eq!(
                call_winapi_export(
                    &mut emu,
                    "kernel32.dll",
                    "GetProcAddress",
                    &[base, argument]
                ),
                base + FIRST_RVA as u64,
                "is64={is64}"
            );
            assert_eq!(
                call_winapi_export(
                    &mut emu,
                    "kernel32.dll",
                    "GetProcAddress",
                    &[SECOND_BASE, argument]
                ),
                0,
                "is64={is64}"
            );
        }
    }
}

#[test]
fn get_proc_address_follows_named_and_ordinal_forwarders_on_both_architectures() {
    for (is64, base) in GUEST_MODULES {
        let (mut emu, name) = prepare_winapi(is64);
        emu.export_indexes
            .register(direct_module("second.dll", base));
        let host = REPLACEMENT_BASE;
        emu.export_indexes
            .register(direct_module("kernelbase.dll", host));
        let contract = "api-ms-win-core-libraryloader-l1-1-0";
        for (target, expected) in [
            (format!("{contract}.First"), host + FIRST_RVA as u64),
            (
                format!("{contract}.#{EXPORT_BASE}"),
                host + FIRST_RVA as u64,
            ),
            (format!("{contract}.first"), 0),
            ("second.First".to_string(), base + FIRST_RVA as u64),
            (format!("second.#{EXPORT_BASE}"), base + FIRST_RVA as u64),
            ("second.first".to_string(), 0),
        ] {
            emu.export_indexes.register(module_index(
                "first.dll",
                FIRST_BASE,
                EXPORT_BASE,
                &[(
                    "Forwarded",
                    ExportTarget::Forwarder {
                        value: target.clone(),
                    },
                )],
            ));
            emu.maps.write_string(name, "Forwarded");
            for argument in [name, EXPORT_BASE as u64] {
                assert_eq!(
                    call_winapi_export(
                        &mut emu,
                        "kernel32.dll",
                        "GetProcAddress",
                        &[FIRST_BASE, argument]
                    ),
                    expected,
                    "is64={is64}, {target}"
                );
            }
        }
    }
}

#[test]
fn get_proc_address_preserves_name_case_and_rejects_unknown_module_handles() {
    const ERROR_MARKER: u64 = 0x1234;
    for (is64, base) in GUEST_MODULES {
        let (mut emu, name) = prepare_winapi(is64);
        emu.export_indexes.register(module_index(
            "first.dll",
            base,
            EXPORT_BASE,
            &[
                ("Export", ExportTarget::Direct { rva: FIRST_RVA }),
                ("export", ExportTarget::Direct { rva: SECOND_RVA }),
            ],
        ));
        for (handle, symbol, address, error) in [
            (base, "Export", base + FIRST_RVA as u64, ERROR_MARKER),
            (base, "export", base + SECOND_RVA as u64, ERROR_MARKER),
            (base, "EXPORT", 0, ERROR_PROC_NOT_FOUND),
            (base, " Export", 0, ERROR_PROC_NOT_FOUND),
            (0, "Export", 0, ERROR_MOD_NOT_FOUND),
            (REPLACEMENT_BASE, "Export", 0, ERROR_MOD_NOT_FOUND),
        ] {
            emu.maps.write_string(name, symbol);
            emu.set_last_error(ERROR_MARKER);
            assert_eq!(
                call_winapi_export(&mut emu, "kernel32.dll", "GetProcAddress", &[handle, name]),
                address,
                "is64={is64}, {handle:#x}!{symbol}"
            );
            assert_eq!(emu.get_last_error(), error);
        }
    }
}
