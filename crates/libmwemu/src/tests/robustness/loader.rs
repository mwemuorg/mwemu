//! Emu loader robustness: missing/garbage files and the format classifier.

use crate::emu::Emu;
use crate::emu64;
use std::io::Write;

fn write_temp(name: &str, bytes: &[u8]) -> String {
    let path = std::env::temp_dir().join(name);
    std::fs::File::create(&path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
    path.to_string_lossy().into_owned()
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
