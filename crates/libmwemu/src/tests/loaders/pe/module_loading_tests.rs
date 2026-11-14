use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;
use std::collections::HashSet;
use std::path::Path;

use itertools::iproduct;

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{Allocation, MemoryKind, PageState};
use crate::tests::helpers::{
    self, WINAPI_MODULE_BASE, call_winapi_export as call, prepare_winapi as prepare,
};
use crate::winapi::{helper, winapi64};
use crate::windows::structures;
use crate::{emu32, emu64};

const MAX_LDR_ENTRIES: usize = 4096;
const LDR64_LOAD_ORDER_LIST_OFFSET: u64 = 0x10;
const LDR64_MEMORY_ORDER_LINKS_OFFSET: u64 = 0x10;
const LDR64_INITIALIZATION_ORDER_LINKS_OFFSET: u64 = 0x20;
const LDR32_MEMORY_ORDER_LINKS_OFFSET: u32 = 0x08;
const LDR32_INITIALIZATION_ORDER_LINKS_OFFSET: u32 = 0x10;

const CORE_MODULES: [&str; 4] = ["loader.exe", "ntdll.dll", "kernel32.dll", "kernelbase.dll"];

fn assert_module_image(emu: &Emu, name: &str, base: u64) {
    assert!(base != 0, "{} must have a nonzero DllBase", name);
    assert_eq!(
        emu.maps.read_word(base),
        Some(u16::from_le_bytes(*b"MZ")),
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

    let sentinel = ldr_addr + LDR64_LOAD_ORDER_LIST_OFFSET;
    let mut current = ldr.in_load_order_module_list.flink;
    let mut visited = HashSet::new();
    let mut names = Vec::new();

    while current != sentinel {
        assert!(current != 0, "x64 LDR list reached a null entry");
        assert!(visited.insert(current), "x64 LDR list contains a cycle");
        assert!(
            visited.len() <= MAX_LDR_ENTRIES,
            "x64 LDR list exceeded the traversal guard"
        );

        let entry = structures::LdrDataTableEntry64::load(current, &emu.maps);
        let name = emu.maps.read_wide_string(entry.base_dll_name.buffer);
        assert_eq!(
            entry.in_memory_order_links.flink,
            entry.in_load_order_links.flink + LDR64_MEMORY_ORDER_LINKS_OFFSET,
            "x64 {} memory-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.flink,
            entry.in_load_order_links.flink + LDR64_INITIALIZATION_ORDER_LINKS_OFFSET,
            "x64 {} initialization-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_memory_order_links.blink,
            entry.in_load_order_links.blink + LDR64_MEMORY_ORDER_LINKS_OFFSET,
            "x64 {} memory-order Blink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.blink,
            entry.in_load_order_links.blink + LDR64_INITIALIZATION_ORDER_LINKS_OFFSET,
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
            visited.len() <= MAX_LDR_ENTRIES,
            "x86 LDR list exceeded the traversal guard"
        );

        let entry = structures::LdrDataTableEntry::load(current, &emu.maps);
        last_entry = current;
        let name = emu.maps.read_wide_string(entry.base_dll_name.buffer as u64);
        assert_eq!(
            entry.in_memory_order_links.flink,
            entry.in_load_order_links.flink + LDR32_MEMORY_ORDER_LINKS_OFFSET,
            "x86 {} memory-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.flink,
            entry.in_load_order_links.flink + LDR32_INITIALIZATION_ORDER_LINKS_OFFSET,
            "x86 {} initialization-order Flink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_memory_order_links.blink,
            entry.in_load_order_links.blink + LDR32_MEMORY_ORDER_LINKS_OFFSET,
            "x86 {} memory-order Blink is inconsistent",
            name
        );
        assert_eq!(
            entry.in_initialization_order_links.blink,
            entry.in_load_order_links.blink + LDR32_INITIALIZATION_ORDER_LINKS_OFFSET,
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
    emu.cfg.maps_folder = win_maps!(32);
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
    emu.cfg.maps_folder = win_maps!(64);
    let loader = Path::new(&emu.cfg.maps_folder).join("loader.exe");
    assert!(
        loader.is_file(),
        "missing support executable: {}",
        loader.display()
    );

    emu.load_code(loader.to_str().expect("loader path must be UTF-8"));
    assert_x64_loader_order(&emu);
}

const MARKER: u32 = 0xcccc_cccc;
const UNKNOWN_MODULE_FLAG: u64 = 1 << 3;

#[test]
fn module_handles_are_image_bases_and_extended_outputs_have_pointer_width() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        let main = if is64 { 0x1_4000_0000 } else { 0x0040_0000 };
        emu.base = main;
        emu.regs_mut().rip = WINAPI_MODULE_BASE; // A call from a DLL still returns the main image.
        emu.maps
            .create_map("module.with.dots", main, 0x2000, Permission::READ_EXECUTE)
            .unwrap();
        emu.maps.register_allocation(Allocation::new(
            main,
            0x2000,
            PAGE_EXECUTE_READ,
            MemoryKind::Image,
            PageState::committed(PAGE_EXECUTE_READ, Permission::READ_EXECUTE),
        ));
        let (plain, extended) = if wide {
            ("GetModuleHandleW", "GetModuleHandleExW")
        } else {
            ("GetModuleHandleA", "GetModuleHandleExA")
        };
        let output = data + 0x200;
        let pointer_size = if is64 { 8 } else { 4 };
        assert_eq!(call(&mut emu, library, plain, &[0]), main);
        for (flags, name) in [
            (0, 0),
            (GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS as u64, main + 0x1234),
        ] {
            emu.maps.write_dword(output + pointer_size, MARKER);
            assert_eq!(call(&mut emu, library, extended, &[flags, name, output]), 1);
            let base = if is64 {
                emu.maps.read_qword(output).unwrap()
            } else {
                emu.maps.read_dword(output).unwrap() as u64
            };
            assert_eq!(base, main);
            assert_eq!(emu.maps.read_dword(output + pointer_size), Some(MARKER));
        }
        call(&mut emu, "kernel32.dll", plain, &[0]);
        for module in [
            "KERNEL32.DLL",
            "kernel32",
            "C:\\Windows\\System32\\KERNEL32.dll",
            "C:/Windows/System32/kernel32.dll",
        ] {
            if wide {
                emu.maps.write_wide_string(data, module);
            } else {
                emu.maps.write_string(data, module);
            }
            assert_eq!(call(&mut emu, library, plain, &[data]), WINAPI_MODULE_BASE);
        }
        if wide {
            emu.maps.write_wide_string(data, "missing.dll");
        } else {
            emu.maps.write_string(data, "missing.dll");
        }
        assert_eq!(call(&mut emu, library, plain, &[data]), 0);
        assert_eq!(emu.get_last_error(), ERROR_MOD_NOT_FOUND);
        for (flags, name, error) in [
            (0, data, ERROR_MOD_NOT_FOUND),
            (
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS as u64,
                data,
                ERROR_MOD_NOT_FOUND,
            ),
            (
                (GET_MODULE_HANDLE_EX_FLAG_PIN | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT)
                    as u64,
                0,
                ERROR_INVALID_PARAMETER,
            ),
            (UNKNOWN_MODULE_FLAG, 0, ERROR_INVALID_PARAMETER),
        ] {
            emu.maps.write_qword(output, u64::MAX);
            assert_eq!(call(&mut emu, library, extended, &[flags, name, output]), 0);
            assert_eq!(emu.maps.read_dword(output), Some(0));
            assert_eq!(emu.get_last_error(), error);
        }
        assert_eq!(call(&mut emu, library, extended, &[0, 0, 0]), 0);
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
    }
}

#[test]
fn module_file_names_honor_capacity_and_report_utf16_lengths() {
    for (is64, library, wide) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [false, true]
    ) {
        let (mut emu, data) = prepare(is64);
        emu.base = 0x0040_0000;
        emu.cfg.exe_name = if wide { "test-🦀.exe" } else { "test.exe" }.into();
        let path = format!("C:\\{}", emu.cfg.exe_name);
        let name = if wide {
            "GetModuleFileNameW"
        } else {
            "GetModuleFileNameA"
        };
        let length = if wide {
            path.encode_utf16().count()
        } else {
            path.len()
        };
        let width = if wide { 2 } else { 1 };
        for capacity in [0, 1, 4, length, length + 1, length + 8] {
            emu.maps.memset(data, 0xcc, 256);
            emu.set_last_error(0);
            let returned = call(&mut emu, library, name, &[0, data, capacity as u64]);
            assert_eq!(returned, length.min(capacity) as u64);
            assert_eq!(
                emu.maps.read_byte(data + (capacity * width) as u64),
                Some(0xcc)
            );
            if capacity != 0 {
                let end = data + (length.min(capacity - 1) * width) as u64;
                assert_eq!(emu.maps.read_byte(end), Some(0));
                if wide {
                    assert_eq!(emu.maps.read_byte(end + 1), Some(0));
                }
            }
            if capacity <= length {
                assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
            }
        }
        assert_eq!(
            call(&mut emu, library, name, &[0, data, (length + 1) as u64]),
            length as u64
        );
        let actual = if wide {
            emu.maps.read_wide_string(data)
        } else {
            emu.maps.read_string(data)
        };
        assert_eq!(actual, path);
        assert_eq!(call(&mut emu, library, name, &[0xdead, data, 256]), 0);
        assert_eq!(emu.get_last_error(), ERROR_MOD_NOT_FOUND);
    }
}

