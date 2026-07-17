use crate::tests::helpers;
use crate::windows::structures;
use crate::*;
use std::collections::HashSet;
use std::path::Path;

const CORE_MODULES: [&str; 4] = ["loader.exe", "ntdll.dll", "kernel32.dll", "kernelbase.dll"];

fn assert_module_image(emu: &Emu, name: &str, base: u64) {
    assert!(base != 0, "{} must have a nonzero DllBase", name);
    assert_eq!(
        emu.maps.read_word(base),
        Some(0x5a4d),
        "{} DllBase 0x{:x} must point to an MZ image",
        name,
        base
    );

    let map_name = format!(
        "{}.pe",
        name.strip_suffix(".dll")
            .or_else(|| name.strip_suffix(".exe"))
            .unwrap_or(name)
    );
    let map = emu
        .maps
        .get_map_by_name(&map_name)
        .unwrap_or_else(|| panic!("{} must have map {}", name, map_name));
    assert_eq!(
        map.get_base(),
        base,
        "{} LDR DllBase must match {} base",
        name,
        map_name
    );
}

fn assert_x64_loader_order(emu: &Emu) {
    let peb_map = emu.maps.get_mem("peb");
    let peb_addr = peb_map.get_base();
    let ldr_map = emu.maps.get_mem("ldr");
    let ldr_addr = ldr_map.get_base();
    let peb = structures::PEB64::load(peb_addr, &emu.maps);
    let ldr = structures::PebLdrData64::load(ldr_addr, &emu.maps);

    assert_eq!(peb.ldr, ldr_addr, "PEB.Ldr must point to the LDR map");
    assert_ne!(ldr.initializated, 0, "PEB_LDR_DATA must be initialized");

    let sentinel = ldr_addr + 0x10;
    let mut current = ldr.in_load_order_module_list.flink;
    let mut visited = HashSet::new();
    let mut names = Vec::new();

    while current != sentinel {
        assert!(current != 0, "x64 LDR list reached a null entry");
        assert!(visited.insert(current), "x64 LDR list contains a cycle");
        assert!(
            visited.len() <= 4096,
            "x64 LDR list exceeded the traversal guard"
        );

        let entry = structures::LdrDataTableEntry64::load(current, &emu.maps);
        let name = emu.maps.read_wide_string(entry.base_dll_name.buffer);
        assert_eq!(
            entry.in_memory_order_links.flink,
            entry.in_load_order_links.flink + 0x10,
            "x64 {} memory-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.flink,
            entry.in_load_order_links.flink + 0x20,
            "x64 {} initialization-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_memory_order_links.blink,
            entry.in_load_order_links.blink + 0x10,
            "x64 {} memory-order Blink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.blink,
            entry.in_load_order_links.blink + 0x20,
            "x64 {} initialization-order Blink is inconsistent",
            name
        );

        if names.len() < CORE_MODULES.len() {
            assert_eq!(
                name,
                CORE_MODULES[names.len()],
                "x64 LDR prefix so far: {:?}",
                names
            );
            assert_module_image(emu, &name, entry.dll_base);
        }
        names.push(name);
        current = entry.in_load_order_links.flink;
    }

    assert!(
        names.len() >= CORE_MODULES.len(),
        "x64 LDR list has only {:?}; expected the core prefix {:?}",
        names,
        CORE_MODULES
    );
}

fn assert_x86_loader_order(emu: &Emu) {
    let peb_map = emu.maps.get_mem("peb");
    let peb_addr = peb_map.get_base();
    let ldr_map = emu.maps.get_mem("ldr");
    let ldr_addr = ldr_map.get_base();
    let peb = structures::PEB::load(peb_addr, &emu.maps);
    let ldr = structures::PebLdrData::load(ldr_addr, &emu.maps);

    assert_eq!(
        peb.ldr as u64, ldr_addr,
        "PEB.Ldr must point to the LDR map"
    );
    assert_ne!(ldr.initializated, 0, "PEB_LDR_DATA must be initialized");

    let first = ldr.in_load_order_module_list.flink as u64;
    assert_ne!(first, 0, "x86 LDR list must have a first entry");
    let mut current = ldr.in_load_order_module_list.flink as u64;
    let mut visited = HashSet::new();
    let mut names = Vec::new();
    let mut last_entry = 0;

    loop {
        assert!(current != 0, "x86 LDR list reached a null entry");
        if current == first && !names.is_empty() {
            break;
        }
        assert!(
            visited.insert(current),
            "x86 LDR list contains an invalid cycle"
        );
        assert!(
            visited.len() <= 4096,
            "x86 LDR list exceeded the traversal guard"
        );

        let entry = structures::LdrDataTableEntry::load(current, &emu.maps);
        last_entry = current;
        let name = emu.maps.read_wide_string(entry.base_dll_name.buffer as u64);
        assert_eq!(
            entry.in_memory_order_links.flink,
            entry.in_load_order_links.flink + 0x08,
            "x86 {} memory-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.flink,
            entry.in_load_order_links.flink + 0x10,
            "x86 {} initialization-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_memory_order_links.blink,
            entry.in_load_order_links.blink + 0x08,
            "x86 {} memory-order Blink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.blink,
            entry.in_load_order_links.blink + 0x10,
            "x86 {} initialization-order Blink is inconsistent",
            name
        );

        if names.len() < CORE_MODULES.len() {
            assert_eq!(
                name,
                CORE_MODULES[names.len()],
                "x86 LDR prefix so far: {:?}",
                names
            );
            assert_module_image(emu, &name, entry.dll_base as u64);
        }
        names.push(name);
        current = entry.in_load_order_links.flink as u64;
    }

    assert_eq!(
        ldr.in_load_order_module_list.blink as u64, last_entry,
        "x86 load-order list head Blink must point to the tail"
    );

    assert!(
        names.len() >= CORE_MODULES.len(),
        "x86 LDR list has only {:?}; expected the core prefix {:?}",
        names,
        CORE_MODULES
    );
}

#[test]
fn loader_exe_peb_module_order_x86() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    let loader = Path::new(&emu.cfg.maps_folder).join("loader.exe");
    assert!(
        loader.is_file(),
        "missing support executable: {}",
        loader.display()
    );

    emu.load_code(loader.to_str().expect("loader path must be UTF-8"));
    assert_x86_loader_order(&emu);
}

#[test]
fn loader_exe_peb_module_order_x64() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    let loader = Path::new(&emu.cfg.maps_folder).join("loader.exe");
    assert!(
        loader.is_file(),
        "missing support executable: {}",
        loader.display()
    );

    emu.load_code(loader.to_str().expect("loader path must be UTF-8"));
    assert_x64_loader_order(&emu);
}
