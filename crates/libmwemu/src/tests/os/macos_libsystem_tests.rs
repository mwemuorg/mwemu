//! macOS libSystem stubs, called directly through the libsystem gateway.
use super::unix_api_helpers::{aarch64_env, put_str, ret, set_args, stdout_text};
use crate::api::macos::libsystem::gateway;

/// Apple AArch64 ABI: variadic arguments always live on the stack.
fn push_varargs(emu: &mut crate::emu::Emu, args: &[u64]) {
    let sp = emu.regs_aarch64().sp;
    for (i, a) in args.iter().enumerate() {
        emu.maps.write_qword(sp + i as u64 * 8, *a);
    }
}

#[test]
fn printf_reads_varargs_from_stack() {
    let (mut emu, d) = aarch64_env();
    let fmt = put_str(&mut emu, d, "%s:%d:%c");
    let word = put_str(&mut emu, d + 0x100, "pid");
    set_args(&mut emu, &[fmt, 0xbad, 0xbad]); // registers must be ignored
    push_varargs(&mut emu, &[word, 1234, b'R' as u64]);
    gateway("_printf", &mut emu);
    assert_eq!(stdout_text(&emu), "pid:1234:R");
}

#[test]
fn snprintf_truncates_and_reports_full_length() {
    let (mut emu, d) = aarch64_env();
    let buf = d + 0x200;
    let fmt = put_str(&mut emu, d, "%5d|");
    set_args(&mut emu, &[buf, 4, fmt]);
    push_varargs(&mut emu, &[42]);
    gateway("_snprintf", &mut emu);
    assert_eq!(ret(&emu), 6);
    assert_eq!(emu.maps.read_string(buf), "   ");
}

#[test]
fn fmtcheck_returns_the_suspect_format() {
    let (mut emu, d) = aarch64_env();
    let suspect = put_str(&mut emu, d, "%5d");
    let default = put_str(&mut emu, d + 0x100, "%d");
    set_args(&mut emu, &[suspect, default]);
    gateway("_fmtcheck", &mut emu);
    assert_eq!(ret(&emu), suspect);
}

#[test]
fn strvis_copies_printable_text() {
    let (mut emu, d) = aarch64_env();
    let src = put_str(&mut emu, d, "(launchd)");
    let dst = d + 0x100;
    set_args(&mut emu, &[dst, src, 0]);
    gateway("_strvis", &mut emu);
    assert_eq!(ret(&emu), 9);
    assert_eq!(emu.maps.read_string(dst), "(launchd)");
}

#[test]
fn ioctl_reports_terminal_size() {
    let (mut emu, d) = aarch64_env();
    set_args(&mut emu, &[1, 0x40087468, d]); // TIOCGWINSZ
    gateway("_ioctl", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert_eq!(emu.maps.read_word(d), Some(24));
    assert_eq!(emu.maps.read_word(d + 2), Some(80));

    set_args(&mut emu, &[1, 0x1234, d]);
    gateway("_ioctl", &mut emu);
    assert_eq!(ret(&emu) as i64, -1);
}

#[test]
fn strtol_family_honours_sign_base_and_endptr() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "-0x10 rest");
    let endp = d + 0x100;
    set_args(&mut emu, &[s, endp, 16]);
    gateway("_strtol", &mut emu);
    assert_eq!(ret(&emu) as i64, -16);
    assert_eq!(emu.maps.read_qword(endp), Some(s + 5));

    let oct = put_str(&mut emu, d + 0x200, "0755");
    set_args(&mut emu, &[oct, 0, 0]);
    gateway("_strtoul", &mut emu);
    assert_eq!(ret(&emu), 0o755);

    let n = put_str(&mut emu, d + 0x300, " 99 bottles");
    set_args(&mut emu, &[n]);
    gateway("_atoi", &mut emu);
    assert_eq!(ret(&emu), 99);
}

#[test]
fn strtod_returns_in_d0() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "2.75xyz");
    let endp = d + 0x100;
    set_args(&mut emu, &[s, endp]);
    gateway("_strtod", &mut emu);
    assert_eq!(f64::from_bits(emu.regs_aarch64().v[0] as u64), 2.75);
    assert_eq!(emu.maps.read_qword(endp), Some(s + 4));
}

#[test]
fn case_insensitive_compare_and_case_mapping() {
    let (mut emu, d) = aarch64_env();
    let a = put_str(&mut emu, d, "HeLLo");
    let b = put_str(&mut emu, d + 0x100, "hello world");
    set_args(&mut emu, &[a, b]);
    gateway("_strcasecmp", &mut emu);
    assert!((ret(&emu) as i64) < 0);
    set_args(&mut emu, &[a, b, 5]);
    gateway("_strncasecmp", &mut emu);
    assert_eq!(ret(&emu), 0);

    set_args(&mut emu, &[b'q' as u64]);
    gateway("_toupper", &mut emu);
    assert_eq!(ret(&emu), b'Q' as u64);
    set_args(&mut emu, &[b'Q' as u64]);
    gateway("_tolower", &mut emu);
    assert_eq!(ret(&emu), b'q' as u64);
}

