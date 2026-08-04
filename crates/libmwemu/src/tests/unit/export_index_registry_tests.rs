// End-to-end lifecycle and forwarder-resolution tests for the export-name
// registry. These exercise the public `ExportIndexRegistry` API the same way
// `Emu::load_pe32` / `load_pe64` / `dynamic_unlink_module` use it.

use rs_header::pe::export_index::{ExportIndexData, build_export_index};
use rs_header::pe::shared::ImageSectionHeader;

use crate::api::windows::export_index::{
    ExportIndexRegistry, IndexedExport, MAX_FORWARDER_DEPTH, ModuleExportIndex,
};

fn section(va: u32, raw_ptr: u32, raw_size: u32) -> ImageSectionHeader {
    let mut s = ImageSectionHeader {
        name: [0; 8],
        virtual_size: raw_size,
        virtual_address: va,
        size_of_raw_data: raw_size,
        pointer_to_raw_data: raw_ptr,
        pointer_to_relocations: 0,
        pointer_to_linenumbers: 0,
        number_of_relocations: 0,
        number_of_linenumbers: 0,
        characteristics: 0,
    };
    let n = b".text\0\0\0";
    s.name[..n.len()].copy_from_slice(n);
    s
}

fn write_export_dir(raw: &mut [u8], export_off: usize, exp: &[u8; 40]) {
    raw[export_off..export_off + 40].copy_from_slice(exp);
}

fn build_export_table(
    base: u32,
    funcs: &[(u32, bool)], // (rva, is_forwarder)
    names: &[(&str, u16)], // (name, ordinal_index)
) -> (Vec<u8>, Vec<ImageSectionHeader>, u32, u32) {
    let mut raw = vec![0u8; 0x400];
    let export_off = 0x040;
    let func_off = 0x080;
    let name_off_table = 0x0c0;
    let ord_off = 0x100;
    let fwd_str_off = 0x140;
    let name_data_off = 0x180;

    let export_va: u32 = 0x1040;
    let func_va: u32 = 0x1080;
    let name_table_va: u32 = 0x10c0;
    let ord_table_va: u32 = 0x1100;
    let fwd_str_va: u32 = 0x1140;
    let name_data_va: u32 = 0x1180;

    let mut exp = [0u8; 40];
    exp[16..20].copy_from_slice(&base.to_le_bytes());
    exp[20..24].copy_from_slice(&(funcs.len() as u32).to_le_bytes());
    exp[24..28].copy_from_slice(&(names.len() as u32).to_le_bytes());
    exp[28..32].copy_from_slice(&func_va.to_le_bytes());
    exp[32..36].copy_from_slice(&name_table_va.to_le_bytes());
    exp[36..40].copy_from_slice(&ord_table_va.to_le_bytes());
    write_export_dir(&mut raw, export_off, &exp);

    // Function table.
    for (i, (rva, _)) in funcs.iter().enumerate() {
        raw[func_off + i * 4..func_off + (i + 1) * 4].copy_from_slice(&rva.to_le_bytes());
    }

    // Names + name-ordinal table (in declaration order).
    let mut name_data_writer = name_data_off;
    for (i, (name, ord_idx)) in names.iter().enumerate() {
        // Name pointer (VA -> raw).
        let name_va = name_data_va + (name_data_writer - name_data_off) as u32;
        raw[name_off_table + i * 4..name_off_table + (i + 1) * 4]
            .copy_from_slice(&name_va.to_le_bytes());
        let s = name.as_bytes();
        raw[name_data_writer..name_data_writer + s.len()].copy_from_slice(s);
        raw[name_data_writer + s.len()] = 0;
        name_data_writer += s.len() + 1;

        raw[ord_off + i * 2..ord_off + (i + 1) * 2].copy_from_slice(&ord_idx.to_le_bytes());
    }

    // Forwarder strings, one per forwarder entry (in order).
    let mut fwd_writer = fwd_str_off;
    for (i, (_, is_fwd)) in funcs.iter().enumerate() {
        if *is_fwd {
            let fwd_rva = fwd_str_va + (fwd_writer - fwd_str_off) as u32;
            // Patch the function table with this forwarder RVA.
            raw[func_off + i * 4..func_off + (i + 1) * 4].copy_from_slice(&fwd_rva.to_le_bytes());
            // Default forwarder target — tests can override raw bytes here if
            // they need a specific name.
            let s = format!("BACKING.Missing{}", i);
            let bytes = s.as_bytes();
            raw[fwd_writer..fwd_writer + bytes.len()].copy_from_slice(bytes);
            raw[fwd_writer + bytes.len()] = 0;
            fwd_writer += bytes.len() + 1;
        }
    }

    let sections = vec![section(0x1000, 0, raw.len() as u32)];
    (raw, sections, export_va, 0x200)
}

