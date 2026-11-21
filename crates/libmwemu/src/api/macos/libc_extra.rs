//! Additional libSystem (libc/pthread) stubs for macOS binaries. Nothing here
//! modifies the host: mutating filesystem calls only pretend to succeed.
use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::macos::libsystem::allocate_memory;
use crate::api::strconv::{integer, parse_c_float};
use crate::api::unix::{api_try, memory};
use crate::emu::Emu;
use crate::maps::mem64::Permission;

const RAND_STATE_MAP: &str = "macos_rand_state";
const FAKE_CWD: &str = "/";
const FAKE_HOSTNAME: &str = "localhost";
const UTSNAME_FIELD: u64 = 256;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    dispatch_api!(name, {
        "atoi" | "atol" | "atoll" | "strtol" | "strtoll" | "strtoimax" | "strtoul" | "strtoull" | "strtoumax" | "strtoq" | "strtouq" => integer(emu, name),
        "strtod" | "atof" => api_strtod(emu, name == "strtod"),
        "strndup" => api_strndup(emu),
        "toupper" => change_case(emu, true),
        "tolower" => change_case(emu, false),
        "abs" => api_abs(emu, true),
        "labs" | "llabs" => api_abs(emu, false),
        "getcwd" => api_getcwd(emu),
        "access" => api_access(emu),
        "unlink" | "rmdir" | "mkdir" | "chmod" | "chdir" | "rename" => api_pretend_ok(emu, name),
        "sleep" | "usleep" | "nanosleep" => api_sleep(emu, name),
        "gettimeofday" => api_gettimeofday(emu),
        "clock_gettime" => api_clock_gettime(emu),
        "srand" | "srandom" => api_srand(emu),
        "rand" | "random" => api_rand(emu, 0x7fff_ffff),
        "arc4random" => api_rand(emu, u32::MAX as u64),
        "arc4random_uniform" => api_arc4random_uniform(emu),
        "arc4random_buf" => api_arc4random_buf(emu),
        "fopen" | "fdopen" | "popen" => api_fopen(emu, name),
        "fclose" | "pclose" => api_return(emu, name, 0),
        "fgets" => api_return(emu, name, 0),
        "feof" => api_return(emu, name, 1),
        "fileno" => api_return(emu, name, 1),
        "dup" => api_dup(emu, false),
        "dup2" => api_dup(emu, true),
        "pipe" => api_return(emu, name, -1i64 as u64),
        "perror" => api_perror(emu),
        "gethostname" => api_gethostname(emu),
        "uname" => api_uname(emu),
        "sigaction" | "sigprocmask" | "sigemptyset" | "sigaddset" => api_return(emu, name, 0),
        "pthread_once" => api_pthread_once(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

pub(super) fn trace(emu: &Emu, call: &str) {
    log::info!(
        "{}** {} macOS API {} {}",
        emu.colors.light_red,
        emu.pos,
        call,
        emu.colors.nc
    );
}

fn api_return(emu: &mut Emu, name: &str, value: u64) {
    let call = UnixApi;
    trace(emu, &format!("{}() -> 0x{:x}", name, value));
    call.set_return(emu, value);
}

fn api_pretend_ok(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    let path = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    trace(
        emu,
        &format!("{}(\"{}\") -> 0 (host untouched)", name, path),
    );
    call.set_return(emu, 0);
}

fn write_cstring(emu: &mut Emu, addr: u64, s: &[u8]) {
    emu.maps.write_bytes(addr, s);
    emu.maps.write_byte(addr + s.len() as u64, 0);
}

/// Floating-point results are returned in d0 (AArch64) or xmm0 (x86_64).
fn set_fp_ret(emu: &mut Emu, val: f64) {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().v[0] = val.to_bits() as u128;
    } else {
        emu.regs_mut().xmm0 = val.to_bits() as u128;
    }
}

fn api_strtod(emu: &mut Emu, has_endptr: bool) {
    let call = UnixApi;
    let s_addr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(s_addr);
    let (val, used) = parse_c_float(s.as_bytes());
    let endptr = if has_endptr {
        api_try!(emu, call.arg(emu, 1))
    } else {
        0
    };
    if endptr != 0 {
        emu.maps.write_qword(endptr, s_addr + used as u64);
    }
    trace(emu, &format!("strtod(\"{}\") -> {}", s, val));
    set_fp_ret(emu, val);
}

fn api_strndup(emu: &mut Emu) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let limit = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, memory::length(emu, source, limit));
    let destination = allocate_memory(emu, count + 1).unwrap_or(0);
    trace(
        emu,
        &format!("strndup(0x{source:x}, {limit}) -> 0x{destination:x}"),
    );
    if destination != 0 {
        api_try!(
            emu,
            memory::terminated_copy(emu, destination, source, count)
        );
    }
    call.set_return(emu, destination);
}

