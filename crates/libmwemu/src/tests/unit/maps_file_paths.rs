use crate::windows::constants::errors::*;
use std::fs;
use std::path::{MAIN_SEPARATOR, Path};

use itertools::iproduct;
use tempfile::Builder;

use crate::config::Config;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::windows::syscall64::memory::nt_open_section;

use crate::windows::peb::{peb32, peb64};
use crate::{emu32, emu64};

fn folder_forms(folder: &Path) -> Vec<String> {
    let canonical = fs::canonicalize(folder).unwrap();
    let working_dir = fs::canonicalize(".").unwrap();
    let relative = canonical.strip_prefix(&working_dir).unwrap();
    let ordinary = dunce::simplified(&canonical).to_str().unwrap();
    let mut forms = vec![
        relative.to_str().unwrap().to_string(),
        folder.to_str().unwrap().to_string(),
        ordinary.to_string(),
        format!("{ordinary}{MAIN_SEPARATOR}"),
        canonical.to_str().unwrap().to_string(),
        format!("{}{MAIN_SEPARATOR}", canonical.display()),
    ];
    #[cfg(windows)]
    forms.push(format!("{ordinary}/"));
    forms.sort();
    forms.dedup();
    forms
}

#[test]
fn maps_file_paths_read_relative_absolute_and_canonical_folders() {
    let dir = Builder::new()
        .prefix("mwemu maps ü ")
        .tempdir_in(".")
        .unwrap();
    fs::write(dir.path().join("probe.dll"), b"maps fixture").unwrap();
    for folder in folder_forms(dir.path()) {
        let cfg = Config {
            maps_folder: folder,
            ..Config::new()
        };
        assert_eq!(
            fs::read(cfg.get_maps_folder("probe.dll")).unwrap(),
            b"maps fixture"
        );
    }
    // An empty maps folder refers to the working directory, not the filesystem root.
    assert_eq!(Config::new().get_maps_folder("probe.dll"), "probe.dll");
}

fn dll_image(is_x64: bool) -> Vec<u8> {
    let mut emu = if is_x64 { emu64() } else { emu32() };
    if is_x64 {
        assert_ne!(emu.create_stub_pe64("probe.dll", &["ProbeExport"]), 0);
    } else {
        assert_ne!(emu.create_stub_pe32("probe.dll", &["ProbeExport"]), 0);
    }
    // Stub sections have identical file offsets and RVAs, so they can be saved directly.
    let mut image: Vec<_> = ["probe.pe", "probe.text", "probe.edata"]
        .into_iter()
        .flat_map(|name| emu.maps.get_mem(name).get_mem())
        .collect();
    let pe_offset = u32::from_le_bytes(image[0x3c..0x40].try_into().unwrap()) as usize;
    // Distinguish this disk image from an automatically generated fallback stub.
    image[pe_offset + 8..pe_offset + 12].copy_from_slice(&0x1234_5678u32.to_le_bytes());
    image
}

#[test]
fn maps_file_paths_load_new_dlls_on_both_architectures() {
    let dir = Builder::new()
        .prefix("mwemu maps ü ")
        .tempdir_in(".")
        .unwrap();
    let names = [
        "probe.dll",
        r"C:\Windows\System32\PROBE.DLL",
        "/guest/system/probe.dll",
    ];
    for (is_x64, folder, name) in iproduct!([false, true], folder_forms(dir.path()), names) {
        fs::write(dir.path().join("probe.dll"), dll_image(is_x64)).unwrap();
        let mut emu = if is_x64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        emu.cfg.winver = None;
        emu.cfg.maps_folder = folder.clone();
        if is_x64 {
            peb64::init_peb(&mut emu);
        } else {
            peb32::init_peb(&mut emu);
        }
        assert!(emu.maps.get_map_by_name("probe.pe").is_none());
        let base = emu.link_library(name);
        assert_ne!(base, 0, "x64={is_x64} folder={folder} name={name}");
        let pe_offset = u64::from(emu.maps.read_dword(base + 0x3c).unwrap());
        assert_eq!(emu.maps.read_dword(base + pe_offset + 8), Some(0x1234_5678));
        assert_eq!(emu.link_library("probe.dll"), base);
        for invalid in ["", "C:probe.dll", "probe.dll:stream", r"C:\Windows\"] {
            assert_eq!(emu.link_library(invalid), 0, "{invalid}");
        }
    }
}

fn open_known_dll(emu: &mut Emu, name: &str, relative: bool) -> u64 {
    let args = emu.alloc("section_args", 0x1000, Permission::READ_WRITE);
    let object_attributes = args + 0x10;
    let unicode_string = args + 0x50;
    let buffer = args + 0x100;
    if relative {
        let directory = 0x1234;
        emu.known_dll_dir_handles.insert(directory);
        assert!(emu.maps.write_qword(object_attributes + 8, directory));
    }
    assert!(
        emu.maps
            .write_qword(object_attributes + 0x10, unicode_string)
    );
    assert!(
        emu.maps
            .write_word(unicode_string, (name.encode_utf16().count() * 2) as u16)
    );
    assert!(emu.maps.write_qword(unicode_string + 8, buffer));
    emu.maps.write_wide_string(buffer, name);
    emu.regs_mut().rcx = args;
    emu.regs_mut().r8 = object_attributes;
    nt_open_section(emu);
    emu.maps.read_qword(args).unwrap()
}

#[test]
fn maps_file_paths_open_known_dlls_only_inside_maps() {
    let dir = Builder::new()
        .prefix("mwemu maps ü ")
        .tempdir_in(".")
        .unwrap();
    fs::write(dir.path().join("probe.dll"), b"section fixture").unwrap();
    for (folder, relative) in iproduct!(folder_forms(dir.path()), [false, true]) {
        let mut emu = emu64();
        emu.cfg.maps_folder = folder;
        let name = if relative {
            "probe.dll"
        } else {
            r"\KnownDlls\probe.dll"
        };
        let handle = open_known_dll(&mut emu, name, relative);
        assert_eq!(emu.regs().rax, STATUS_SUCCESS);
        assert_eq!(
            emu.section_handles.get(&handle).map(String::as_str),
            Some("probe.dll")
        );
    }
    let outside = fs::canonicalize(dir.path().join("probe.dll")).unwrap();
    for name in [
        outside.to_str().unwrap(),
        ".",
        "..",
        "../probe.dll",
        r"..\probe.dll",
        "probe.dll:stream",
    ] {
        let mut emu = emu64();
        emu.cfg.maps_folder = dir.path().to_str().unwrap().to_string();
        assert_eq!(open_known_dll(&mut emu, name, true), 0);
        assert_eq!(emu.regs().rax, STATUS_OBJECT_NAME_NOT_FOUND, "{name}");
    }
}

#[test]
fn maps_file_paths_load_banzai_from_canonical_folder() {
    let dir = Builder::new().prefix("mwemu maps ü ").tempdir().unwrap();
    // A header keeps this test independent of the separate CSV header-handling fix.
    fs::write(
        dir.path().join("banzai.csv"),
        "api,params\nMapsPathProbe,3\n",
    )
    .unwrap();
    let mut emu = emu32();
    emu.cfg.console_enabled = false;
    emu.cfg.winver = None;
    emu.cfg.skip_unimplemented = true;
    emu.cfg.maps_folder = fs::canonicalize(dir.path())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    emu.init_win32(false, false).unwrap();
    assert_eq!(emu.banzai.get_params("MapsPathProbe"), 3);
}