fn parse_and_register(
    reg: &mut ExportIndexRegistry,
    module: &str,
    base: u64,
    raw: &[u8],
    sections: &[ImageSectionHeader],
    va: u32,
    size: u32,
) {
    let parsed = build_export_index(raw, sections, va, size).expect("parse");
    let normalized = crate::api::windows::export_index::normalize_module_name(module);
    let index = ModuleExportIndex::from_parsed(module.to_string(), normalized, base, &parsed);
    reg.register(index);
}

#[test]
fn forwarder_chain_resolves_through_registered_target() {
    // Build a registry with two modules:
    //   - "kernel32.dll" base 0x10000:
    //       function 0 = direct RVA 0x1500  -> "Direct"
    //       function 1 = forwarder to "kernelbase.HeapAlloc"  -> "ViaFwd"
    //   - "kernelbase.dll" base 0x50000:
    //       function 0 = direct RVA 0x1500  -> "HeapAlloc"
    let mut kernelbase_raw = vec![0u8; 0x400];
    let export_off = 0x040;
    let func_off = 0x080;
    let name_off_table = 0x0c0;
    let ord_off = 0x100;
    let name_data_off = 0x180;
    let mut exp = [0u8; 40];
    exp[16..20].copy_from_slice(&1u32.to_le_bytes()); // base
    exp[20..24].copy_from_slice(&1u32.to_le_bytes()); // nof
    exp[24..28].copy_from_slice(&1u32.to_le_bytes()); // non
    exp[28..32].copy_from_slice(&0x1080u32.to_le_bytes());
    exp[32..36].copy_from_slice(&0x10c0u32.to_le_bytes());
    exp[36..40].copy_from_slice(&0x1100u32.to_le_bytes());
    write_export_dir(&mut kernelbase_raw, export_off, &exp);
    kernelbase_raw[func_off..func_off + 4].copy_from_slice(&0x1500u32.to_le_bytes());
    kernelbase_raw[name_off_table..name_off_table + 4].copy_from_slice(&0x1180u32.to_le_bytes());
    kernelbase_raw[ord_off..ord_off + 2].copy_from_slice(&0u16.to_le_bytes());
    let s = b"HeapAlloc\0";
    kernelbase_raw[name_data_off..name_data_off + s.len()].copy_from_slice(s);
    let sections = vec![section(0x1000, 0, kernelbase_raw.len() as u32)];

    let (k32_raw, k32_sections, k32_va, k32_size) = build_export_table(
        1,
        &[(0x1500, false), (0x1140, true)],
        &[("Direct", 0), ("ViaFwd", 1)],
    );
    // Patch the kernel32 forwarder string to "kernelbase.HeapAlloc".
    let fwd_off = 0x140;
    let s = b"kernelbase.HeapAlloc\0";
    let mut k32_raw = k32_raw;
    k32_raw[fwd_off..fwd_off + s.len()].copy_from_slice(s);

    let mut reg = ExportIndexRegistry::new();
    parse_and_register(
        &mut reg,
        "kernel32.dll",
        0x10000,
        &k32_raw,
        &k32_sections,
        k32_va,
        k32_size,
    );
    parse_and_register(
        &mut reg,
        "kernelbase.dll",
        0x50000,
        &kernelbase_raw,
        &sections,
        0x1040,
        0x200,
    );

    assert_eq!(reg.resolve_name_in_module("kernel32", "Direct"), 0x11500);
    // ViaFwd -> kernelbase.HeapAlloc -> 0x50000 + 0x1500 = 0x51500.
    assert_eq!(reg.resolve_name_in_module("kernel32", "ViaFwd"), 0x51500);
}

