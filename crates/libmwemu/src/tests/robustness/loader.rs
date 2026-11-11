//! Emu loader robustness: missing/garbage files and the format classifier.

use crate::emu::Emu;
use crate::tests::helpers::{LOADER32, LOADER64};
use crate::{emu32, emu64};
use rs_header::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386, pe_machine_type};
use std::fs;
use std::io::Write;

fn write_temp(name: &str, bytes: &[u8]) -> String {
    let path = std::env::temp_dir().join(name);
    std::fs::File::create(&path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
    path.to_string_lossy().into_owned()
}

/// Build a minimal but recognizable PE image with the given COFF `Machine`
/// value, suitable for `pe_machine_type()` and the CLI classifier. The buffer
/// only needs to be big enough to cover the e_lfanew pointer (DOS header) plus
/// the PE signature (4 bytes) plus the Machine field (2 bytes at e_lfanew + 4).
fn build_minimal_pe(machine: u16) -> Vec<u8> {
    // e_lfanew must point somewhere inside the buffer. Use 0x40 so there's room
    // for a stub DOS header (real-world value) before the NT headers start.
    let e_lfanew: u32 = 0x40;
    let nt_off = e_lfanew as usize;
    let mut bytes = vec![0u8; nt_off + 6];
    bytes[0] = b'M';
    bytes[1] = b'Z';
    bytes[0x3c..0x40].copy_from_slice(&e_lfanew.to_le_bytes());
    // "PE\0\0" NT signature
    bytes[nt_off..nt_off + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());
    // COFF Machine field
    bytes[nt_off + 4..nt_off + 6].copy_from_slice(&machine.to_le_bytes());
    bytes
}

#[test]
fn load_missing_file_does_not_panic() {
    let mut emu = emu64();
    // The read fix logs and returns instead of panicking on a bad -f path.
    emu.load_code("/nonexistent/path/definitely/not/here.bin");
}

#[test]
fn classifier_detects_elf() {
    // Minimal ELF64 x86_64 header: magic + ELFCLASS64 + e_machine = EM_X86_64.
    let mut elf = vec![0u8; 20];
    elf[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    elf[4] = 2; // ELFCLASS64
    elf[18..20].copy_from_slice(&0x003eu16.to_le_bytes()); // EM_X86_64
    let path = write_temp("mwemu_rob_elf.bin", &elf);
    // ELF is a non-Windows guest → true; and --is_shellcode forces false.
    assert!(Emu::is_non_windows_file(&path, false));
    assert!(!Emu::is_non_windows_file(&path, true));
}

#[test]
fn classifier_treats_garbage_as_shellcode() {
    // Not ELF/PE/Mach-O → shellcode (Windows path) → false.
    let path = write_temp("mwemu_rob_sc.bin", &[0x90, 0x90, 0xc3]);
    assert!(!Emu::is_non_windows_file(&path, false));
}

#[test]
fn classifier_on_missing_file_does_not_panic() {
    // Unreadable file → empty bytes → not ELF/Mach-O → false, no panic.
    assert!(!Emu::is_non_windows_file("/nonexistent/xyz", false));
}

#[test]
fn detect_pe_arch_identifies_pe32_x86() {
    // Sanity-check the fixture itself, then the CLI classifier built on top.
    let bytes = build_minimal_pe(IMAGE_FILE_MACHINE_I386);
    assert_eq!(pe_machine_type(&bytes), Some(IMAGE_FILE_MACHINE_I386));
    let path = write_temp("mwemu_rob_pe32.bin", &bytes);
    assert_eq!(Emu::detect_pe_arch(&path), Some(IMAGE_FILE_MACHINE_I386));
}

#[test]
fn detect_pe_arch_identifies_pe64_x86_64() {
    let bytes = build_minimal_pe(IMAGE_FILE_MACHINE_AMD64);
    assert_eq!(pe_machine_type(&bytes), Some(IMAGE_FILE_MACHINE_AMD64));
    let path = write_temp("mwemu_rob_pe64.bin", &bytes);
    assert_eq!(Emu::detect_pe_arch(&path), Some(IMAGE_FILE_MACHINE_AMD64));
}

#[test]
fn detect_pe_arch_returns_none_for_non_pe() {
    // ELF/Mach-O/shellcode/missing → all `None`. The CLI's later handlers
    // (load_code, is_non_windows_file) decide what to do with these.
    let mut elf = vec![0u8; 20];
    elf[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    elf[4] = 2;
    let elf_path = write_temp("mwemu_rob_elf_for_pe.bin", &elf);
    assert_eq!(Emu::detect_pe_arch(&elf_path), None);

    let sc_path = write_temp("mwemu_rob_sc_for_pe.bin", &[0x90, 0x90, 0xc3]);
    assert_eq!(Emu::detect_pe_arch(&sc_path), None);

    // MZ magic without a valid e_lfanew → not a recognizable PE.
    let mut mz_only = vec![0u8; 0x40];
    mz_only[0] = b'M';
    mz_only[1] = b'Z';
    let mz_path = write_temp("mwemu_rob_mz_only.bin", &mz_only);
    assert_eq!(Emu::detect_pe_arch(&mz_path), None);

    // Missing file → None, no panic.
    assert_eq!(Emu::detect_pe_arch("/nonexistent/xyz"), None);
}

#[test]
fn pe32_x64_mismatch_only_fires_for_pe32_plus_dash_six() {
    let pe32 = write_temp(
        "mwemu_rob_mismatch_pe32.bin",
        &build_minimal_pe(IMAGE_FILE_MACHINE_I386),
    );
    let pe64 = write_temp(
        "mwemu_rob_mismatch_pe64.bin",
        &build_minimal_pe(IMAGE_FILE_MACHINE_AMD64),
    );
    let sc = write_temp("mwemu_rob_mismatch_sc.bin", &[0x90, 0x90, 0xc3]);

    // The case the CLI now rejects: PE32 + `-6`.
    assert!(Emu::pe32_x64_mismatch_error(&pe32, true).is_some());
    // PE64 + `-6` is fine — both are x86_64.
    assert!(Emu::pe32_x64_mismatch_error(&pe64, true).is_none());
    // PE32 without `-6` is the default 32-bit path, also fine.
    assert!(Emu::pe32_x64_mismatch_error(&pe32, false).is_none());
    // Non-PE inputs are out of scope for this check: load_code handles them.
    assert!(Emu::pe32_x64_mismatch_error(&sc, true).is_none());
    assert!(Emu::pe32_x64_mismatch_error("/nonexistent/xyz", true).is_none());
}

fn pe_header_image(is_x64: bool, header_size: u32) -> Vec<u8> {
    let fixture = if is_x64 { LOADER64 } else { LOADER32 };
    // Keep the section table and the first 512 bytes of .text.
    let mut raw = fixture[..0x600].to_vec();
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    let optional_header = nt + 24;
    let size_offset = optional_header + 60;
    raw[size_offset..size_offset + 4].copy_from_slice(&header_size.to_le_bytes());
    // No imports, relocations or TLS: the test needs no system DLLs or guest execution.
    let directories = optional_header + if is_x64 { 112 } else { 96 };
    raw[directories..directories + 16 * 8].fill(0);
    raw
}

fn check_pe_header_mapping(name: &str, is_x64: bool, load: impl Fn(&mut Emu, &str) -> u64) {
    for declared in [0x400, 0x600, 0x601, 0x1000] {
        let raw = pe_header_image(is_x64, declared);
        let path = write_temp(name, &raw);
        let mut emu = if is_x64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        emu.cfg.winver = None;
        let base = load(&mut emu, &path);
        assert_ne!(base, 0, "header size {declared:#x}");
        let header = emu.maps.get_mem_by_addr(base).unwrap();
        assert_eq!(header.get_base(), base);
        assert_eq!(header.size(), 0x1000, "preserve the aligned mapped size");
        let copied = (declared as usize).min(raw.len());
        assert_eq!(&header.get_mem()[..copied], &raw[..copied]);
        assert!(header.get_mem()[copied..].iter().all(|&byte| byte == 0));
        let section = emu.maps.get_mem_by_addr(base + 0x1000).unwrap();
        assert_eq!(&section.get_mem()[..0x200], &raw[0x400..0x600]);
        fs::remove_file(path).unwrap();
    }
}

#[test]
fn pe32_headers_load_available_bytes_and_zero_fill() {
    check_pe_header_mapping("mwemu_rob_headers32.exe", false, |emu, path| {
        emu.load_pe32(path, false, 0x400000).unwrap().base
    });
}

#[test]
fn pe64_headers_load_available_bytes_and_zero_fill() {
    check_pe_header_mapping("mwemu_rob_headers64.exe", true, |emu, path| {
        emu.load_pe64(path, false, 0x140000000).unwrap().base
    });
}

#[test]
fn pe64_dll_headers_load_available_bytes_and_zero_fill() {
    check_pe_header_mapping("mwemu_rob_headers64.dll", true, |emu, path| {
        emu.map_dll_pe64(path).unwrap().base
    });
}
