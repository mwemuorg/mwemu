use std::collections::HashSet;
use std::path::Path;

use goblin::pe::dll_characteristic::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE;
use goblin::pe::header::{Header, SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC};
use goblin::pe::section_table::SIZEOF_SECTION_TABLE;
use itertools::iproduct;

use crate::ApiMode;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{Allocation, MemoryKind, PAGE_SIZE, PageState};
use crate::serialization::Serialization;
use crate::tests::helpers::{
    self, WINAPI_MODULE_BASE, call_winapi_export as call, prepare_winapi as prepare,
};
use crate::winapi::common::file_names::read_path;
use crate::winapi::policy::{ApiDisposition, ApiStopReason};
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;
use crate::windows::peb::{peb32, peb64};
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
    let peb = structures::Peb64::load(peb_addr, &emu.maps);
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
    let peb = structures::Peb::load(peb_addr, &emu.maps);
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
            (
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS as u64,
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
        emu.cfg.exe_name = if wide { "test-🦀.exe" } else { "café.exe" }.into();
        let path = format!("C:\\{}", emu.cfg.exe_name);
        let name = if wide {
            "GetModuleFileNameW"
        } else {
            "GetModuleFileNameA"
        };
        let length = if wide {
            path.encode_utf16().count()
        } else {
            path.chars().count()
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
            assert_eq!(emu.maps.read_byte(data + 6), Some(0xe9));
            read_path(&emu, data, false).unwrap()
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
fn free_library_rejects_unknown_modules_and_preserves_pinned_system_images() {
    for wide in [false, true] {
        let (mut emu, _) = prepare(wide);
        let lib = "kernel32.dll";
        assert_eq!(
            helpers::call_winapi_export(&mut emu, lib, "FreeLibrary", &[0x7ff00000]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_MOD_NOT_FOUND);
        let base = emu.export_indexes.get_by_name(lib).unwrap().base;
        assert_eq!(
            helpers::call_winapi_export(&mut emu, lib, "FreeLibrary", &[base]),
            1
        );
        assert!(emu.export_indexes.get_by_base(base).is_some());
    }
}

#[test]
fn resource_updates_and_activation_contexts_use_policy() {
    let (mut emu, data) = prepare(true);
    let path = data + 128;
    emu.maps.write_wide_string(path, r"C:\guest-resource.dll");
    emu.maps.write_dword(data, 56);
    emu.maps.write_qword(data + 8, path);
    emu.set_api_mode(ApiMode::Strict);
    for (api, args) in [
        ("BeginUpdateResourceW", &[path, 0][..]),
        ("CreateActCtxW", &[data][..]),
    ] {
        helpers::call_winapi64(
            &mut emu,
            |emu| helpers::invoke_winapi_export(emu, "kernel32.dll", api),
            args,
        );
        assert_eq!(
            emu.api_policy
                .last_outcome
                .as_ref()
                .map(|outcome| &outcome.disposition),
            Some(&ApiDisposition::Stopped(ApiStopReason::StrictPolicy))
        );
        emu.api_policy.clear_outcome();
    }
}

const GUEST_CALLER: u64 = 0x0040_0000;
const GUEST_DLL_MAIN: u32 = 0x1080;

fn guest_dll_image(is64: bool, preferred: u64, accept_attach: bool) -> Vec<u8> {
    let mut template = if is64 { emu64() } else { emu32() };
    let base = if is64 {
        template.create_stub_pe64("probe.dll", &["Answer"])
    } else {
        template.create_stub_pe32("probe.dll", &["Answer"]) as u64
    };
    let size = template.maps.allocation_at(base).unwrap().size as usize;
    // Stub images use RVAs as their file offsets, so the mapped bytes form a PE file.
    let mut raw = template
        .maps
        .try_read_bytes(base, size)
        .unwrap()
        .into_owned();
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    let opt = nt + SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER;
    raw[opt + 16..opt + 20].copy_from_slice(&GUEST_DLL_MAIN.to_le_bytes());
    if is64 {
        raw[opt + 24..opt + 32].copy_from_slice(&preferred.to_le_bytes());
    } else {
        raw[opt + 28..opt + 32].copy_from_slice(&(preferred as u32).to_le_bytes());
    }

    let answer = (template.export_indexes.resolve_name_by_base(base, "Answer") - base) as usize;
    raw[answer..answer + 6].copy_from_slice(&[0xb8, 42, 0, 0, 0, 0xc3]); // mov eax,42; ret
    let entry = GUEST_DLL_MAIN as usize;
    raw[entry..entry + 5].copy_from_slice(&[0xb8, u8::from(accept_attach), 0, 0, 0]);
    let ret = if is64 {
        &[0xc3][..]
    } else {
        &[0xc2, 12, 0][..]
    };
    raw[entry + 5..entry + 5 + ret.len()].copy_from_slice(ret);
    raw
}

fn guest_dll_caller(is64: bool) -> (Emu, u64, u64) {
    let (mut emu, data) = prepare(is64);
    if is64 {
        peb64::init_peb(&mut emu);
    } else {
        peb32::init_peb(&mut emu);
    }
    emu.maps
        .create_map(
            "caller",
            GUEST_CALLER,
            PAGE_SIZE,
            Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    assert!(emu.maps.write_bytes(GUEST_CALLER, &[0xff, 0xd0, 0x90])); // call eax/rax; nop
    emu.set_pc(GUEST_CALLER);
    let base = if is64 { LIBS64_MIN } else { LIBS32_MIN } + 0x0100_0000;
    (emu, data, base)
}

#[test]
fn guest_dll_exports_execute_and_unload_at_the_last_reference() {
    for is64 in [false, true] {
        const DLL_CHARACTERISTICS_OFFSET: usize = 70;
        let (mut emu, data, preferred) = guest_dll_caller(is64);
        emu.set_seed(1);
        let mut plugin = guest_dll_image(is64, preferred, true);
        let header = Header::parse(&plugin).unwrap();
        let flags = header.dos_header.pe_pointer as usize
            + SIZEOF_PE_MAGIC
            + SIZEOF_COFF_HEADER
            + DLL_CHARACTERISTICS_OFFSET;
        plugin[flags..flags + 2]
            .copy_from_slice(&IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE.to_le_bytes());
        let path = r"C:\app\plugin.probe.dll";
        let base = emu.load_guest_dll(path, plugin).unwrap();
        assert_ne!(base, preferred);
        assert_eq!(emu.retain_guest_dll("PLUGIN.PROBE.DLL"), Some(base));
        let sibling = preferred;
        assert_eq!(
            emu.load_guest_dll(r"C:\app\sibling.dll", guest_dll_image(is64, sibling, true))
                .unwrap(),
            sibling
        );
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        emu.maps.write_string(data, "Answer");
        let address = call(&mut emu, "kernel32.dll", "GetProcAddress", &[base, data]);
        assert!(address > base);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetProcAddress", &[base, 1]),
            address
        );
        emu.regs_mut().rax = address;
        emu.set_pc(GUEST_CALLER);
        assert_eq!(emu.run(Some(GUEST_CALLER + 2)).unwrap(), GUEST_CALLER + 2);
        assert_eq!(emu.regs().rax, 42);
        assert_eq!(call(&mut emu, "kernel32.dll", "FreeLibrary", &[base]), 1);
        assert!(emu.maps.is_mapped(address));
        assert_eq!(call(&mut emu, "kernel32.dll", "FreeLibrary", &[base]), 1);
        assert!(!emu.maps.is_mapped(address));
        assert!(emu.export_indexes.get_by_base(base).is_none());
        assert!(emu.maps.allocation_at(base).is_none());
        let find = if is64 {
            peb64::get_module_base
        } else {
            peb32::get_module_base
        };
        assert_eq!(find("plugin.probe.dll", &mut emu), None);
        assert_eq!(find("sibling.dll", &mut emu), Some(sibling));
        assert_eq!(
            call(&mut emu, "kernel32.dll", "GetProcAddress", &[base, data]),
            0
        );
    }
}

#[test]
fn guest_dll_attach_failure_runs_detach_and_rolls_back_the_load() {
    for is64 in [false, true] {
        let (mut emu, output, base) = guest_dll_caller(is64);
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if pc == base + GUEST_DLL_MAIN as u64 {
                let reason = if is64 {
                    emu.regs().rdx as u32
                } else {
                    emu.maps.read_dword(emu.sp() + 8).unwrap()
                };
                let count = emu.maps.read_dword(output).unwrap();
                emu.maps.write_dword(output, count + 1);
                emu.maps.write_dword(output + 4 + count as u64 * 4, reason);
            }
            true
        });
        let err = emu
            .load_guest_dll("rejected.dll", guest_dll_image(is64, base, false))
            .unwrap_err();
        assert_eq!(err.win32_error(), ERROR_DLL_INIT_FAILED);
        assert_eq!(emu.maps.read_dword(output), Some(2));
        assert_eq!(emu.maps.read_dword(output + 4), Some(DLL_PROCESS_ATTACH));
        assert_eq!(emu.maps.read_dword(output + 8), Some(DLL_PROCESS_DETACH));
        assert!(emu.maps.allocation_at(base).is_none());
        assert!(emu.export_indexes.get_by_base(base).is_none());
        assert!(emu.guest_dll_base("rejected.dll").is_none());
        assert_eq!(emu.pc(), GUEST_CALLER);
    }
}

#[test]
fn malformed_guest_dll_does_not_publish_a_partial_image() {
    for is64 in [false, true] {
        let (mut emu, _, base) = guest_dll_caller(is64);
        let mut raw = guest_dll_image(is64, base, true);
        let header = Header::parse(&raw).unwrap();
        let coff = header.dos_header.pe_pointer as usize + SIZEOF_PE_MAGIC;
        let sections =
            coff + SIZEOF_COFF_HEADER + header.coff_header.size_of_optional_header as usize;
        // A third section aliases the export section without changing its file contents.
        let third = sections + 2 * SIZEOF_SECTION_TABLE;
        raw.copy_within(third - SIZEOF_SECTION_TABLE..third, third);
        raw[coff + 2..coff + 4].copy_from_slice(&3u16.to_le_bytes());
        let err = emu.load_guest_dll("broken.dll", raw).unwrap_err();
        assert_eq!(err.win32_error(), ERROR_BAD_EXE_FORMAT);
        assert!(err.to_string().contains("cannot map PE"), "{err}");
        assert!(emu.maps.is_range_free(base, PAGE_SIZE * 3));
        assert!(emu.guest_dll_base("broken.dll").is_none());
        assert!(emu.export_indexes.get_by_base(base).is_none());
        assert_eq!(emu.pc(), GUEST_CALLER);
    }
}