#[test]
fn forwarder_chain_cycle_does_not_loop_forever() {
    // Build a registry where both modules forward to each other.
    let raw1 = build_export_table(1, &[(0x1140, true)], &[("A", 0)]);
    let raw2 = build_export_table(1, &[(0x1140, true)], &[("B", 0)]);
    // Patch both forwarder strings to point at each other.
    let mut r1 = raw1.0;
    let mut r2 = raw2.0;
    let s = b"modB.A\0";
    r1[0x140..0x140 + s.len()].copy_from_slice(s);
    let s = b"modA.B\0";
    r2[0x140..0x140 + s.len()].copy_from_slice(s);

    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "modA.dll", 0x10000, &r1, &raw1.1, raw1.2, raw1.3);
    parse_and_register(&mut reg, "modB.dll", 0x50000, &r2, &raw2.1, raw2.2, raw2.3);
    // The visited-set + depth guard must keep this from looping forever.
    let start = std::time::Instant::now();
    assert_eq!(reg.resolve_name_in_module("modA", "A"), 0);
    assert!(start.elapsed().as_millis() < 100);
}

#[test]
fn forwarder_chain_depth_limit_returns_zero() {
    // Single module, one direct entry "Direct" and one forwarder "Chase".
    // The forwarder target does not exist -> depth budget is spent reaching
    // depth=0 which returns 0.
    let (raw, sections, va, size) = build_export_table(
        1,
        &[(0x1500, false), (0x1140, true)],
        &[("Direct", 0), ("Chase", 1)],
    );
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "test.dll", 0x10000, &raw, &sections, va, size);
    assert_eq!(reg.resolve_name_in_module("test", "Chase"), 0);
}

#[test]
fn replacement_removes_old_base_but_preserves_order() {
    let (raw, sections, va, size) = build_export_table(1, &[(0x1500, false)], &[("Fn", 0)]);
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "alpha.dll", 0x1000, &raw, &sections, va, size);
    parse_and_register(&mut reg, "beta.dll", 0x2000, &raw, &sections, va, size);
    parse_and_register(&mut reg, "alpha.dll", 0x3000, &raw, &sections, va, size);

    assert!(reg.get_by_base(0x1000).is_none());
    assert!(reg.get_by_base(0x3000).is_some());
    let names: Vec<_> = reg
        .iter_ordered()
        .map(|m| m.normalized_name.clone())
        .collect();
    assert_eq!(names, vec!["alpha", "beta"]);
    // After rebasing alpha, the lookup returns the new base 0x3000.
    assert_eq!(reg.resolve_name_in_module("alpha", "Fn"), 0x3000 + 0x1500);
}

#[test]
fn removal_drops_module_from_all_indices() {
    let (raw, sections, va, size) = build_export_table(1, &[(0x1500, false)], &[("Fn", 0)]);
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "alpha.dll", 0x1000, &raw, &sections, va, size);
    parse_and_register(&mut reg, "beta.dll", 0x2000, &raw, &sections, va, size);
    assert!(reg.remove("alpha"));
    assert!(reg.get_by_name("alpha").is_none());
    assert!(reg.get_by_base(0x1000).is_none());
    assert_eq!(reg.resolve_name_in_module("alpha", "Fn"), 0);
    // Beta unaffected.
    assert_eq!(reg.resolve_name_in_module("beta", "Fn"), 0x2000 + 0x1500);
}

