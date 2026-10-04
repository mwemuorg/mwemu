//! Linux libc stubs, called directly through the libc gateway.
use super::unix_api_helpers::{aarch64_env, put_str, ret, set_args, stdout_text};
use crate::api::linux::libc::gateway;

#[test]
fn string_length_and_compare() {
    let (mut emu, d) = aarch64_env();
    let a = put_str(&mut emu, d, "hello");
    let b = put_str(&mut emu, d + 0x100, "help");
    set_args(&mut emu, &[a]);
    gateway("strlen", &mut emu);
    assert_eq!(ret(&emu), 5);

    set_args(&mut emu, &[a, b]);
    gateway("strcmp", &mut emu);
    assert!((ret(&emu) as i64) < 0, "'l' < 'p'");

    set_args(&mut emu, &[a, b, 3]);
    gateway("strncmp", &mut emu);
    assert_eq!(ret(&emu), 0);

    set_args(&mut emu, &[a, a]);
    gateway("strcmp", &mut emu);
    assert_eq!(ret(&emu), 0);
}

#[test]
fn string_copy_concat_and_search() {
    let (mut emu, d) = aarch64_env();
    let src = put_str(&mut emu, d, "abc");
    let dst = d + 0x100;
    set_args(&mut emu, &[dst, src]);
    gateway("strcpy", &mut emu);
    assert_eq!(ret(&emu), dst);
    let tail = put_str(&mut emu, d + 0x200, "def");
    set_args(&mut emu, &[dst, tail]);
    gateway("strcat", &mut emu);
    assert_eq!(emu.maps.read_string(dst), "abcdef");

    set_args(&mut emu, &[dst, b'd' as u64]);
    gateway("strchr", &mut emu);
    assert_eq!(ret(&emu), dst + 3);
    set_args(&mut emu, &[dst, b'z' as u64]);
    gateway("strchr", &mut emu);
    assert_eq!(ret(&emu), 0);

    let needle = put_str(&mut emu, d + 0x300, "cde");
    set_args(&mut emu, &[dst, needle]);
    gateway("strstr", &mut emu);
    assert_eq!(ret(&emu), dst + 2);

    set_args(&mut emu, &[dst]);
    gateway("strdup", &mut emu);
    let dup = ret(&emu);
    assert_ne!(dup, dst);
    assert_eq!(emu.maps.read_string(dup), "abcdef");
}

#[test]
fn strncpy_pads_with_nul() {
    let (mut emu, d) = aarch64_env();
    let src = put_str(&mut emu, d, "ab");
    let dst = d + 0x100;
    emu.maps.memset(dst, 0x41, 8);
    set_args(&mut emu, &[dst, src, 5]);
    gateway("strncpy", &mut emu);
    assert_eq!(emu.maps.read_bytes(dst, 6), b"ab\0\0\0A");
}

#[test]
fn memory_functions() {
    let (mut emu, d) = aarch64_env();
    emu.maps.write_bytes(d, b"0123456789");
    set_args(&mut emu, &[d + 2, d, 5]);
    gateway("memmove", &mut emu); // overlapping copy
    assert_eq!(emu.maps.read_bytes(d, 10), b"0101234789");

    set_args(&mut emu, &[d, b'x' as u64, 3]);
    gateway("memset", &mut emu);
    assert_eq!(emu.maps.read_bytes(d, 4), b"xxx1");

    set_args(&mut emu, &[d, b'1' as u64, 10]);
    gateway("memchr", &mut emu);
    assert_eq!(ret(&emu), d + 3);

    emu.maps.write_bytes(d + 0x100, b"xxx2");
    set_args(&mut emu, &[d, d + 0x100, 4]);
    gateway("memcmp", &mut emu);
    assert!((ret(&emu) as i64) < 0);

    set_args(&mut emu, &[d, 2]);
    gateway("bzero", &mut emu);
    assert_eq!(emu.maps.read_bytes(d, 3), b"\0\0x");
}

#[test]
fn integer_parsing() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "  -0x1Fz");
    let endp = d + 0x100;
    set_args(&mut emu, &[s, endp, 0]);
    gateway("strtol", &mut emu);
    assert_eq!(ret(&emu) as i64, -31);
    assert_eq!(emu.maps.read_qword(endp), Some(s + 7));

    let n = put_str(&mut emu, d + 0x200, "42abc");
    set_args(&mut emu, &[n]);
    gateway("atoi", &mut emu);
    assert_eq!(ret(&emu), 42);
}