pub(super) fn change_case(emu: &mut Emu, upper: bool) {
    let call = UnixApi;
    let c = api_try!(emu, call.arg_u32(emu, 0));
    let r = match u8::try_from(c) {
        Ok(b) if upper => b.to_ascii_uppercase() as u64,
        Ok(b) => b.to_ascii_lowercase() as u64,
        Err(_) => c as u64,
    };
    call.set_return(emu, r);
}

fn api_abs(emu: &mut Emu, is_int: bool) {
    let call = UnixApi;
    let v = api_try!(emu, call.arg(emu, 0));
    let r = if is_int {
        (v as i32).wrapping_abs() as u32 as u64
    } else {
        (v as i64).wrapping_abs() as u64
    };
    call.set_return(emu, r);
}

fn api_getcwd(emu: &mut Emu) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1)) as usize;
    trace(
        emu,
        &format!("getcwd(0x{:x}, {}) -> \"{}\"", buf, size, FAKE_CWD),
    );
    if buf == 0 || size <= FAKE_CWD.len() {
        call.set_return(emu, 0);
        return;
    }
    write_cstring(emu, buf, FAKE_CWD.as_bytes());
    call.set_return(emu, buf);
}

fn api_access(emu: &mut Emu) {
    let call = UnixApi;
    let path = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let exists = std::path::Path::new(&path).exists();
    trace(emu, &format!("access(\"{}\") -> {}", path, exists));
    call.set_return(emu, if exists { 0 } else { -1i64 as u64 });
}

fn api_sleep(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    let amount = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("{}({}) -> 0 (skipped)", name, amount));
    call.set_return(emu, 0);
}

fn host_now() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
}

/// struct timeval on Darwin: { long tv_sec; int tv_usec; }
fn api_gettimeofday(emu: &mut Emu) {
    let call = UnixApi;
    let tv = api_try!(emu, call.arg(emu, 0));
    let now = host_now();
    if tv != 0 {
        emu.maps.write_qword(tv, now.as_secs());
        emu.maps.write_dword(tv + 8, now.subsec_micros());
    }
    trace(emu, &format!("gettimeofday() -> {}", now.as_secs()));
    call.set_return(emu, 0);
}

/// struct timespec: { long tv_sec; long tv_nsec; }
fn api_clock_gettime(emu: &mut Emu) {
    let call = UnixApi;
    let ts = api_try!(emu, call.arg(emu, 1));
    let now = host_now();
    if ts != 0 {
        emu.maps.write_qword(ts, now.as_secs());
        emu.maps.write_qword(ts + 8, now.subsec_nanos() as u64);
    }
    trace(emu, &format!("clock_gettime() -> {}", now.as_secs()));
    call.set_return(emu, 0);
}

/// PRNG state lives in a tiny emulated map so it survives between calls
/// and is serialized with the rest of the emulator memory.
fn rand_state_addr(emu: &mut Emu) -> u64 {
    if let Some(m) = emu.maps.get_map_by_name(RAND_STATE_MAP) {
        return m.get_base();
    }
    let base = match emu.maps.alloc(8) {
        Some(a) => a,
        None => {
            log::error!("macOS rand: out of memory");
            return 0;
        }
    };
    if emu
        .maps
        .create_map(RAND_STATE_MAP, base, 8, Permission::READ_WRITE)
        .is_err()
    {
        log::error!("macOS rand: cannot create map");
        return 0;
    }
    emu.maps.write_qword(base, 1);
    base
}