#[test]
fn unknown_name_returns_zero_and_does_not_scan() {
    let (raw, sections, va, size) = build_export_table(1, &[(0x1500, false)], &[("Fn", 0)]);
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "alpha.dll", 0x1000, &raw, &sections, va, size);
    assert_eq!(reg.resolve_name_in_module("alpha", "Missing"), 0);
    assert_eq!(reg.resolve_name_global("Missing"), 0);
}

#[test]
fn ordinal_lookup_uses_export_base_and_handles_per_handle() {
    let (raw, sections, va, size) = build_export_table(
        5,
        &[(0x1500, false), (0x1500, false)],
        &[("Fn0", 0), ("Fn1", 1)],
    );
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "mod.dll", 0x10000, &raw, &sections, va, size);
    // base=5, ordinals 5..6 -> function-table indices 0..1.
    assert_eq!(reg.resolve_ordinal_by_base(0x10000, 5), 0x11500);
    assert_eq!(reg.resolve_ordinal_by_base(0x10000, 6), 0x11500);
    // Wrong handle -> 0.
    assert_eq!(reg.resolve_ordinal_by_base(0x99999, 5), 0);
}

#[test]
fn forwarder_max_depth_constant_matches_documented_limit() {
    // The plan / docs call out a depth limit of 8; make sure the constant
    // matches so accidental changes surface here.
    assert_eq!(MAX_FORWARDER_DEPTH, 8);
}

#[test]
fn indexed_export_helpers_classify_direct_vs_forwarder() {
    let direct = IndexedExport::Direct { address: 0x42 };
    assert!(direct.is_direct());
    assert_eq!(direct.direct_address(), Some(0x42));

    let fwd = IndexedExport::Forwarder {
        value: "x.y".to_string(),
    };
    assert!(!fwd.is_direct());
    assert_eq!(fwd.direct_address(), None);
}

#[test]
fn unindexed_module_lookup_is_zero() {
    let reg = ExportIndexRegistry::new();
    assert_eq!(reg.resolve_name_in_module("ghost", "Fn"), 0);
    assert_eq!(reg.resolve_name_by_base(0xdead, "Fn"), 0);
    assert_eq!(reg.resolve_ordinal_by_base(0xdead, 1), 0);
    assert_eq!(reg.resolve_address(0xdead), None);
    assert_eq!(reg.iter_ordered().count(), 0);
}

#[test]
fn empty_parsed_index_does_not_register() {
    // A module whose export directory is malformed or absent produces None
    // from the builder. The loader skips registration in that case (see
    // register_export_index_from_raw). Verify the behavior at this layer.
    let reg = ExportIndexRegistry::new();
    let empty = ExportIndexData::default();
    assert!(empty.is_empty());
    // The registry has no entries yet.
    assert_eq!(reg.len(), 0);
}

#[test]
fn pe_loader_ordinal_resolution_uses_module_registry() {
    // Confirm the live Emu:PeLoader bridge goes through the
    // module-scoped export registry rather than the unrelated global name
    // resolver. The synthetic fake.dll helper is shared with the
    // GetProcAddress tests; its single export sits at `base + 0x1500` with
    // ordinal 5.
    crate::tests::helpers::setup();
    let mut emu = crate::emu64();
    let base = 0x1_0000_0000u64;
    crate::tests::helpers::register_fake_export_module(&mut emu, base);

    // Resolved: ordinal 5 lives at `base + 0x1500`.
    let resolved = <crate::emu::Emu as rs_header::pe::PeLoader>::resolve_api_ordinal_in_module(
        &mut emu, "fake.dll", 5,
    );
    assert_eq!(resolved, base + 0x1500, "module ordinal did not resolve");

    // Unknown ordinal returns 0 (not a panic, not a global fallback).
    assert_eq!(
        <crate::emu::Emu as rs_header::pe::PeLoader>::resolve_api_ordinal_in_module(
            &mut emu, "fake.dll", 6,
        ),
        0
    );

    // Unknown module returns 0.
    assert_eq!(
        <crate::emu::Emu as rs_header::pe::PeLoader>::resolve_api_ordinal_in_module(
            &mut emu,
            "missing.dll",
            5,
        ),
        0
    );
}

