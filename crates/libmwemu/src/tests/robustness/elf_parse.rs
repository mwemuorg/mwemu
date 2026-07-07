//! rs-header ELF parser robustness — a malformed/hostile ELF must not panic.

use rs_header::elf::elf32::Elf32;
use rs_header::elf::elf64::Elf64;

#[test]
fn detect_rejects_garbage() {
    assert!(!Elf64::is_elf(b""));
    assert!(!Elf64::is_elf(b"MZ\x90\x90"));
    assert!(!Elf64::is_elf(&[0u8; 64]));
    assert!(!Elf32::is_elf32(&[0xffu8; 64]));
}

#[test]
fn parse_empty_returns_err() {
    assert!(Elf64::parse(&[]).is_err());
    assert!(Elf32::parse(&[]).is_err());
}

#[test]
fn parse_magic_only_does_not_panic() {
    // \x7fELF magic followed by zeros: parse must fail gracefully, not panic.
    let mut bytes = vec![0x7f, b'E', b'L', b'F'];
    bytes.extend_from_slice(&[0u8; 256]);
    let _ = Elf64::parse(&bytes);
    let _ = Elf32::parse(&bytes);
}

#[test]
fn parse_bogus_offsets_does_not_panic() {
    // Valid magic + ELFCLASS64, but program/section header offsets are absurd.
    let mut bytes = vec![0u8; 64];
    bytes[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    bytes[4] = 2; // ELFCLASS64
    bytes[5] = 1; // little endian
    // e_phoff / e_shoff (offsets 32 and 40) → huge
    bytes[32..40].copy_from_slice(&0x7fff_ffff_ffff_ffffu64.to_le_bytes());
    bytes[40..48].copy_from_slice(&0x7fff_ffff_ffff_ffffu64.to_le_bytes());
    let _ = Elf64::parse(&bytes);
}