#[test]
fn strnlen_strndup_and_abs() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "abcdef");
    set_args(&mut emu, &[s, 3]);
    gateway("_strnlen", &mut emu);
    assert_eq!(ret(&emu), 3);

    set_args(&mut emu, &[s, 4]);
    gateway("_strndup", &mut emu);
    assert_eq!(emu.maps.read_string(ret(&emu)), "abcd");

    set_args(&mut emu, &[(-5i32) as u32 as u64]);
    gateway("_abs", &mut emu);
    assert_eq!(ret(&emu), 5);
    set_args(&mut emu, &[(-7i64) as u64]);
    gateway("_labs", &mut emu);
    assert_eq!(ret(&emu), 7);
}

#[test]
fn host_identity_is_faked() {
    let (mut emu, d) = aarch64_env();
    set_args(&mut emu, &[d, 64]);
    gateway("_getcwd", &mut emu);
    assert_eq!(ret(&emu), d);
    assert_eq!(emu.maps.read_string(d), "/");

    set_args(&mut emu, &[d, 64]);
    gateway("_gethostname", &mut emu);
    assert_eq!(emu.maps.read_string(d), "localhost");

    let u = d + 0x1000;
    set_args(&mut emu, &[u]);
    gateway("_uname", &mut emu);
    assert_eq!(emu.maps.read_string(u), "Darwin");
    assert_eq!(emu.maps.read_string(u + 4 * 256), "arm64");
}

#[test]
fn time_queries_fill_structs() {
    let (mut emu, d) = aarch64_env();
    set_args(&mut emu, &[d, 0]);
    gateway("_gettimeofday", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert!(emu.maps.read_qword(d).unwrap() > 1_600_000_000);
    assert!(emu.maps.read_dword(d + 8).unwrap() < 1_000_000);

    set_args(&mut emu, &[0, d + 0x100]);
    gateway("_clock_gettime", &mut emu);
    assert!(emu.maps.read_qword(d + 0x100).unwrap() > 1_600_000_000);
    assert!(emu.maps.read_qword(d + 0x108).unwrap() < 1_000_000_000);
}

#[test]
fn rand_is_deterministic_per_seed() {
    let (mut emu, _) = aarch64_env();
    let sequence = |emu: &mut crate::emu::Emu| -> Vec<u64> {
        set_args(emu, &[7]);
        gateway("_srand", emu);
        (0..3)
            .map(|_| {
                gateway("_rand", emu);
                ret(emu)
            })
            .collect()
    };
    let first = sequence(&mut emu);
    let second = sequence(&mut emu);
    assert_eq!(first, second);
    assert!(first.iter().all(|&v| v <= 0x7fff_ffff));

    for _ in 0..20 {
        set_args(&mut emu, &[10]);
        gateway("_arc4random_uniform", &mut emu);
        assert!(ret(&emu) < 10);
    }
}

#[test]
fn filesystem_mutations_never_touch_the_host() {
    let (mut emu, d) = aarch64_env();
    let path = std::env::temp_dir().join("mwemu_unlink_guard.txt");
    std::fs::write(&path, b"keep me").unwrap();
    let p = put_str(&mut emu, d, path.to_str().unwrap());

    set_args(&mut emu, &[p]);
    gateway("_unlink", &mut emu);
    assert_eq!(ret(&emu), 0, "emulated program sees success");
    assert!(path.exists(), "host file must survive");

    set_args(&mut emu, &[p, 0]);
    gateway("_access", &mut emu);
    assert_eq!(ret(&emu), 0);
    std::fs::remove_file(&path).unwrap();

    let missing = put_str(&mut emu, d + 0x400, "/nonexistent/mwemu/path");
    set_args(&mut emu, &[missing, 0]);
    gateway("_access", &mut emu);
    assert_eq!(ret(&emu) as i64, -1);

    set_args(&mut emu, &[p, d + 0x500]);
    gateway("_fopen", &mut emu);
    assert_eq!(ret(&emu), 0);
}

#[test]
fn pthread_once_runs_init_exactly_once() {
    let (mut emu, d) = aarch64_env();
    // init: ldr x9,[x10] ; add x9,x9,#1 ; str x9,[x10] ; ret
    let code: [u32; 4] = [0xf9400149, 0x91000529, 0xf9000149, 0xd65f03c0];
    let bytes: Vec<u8> = code.iter().flat_map(|w| w.to_le_bytes()).collect();
    emu.load_code_bytes(&bytes);
    let init = emu.cfg.code_base_addr;
    let counter = d + 0x800;
    let once = d + 0x900;

    for _ in 0..3 {
        emu.regs_aarch64_mut().x[10] = counter;
        set_args(&mut emu, &[once, init]);
        gateway("_pthread_once", &mut emu);
        assert_eq!(ret(&emu), 0);
    }
    assert_eq!(emu.maps.read_qword(counter), Some(1));

    set_args(&mut emu, &[d]);
    gateway("_pthread_mutex_lock", &mut emu);
    assert_eq!(ret(&emu), 0);
}

#[test]
fn unknown_symbol_returns_zero_without_panicking() {
    let (mut emu, _) = aarch64_env();
    emu.regs_aarch64_mut().x[0] = 0xdead;
    gateway("_definitely_not_a_libsystem_symbol", &mut emu);
    assert_eq!(ret(&emu), 0);
}
