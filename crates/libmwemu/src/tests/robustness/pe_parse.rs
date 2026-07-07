//! rs-header PE parser robustness — a malformed/hostile PE must not panic.

use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

#[test]
fn detect_rejects_garbage() {
    assert!(!PE64::is_pe64(b""));
    assert!(!PE64::is_pe64(b"not a pe at all"));
    assert!(!PE64::is_pe64(&[0u8; 512]));
    assert!(!PE64::is_pe64(b"MZ")); // MZ magic but truncated
    assert!(!PE32::is_pe32(b""));
    assert!(!PE32::is_pe32(&[0xffu8; 512]));
}

#[test]
fn parse_empty_does_not_panic() {
    let _ = PE64::parse("empty", &[]);
    let _ = PE32::parse("empty", &[]);
}

#[test]
fn parse_truncated_mz_does_not_panic() {
    let mut bytes = vec![b'M', b'Z'];
    bytes.extend_from_slice(&[0u8; 256]);
    let _ = PE64::parse("trunc.exe", &bytes);
    let _ = PE32::parse("trunc.exe", &bytes);
}

#[test]
fn parse_bogus_pe_offset_does_not_panic() {
    // MZ header whose e_lfanew (PE header offset at 0x3c) points out of bounds.
    let mut bytes = vec![0u8; 0x40];
    bytes[0] = b'M';
    bytes[1] = b'Z';
    // e_lfanew = 0x7fffffff (way past the buffer)
    bytes[0x3c..0x40].copy_from_slice(&0x7fff_ffffu32.to_le_bytes());
    let _ = PE64::parse("bogus.exe", &bytes);
    let _ = PE32::parse("bogus.exe", &bytes);
}