/// Deterministic 64-bit LCG (Knuth MMIX constants), so runs are reproducible.
fn next_rand(emu: &mut Emu) -> u64 {
    let addr = rand_state_addr(emu);
    let state = emu.maps.read_qword(addr).unwrap_or(1);
    let next = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    emu.maps.write_qword(addr, next);
    next >> 32
}

fn api_srand(emu: &mut Emu) {
    let call = UnixApi;
    let seed = api_try!(emu, call.arg(emu, 0));
    let addr = rand_state_addr(emu);
    emu.maps.write_qword(addr, seed);
    trace(emu, &format!("srand({})", seed));
}

fn api_rand(emu: &mut Emu, max: u64) {
    let call = UnixApi;
    let r = next_rand(emu) & max;
    call.set_return(emu, r);
}

fn api_arc4random_uniform(emu: &mut Emu) {
    let call = UnixApi;
    let bound = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let r = if bound == 0 {
        0
    } else {
        next_rand(emu) % bound
    };
    call.set_return(emu, r);
}

fn api_arc4random_buf(emu: &mut Emu) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let n = api_try!(emu, call.arg(emu, 1));
    for i in 0..n {
        let b = next_rand(emu) as u8;
        emu.maps.write_byte(buf + i, b);
    }
}

fn api_fopen(emu: &mut Emu, name: &str) {
    let call = UnixApi;
    let path = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    trace(emu, &format!("{}(\"{}\") -> NULL", name, path));
    call.set_return(emu, 0);
}

/// dup(fd) hands back fd itself; dup2(fd, target) returns target.
fn api_dup(emu: &mut Emu, is_dup2: bool) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg(emu, 0));
    let r = if is_dup2 {
        api_try!(emu, call.arg(emu, 1))
    } else {
        fd
    };
    trace(emu, &format!("dup({}) -> {}", fd, r));
    call.set_return(emu, r);
}

fn api_perror(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let line = if s.is_empty() {
        "Unknown error\n".to_string()
    } else {
        format!("{}: Unknown error\n", s)
    };
    trace(emu, &format!("perror(\"{}\")", s));
    emu.emulated_stdout.extend_from_slice(line.as_bytes());
}

fn api_gethostname(emu: &mut Emu) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1)) as usize;
    if buf == 0 || len <= FAKE_HOSTNAME.len() {
        call.set_return(emu, -1i64 as u64);
        return;
    }
    write_cstring(emu, buf, FAKE_HOSTNAME.as_bytes());
    trace(emu, &format!("gethostname() -> \"{}\"", FAKE_HOSTNAME));
    call.set_return(emu, 0);
}

/// struct utsname on Darwin: five char[256] fields.
fn api_uname(emu: &mut Emu) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let machine = if emu.cfg.arch.is_aarch64() {
        "arm64"
    } else {
        "x86_64"
    };
    let fields = [
        "Darwin",
        FAKE_HOSTNAME,
        "24.0.0",
        "Darwin Kernel Version 24.0.0",
        machine,
    ];
    for (i, f) in fields.iter().enumerate() {
        let addr = buf + i as u64 * UTSNAME_FIELD;
        emu.maps.memset(addr, 0, UTSNAME_FIELD as usize);
        write_cstring(emu, addr, f.as_bytes());
    }
    trace(emu, &format!("uname() -> Darwin {}", machine));
    call.set_return(emu, 0);
}

/// pthread_once(once, init): run `init` the first time only. Darwin's
/// pthread_once_t is { long sig; char opaque[8]; }; the opaque word marks done.
fn api_pthread_once(emu: &mut Emu) {
    let call = UnixApi;
    let once = api_try!(emu, call.arg(emu, 0));
    let init = api_try!(emu, call.arg(emu, 1));
    let done_flag = once + 8;
    if emu.maps.read_qword(done_flag).unwrap_or(0) != 0 {
        call.set_return(emu, 0);
        return;
    }
    emu.maps.write_qword(done_flag, 1);
    trace(
        emu,
        &format!("pthread_once(0x{:x}) -> calling 0x{:x}", once, init),
    );
    let result = if emu.cfg.arch.is_aarch64() {
        emu.aarch64_call64(init, &[])
    } else {
        emu.linux_call64(init, &[])
    };
    if let Err(e) = result {
        log::warn!("pthread_once: init routine failed: {}", e);
    }
    call.set_return(emu, 0);
}