#[test]
fn printf_family_uses_register_varargs() {
    let (mut emu, d) = aarch64_env();
    let fmt = put_str(&mut emu, d, "%d-%s-%x");
    let word = put_str(&mut emu, d + 0x100, "hi");
    set_args(&mut emu, &[fmt, 42, word, 255]);
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "42-hi-ff");
    assert_eq!(ret(&emu), 8);

    let buf = d + 0x200;
    let fmt2 = put_str(&mut emu, d + 0x300, "value=%05d");
    set_args(&mut emu, &[buf, 6, fmt2, 7]);
    gateway("snprintf", &mut emu);
    assert_eq!(ret(&emu), 11, "returns the untruncated length");
    assert_eq!(emu.maps.read_string(buf), "value");
}

#[test]
fn printf_spills_ninth_argument_to_stack() {
    let (mut emu, d) = aarch64_env();
    let fmt = put_str(&mut emu, d, "%d %d %d %d %d %d %d %d");
    set_args(&mut emu, &[fmt, 1, 2, 3, 4, 5, 6, 7]);
    let sp = emu.regs_aarch64().sp;
    emu.maps.write_qword(sp, 8);
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "1 2 3 4 5 6 7 8");
}

#[test]
fn puts_and_write_reach_stdout() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "line");
    set_args(&mut emu, &[s]);
    gateway("puts", &mut emu);
    set_args(&mut emu, &[1, s, 2]);
    gateway("write", &mut emu);
    assert_eq!(ret(&emu), 2);
    set_args(&mut emu, &[7, s, 4]);
    gateway("write", &mut emu); // not stdout/stderr
    assert_eq!(stdout_text(&emu), "line\nli");
}

#[test]
fn heap_allocation_round_trip() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[4, 8]);
    gateway("calloc", &mut emu);
    let p = ret(&emu);
    assert_ne!(p, 0);
    assert_eq!(emu.maps.read_qword(p + 24), Some(0));
    emu.maps.write_qword(p, 0x1122_3344);

    set_args(&mut emu, &[p, 64]);
    gateway("realloc", &mut emu);
    let q = ret(&emu);
    assert_eq!(emu.maps.read_qword(q), Some(0x1122_3344));
    assert!(!emu.maps.is_mapped(p), "old block released");

    set_args(&mut emu, &[q]);
    gateway("free", &mut emu);
    assert!(!emu.maps.is_mapped(q));
}

#[test]
fn mmap_and_munmap() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[0, 0x2000, 3, 0x22, u64::MAX, 0]);
    gateway("mmap", &mut emu);
    let p = ret(&emu);
    assert_ne!(p, u64::MAX);
    assert!(emu.maps.write_qword(p + 0x1ff8, 5));

    set_args(&mut emu, &[p, 0x2000]);
    gateway("munmap", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert!(!emu.maps.is_mapped(p));
}

#[test]
fn errno_location_is_stable() {
    let (mut emu, _) = aarch64_env();
    gateway("__errno_location", &mut emu);
    let first = ret(&emu);
    assert!(emu.maps.write_dword(first, 22));
    gateway("__errno_location", &mut emu);
    assert_eq!(ret(&emu), first);
    assert_eq!(emu.maps.read_dword(first), Some(22));
}

#[test]
fn process_and_terminal_queries() {
    let (mut emu, d) = aarch64_env();
    let name = put_str(&mut emu, d, "HOME");
    set_args(&mut emu, &[name]);
    gateway("getenv", &mut emu);
    assert_eq!(ret(&emu), 0);

    gateway("getpid", &mut emu);
    assert_eq!(ret(&emu), 1234);

    set_args(&mut emu, &[1]);
    gateway("isatty", &mut emu);
    assert_eq!(ret(&emu), 1);

    let ws = d + 0x100;
    set_args(&mut emu, &[1, 0x5413, ws]);
    gateway("ioctl", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert_eq!(emu.maps.read_word(ws), Some(24));
    assert_eq!(emu.maps.read_word(ws + 2), Some(80));

    let tloc = d + 0x200;
    set_args(&mut emu, &[tloc]);
    gateway("time", &mut emu);
    assert!(ret(&emu) > 1_600_000_000);
    assert_eq!(emu.maps.read_qword(tloc), Some(ret(&emu)));
}

#[test]
fn unknown_symbol_returns_zero_without_panicking() {
    let (mut emu, _) = aarch64_env();
    emu.regs_aarch64_mut().x[0] = 0xdead;
    gateway("definitely_not_a_libc_symbol", &mut emu);
    assert_eq!(ret(&emu), 0);
}

#[test]
fn x86_64_sysv_arguments() {
    crate::tests::helpers::setup();
    let mut emu = crate::emu64();
    let d = super::unix_api_helpers::scratch(&mut emu, "api_data", 0x1000);
    let fmt = put_str(&mut emu, d, "%s=%d");
    let key = put_str(&mut emu, d + 0x100, "k");
    emu.regs_mut().rdi = fmt;
    emu.regs_mut().rsi = key;
    emu.regs_mut().rdx = 9;
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "k=9");
    assert_eq!(emu.regs().rax, 3);
}