#[test]
fn get_module_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    // HMODULE GetModuleHandleA(
    //   LPCSTR lpModuleName
    // );

    // "kernel32.dll"
    let name_addr = 0x20000;
    emu.maps
        .create_map("data", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "kernel32.dll");

    // Create the expected module map "kernel32.pe"
    emu.maps.create_map(
        "kernel32.pe",
        0x7FF10000000,
        0x10000,
        Permission::READ_EXECUTE,
    );

    let h_mod =
        helpers::call_winapi_export(&mut emu, "kernelbase.dll", "GetModuleHandleA", &[name_addr]);
    assert_eq!(
        h_mod, 0x7FF10000000,
        "GetModuleHandleA('kernel32.dll') returned incorrect base"
    );
}

#[test]
fn get_module_handle_32() {
    helpers::setup();
    let mut emu = emu32();

    let name_addr = 0x20000u64;
    emu.maps
        .create_map("data", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "kernel32.dll");

    emu.maps.create_map(
        "kernel32.dll",
        0x70000000,
        0x10000,
        Permission::READ_EXECUTE,
    );

    let h_mod =
        helpers::call_winapi_export(&mut emu, "kernelbase.dll", "GetModuleHandleA", &[name_addr]);
    assert_eq!(
        h_mod, 0x70000000,
        "GetModuleHandleA('kernel32.dll') returned incorrect base"
    );
}

#[test]
fn free_library_64() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::FreeLibrary, &[0x7FF00000]);
    assert_eq!(ret, 1, "FreeLibrary must return TRUE");
}

#[test]
fn free_library() {
    helpers::setup();
    let mut emu = emu64();

    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::FreeLibrary, &[0x7FF00000]);
    assert_eq!(ret, 1, "FreeLibrary should return TRUE");
}

#[test]
fn wide_resource_and_activation_handlers_are_reachable_on_x64() {
    let (mut emu, data) = prepare(true);
    let path = r"C:\guest-resource.dll";
    emu.maps.write_wide_string(data, path);
    let resource = call(&mut emu, "kernel32.dll", "BeginUpdateResourceW", &[data, 0]);
    assert_ne!(resource, 0);
    assert_eq!(helper::handler_get_uri(resource), path);
    assert_eq!(
        call(&mut emu, "kernel32.dll", "CloseHandle", &[resource]),
        1
    );
    // The existing activation-context model creates a handle without parsing ACTCTX.
    let context = call(&mut emu, "kernel32.dll", "CreateActCtxW", &[data]);
    assert!(helper::handler_exist(context));
    helper::handler_close(context);
}
