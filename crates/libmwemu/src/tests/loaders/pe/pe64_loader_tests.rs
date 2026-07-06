use crate::tests::helpers;
use crate::*;

#[test]
fn pe64_loader_sets_entrypoint_and_maps_main_image() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&helpers::test_data_path("exe64win_msgbox.bin"));

    assert!(emu.pe64.is_some(), "PE64 metadata should be loaded");
    assert!(
        emu.maps.is_mapped(emu.regs().rip),
        "RIP should point to mapped memory"
    );

    let map_name = emu.maps.get_addr_name(emu.regs().rip).unwrap_or("unknown");
    assert!(
        map_name.contains("exe64win_msgbox"),
        "entrypoint should belong to sample image map, got {}",
        map_name
    );

    assert!(
        emu.maps.get_map_by_name("exe64win_msgbox.pe").is_some(),
        "main PE header map should exist"
    );
}

#[test]
fn pe64_loader_adds_core_ldr_modules() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&helpers::test_data_path("exe64win_msgbox.bin"));

    assert!(emu.maps.get_map_by_name("ntdll.pe").is_some());
    assert!(emu.maps.get_map_by_name("kernel32.pe").is_some());
    assert!(emu.maps.get_map_by_name("kernelbase.pe").is_some());
}

#[test]
fn pe64_loader_normalizes_api_set_dependencies() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&helpers::test_data_path("exe64win_mingw.bin"));

    let pe = emu.pe64.take().expect("PE64 metadata should be present");
    let deps = pe.get_dependencies();
    emu.pe64 = Some(pe);
    assert!(!deps.is_empty(), "dependencies should not be empty");
    assert!(
        deps.iter().all(|d| !d.starts_with("api-ms-win-")),
        "api-set dependencies should be normalized, got {:?}",
        deps
    );
    assert!(
        deps.iter().any(|d| d == "kernelbase"),
        "normalized dependencies should include kernelbase, got {:?}",
        deps
    );
}