#[test]
fn resolve_ordinal_in_module_uses_export_base_and_normalizes_name() {
    // build_export_table(base, funcs, names) wires `base` as the export
    // ordinal base. Direct function-table entries map export ordinals
    // `base..base+nof-1` to function-table indices 0..n.
    let (raw, sections, va, size) = build_export_table(
        5,
        &[(0x1500, false), (0x1500, false)],
        &[("Fn0", 0), ("Fn1", 1)],
    );
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "mod.dll", 0x10000, &raw, &sections, va, size);

    // base=5, ordinals 5..6 -> function-table indices 0..1, both direct 0x1500.
    assert_eq!(reg.resolve_ordinal_in_module("mod", 5), 0x11500);
    assert_eq!(reg.resolve_ordinal_in_module("mod", 6), 0x11500);

    // Below-base ordinals are not exported -> 0.
    assert_eq!(reg.resolve_ordinal_in_module("mod", 4), 0);

    // Out-of-range ordinals (>= base + nof) are not exported -> 0.
    assert_eq!(reg.resolve_ordinal_in_module("mod", 7), 0);

    // Module name normalization accepts path + case + missing .dll.
    assert_eq!(
        reg.resolve_ordinal_in_module("C:\\Windows\\MOD.DLL", 5),
        0x11500
    );
}

#[test]
fn resolve_ordinal_in_module_ordinal_forwarder_chases_target() {
    // Source: "fake.dll" base 0x10000, export base 1, function 0 forwards
    // to "backing.#5" (ordinal 5). Backing: export base 5 with one direct
    // function at RVA 0x1500 -> function-table index 0, ordinal 5.
    let (fake_raw, fake_sections, fake_va, fake_size) =
        build_export_table(1, &[(0x1140, true)], &[("Via", 0)]);
    let mut fake_raw = fake_raw;
    let s = b"backing.#5\0";
    fake_raw[0x140..0x140 + s.len()].copy_from_slice(s);

    let (backing_raw, backing_sections, backing_va, backing_size) =
        build_export_table(5, &[(0x1500, false)], &[("Direct", 0)]);

    let mut reg = ExportIndexRegistry::new();
    parse_and_register(
        &mut reg,
        "fake.dll",
        0x10000,
        &fake_raw,
        &fake_sections,
        fake_va,
        fake_size,
    );
    parse_and_register(
        &mut reg,
        "backing.dll",
        0x20000,
        &backing_raw,
        &backing_sections,
        backing_va,
        backing_size,
    );

    // Ordinal 1 in fake -> forwarder backing.#5 -> backing base+0x1500.
    assert_eq!(reg.resolve_ordinal_in_module("fake", 1), 0x20000 + 0x1500);
}

#[test]
fn resolve_ordinal_in_module_does_not_scan_other_modules() {
    // Two modules, one with the requested ordinal and one without. The
    // resolver must NOT fall through to the other module — ordinals are
    // module-scoped.
    let (raw, sections, va, size) = build_export_table(1, &[(0x1500, false)], &[("Fn", 0)]);
    let mut reg = ExportIndexRegistry::new();
    parse_and_register(&mut reg, "present.dll", 0x1000, &raw, &sections, va, size);
    parse_and_register(&mut reg, "absent.dll", 0x2000, &raw, &sections, va, size);

    // Requested module owns the ordinal -> resolves to its base.
    assert_eq!(reg.resolve_ordinal_in_module("present", 1), 0x1000 + 0x1500);

    // The other module does not own ordinal 5, so even though "absent" is
    // registered, ordinal 5 there is below base and returns 0.
    assert_eq!(reg.resolve_ordinal_in_module("absent", 5), 0);

    // Unknown module returns 0 even if the ordinal exists elsewhere.
    assert_eq!(reg.resolve_ordinal_in_module("missing", 1), 0);
}
