//! Additional libSystem (libc/pthread) stubs for macOS binaries. Nothing here
//! modifies the host: mutating filesystem calls only pretend to succeed.
use crate::api::abi::ApiAbi;
use crate::api::strconv::{parse_c_float, parse_c_integer};
use crate::emu::Emu;
use crate::maps::mem64::Permission;

const RAND_STATE_MAP: &str = "macos_rand_state";
const FAKE_CWD: &str = "/";
const FAKE_HOSTNAME: &str = "localhost";
const UTSNAME_FIELD: u64 = 256;

/// Dispatch `symbol` if it is implemented here. Returns false otherwise.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let name = symbol.strip_prefix('_').unwrap_or(symbol);
    match name {
        "strtol" | "strtoll" | "strtoq" | "strtoul" | "strtoull" | "strtouq" => api_strtol(emu),
        "atoi" | "atol" | "atoll" => api_atoi(emu),
        "strtod" | "atof" => api_strtod(emu, name == "strtod"),
        "strnlen" => api_strnlen(emu),
        "strcasecmp" => api_strcasecmp(emu, usize::MAX),
        "strncasecmp" => {
            let n = ApiAbi::from_emu(emu).arg(emu, 2) as usize;
            api_strcasecmp(emu, n)
        }
        "strndup" => api_strndup(emu),
        "toupper" => api_change_case(emu, true),
        "tolower" => api_change_case(emu, false),
        "abs" => api_abs(emu, true),
        "labs" | "llabs" => api_abs(emu, false),
        "getcwd" => api_getcwd(emu),
        "access" => api_access(emu),
        "unlink" | "rmdir" | "mkdir" | "chmod" | "chdir" | "rename" => api_pretend_ok(emu, name),
        "sleep" | "usleep" | "nanosleep" => api_sleep(emu, name),
        "gettimeofday" => api_gettimeofday(emu),
        "clock_gettime" => api_clock_gettime(emu),
        "getppid" => api_getppid(emu),
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
        "pthread_mutex_init"
        | "pthread_mutex_lock"
        | "pthread_mutex_trylock"
        | "pthread_mutex_unlock"
        | "pthread_mutex_destroy" => api_return(emu, name, 0),
        "pthread_once" => api_pthread_once(emu),
        "pthread_self" => api_return(emu, name, 0x1000),
        _ => return false,
    }
    true
}

fn trace(emu: &Emu, call: &str) {
    log::info!(
        "{}** {} macOS API {} {}",
        emu.colors.light_red,
        emu.pos,
        call,
        emu.colors.nc
    );
}

fn api_return(emu: &mut Emu, name: &str, value: u64) {
    trace(emu, &format!("{}() -> 0x{:x}", name, value));
    ApiAbi::from_emu(emu).set_ret(emu, value);
}

fn api_pretend_ok(emu: &mut Emu, name: &str) {
    let path = emu.maps.read_string(ApiAbi::from_emu(emu).arg(emu, 0));
    trace(
        emu,
        &format!("{}(\"{}\") -> 0 (host untouched)", name, path),
    );
    ApiAbi::from_emu(emu).set_ret(emu, 0);
}

fn write_cstring(emu: &mut Emu, addr: u64, s: &[u8]) {
    emu.maps.write_bytes(addr, s);
    emu.maps.write_byte(addr + s.len() as u64, 0);
}

fn api_strtol(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s_addr = abi.arg(emu, 0);
    let endptr = abi.arg(emu, 1);
    let base = abi.arg(emu, 2) as u32;
    let s = emu.maps.read_string(s_addr);
    let (val, used) = parse_c_integer(s.as_bytes(), base);
    if endptr != 0 {
        emu.maps.write_qword(endptr, s_addr + used as u64);
    }
    trace(emu, &format!("strtol(\"{}\", base={}) -> {}", s, base, val));
    abi.set_ret(emu, val as u64);
}

fn api_atoi(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s = emu.maps.read_string(abi.arg(emu, 0));
    let (val, _) = parse_c_integer(s.as_bytes(), 10);
    trace(emu, &format!("atoi(\"{}\") -> {}", s, val));
    abi.set_ret(emu, val as u64);
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
    let abi = ApiAbi::from_emu(emu);
    let s_addr = abi.arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    let (val, used) = parse_c_float(s.as_bytes());
    let endptr = if has_endptr { abi.arg(emu, 1) } else { 0 };
    if endptr != 0 {
        emu.maps.write_qword(endptr, s_addr + used as u64);
    }
    trace(emu, &format!("strtod(\"{}\") -> {}", s, val));
    set_fp_ret(emu, val);
}

fn api_strnlen(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s = emu.maps.read_string(abi.arg(emu, 0));
    let max = abi.arg(emu, 1) as usize;
    abi.set_ret(emu, s.len().min(max) as u64);
}

fn api_strcasecmp(emu: &mut Emu, limit: usize) {
    let abi = ApiAbi::from_emu(emu);
    let a = emu.maps.read_string(abi.arg(emu, 0)).to_ascii_lowercase();
    let b = emu.maps.read_string(abi.arg(emu, 1)).to_ascii_lowercase();
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut r: i64 = 0;
    for i in 0..limit {
        let (ca, cb) = (
            a.get(i).copied().unwrap_or(0),
            b.get(i).copied().unwrap_or(0),
        );
        if ca != cb || ca == 0 {
            r = ca as i64 - cb as i64;
            break;
        }
    }
    abi.set_ret(emu, r as u64);
}

fn api_strndup(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s = emu.maps.read_string(abi.arg(emu, 0));
    let n = (abi.arg(emu, 1) as usize).min(s.len());
    let addr =
        super::libsystem::allocate_memory(emu, n as u64 + 1).expect("macOS strndup: out of memory");
    write_cstring(emu, addr, &s.as_bytes()[..n]);
    trace(emu, &format!("strndup(\"{}\", {}) -> 0x{:x}", s, n, addr));
    abi.set_ret(emu, addr);
}

fn api_change_case(emu: &mut Emu, upper: bool) {
    let abi = ApiAbi::from_emu(emu);
    let c = abi.arg(emu, 0) as u32;
    let r = match u8::try_from(c) {
        Ok(b) if upper => b.to_ascii_uppercase() as u64,
        Ok(b) => b.to_ascii_lowercase() as u64,
        Err(_) => c as u64,
    };
    abi.set_ret(emu, r);
}

fn api_abs(emu: &mut Emu, is_int: bool) {
    let abi = ApiAbi::from_emu(emu);
    let v = abi.arg(emu, 0);
    let r = if is_int {
        (v as i32).wrapping_abs() as u32 as u64
    } else {
        (v as i64).wrapping_abs() as u64
    };
    abi.set_ret(emu, r);
}

fn api_getcwd(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let buf = abi.arg(emu, 0);
    let size = abi.arg(emu, 1) as usize;
    trace(
        emu,
        &format!("getcwd(0x{:x}, {}) -> \"{}\"", buf, size, FAKE_CWD),
    );
    if buf == 0 || size <= FAKE_CWD.len() {
        abi.set_ret(emu, 0);
        return;
    }
    write_cstring(emu, buf, FAKE_CWD.as_bytes());
    abi.set_ret(emu, buf);
}

fn api_access(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let path = emu.maps.read_string(abi.arg(emu, 0));
    let exists = std::path::Path::new(&path).exists();
    trace(emu, &format!("access(\"{}\") -> {}", path, exists));
    abi.set_ret(emu, if exists { 0 } else { -1i64 as u64 });
}

fn api_sleep(emu: &mut Emu, name: &str) {
    let amount = ApiAbi::from_emu(emu).arg(emu, 0);
    trace(emu, &format!("{}({}) -> 0 (skipped)", name, amount));
    ApiAbi::from_emu(emu).set_ret(emu, 0);
}

fn host_now() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
}

/// struct timeval on Darwin: { long tv_sec; int tv_usec; }
fn api_gettimeofday(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let tv = abi.arg(emu, 0);
    let now = host_now();
    if tv != 0 {
        emu.maps.write_qword(tv, now.as_secs());
        emu.maps.write_dword(tv + 8, now.subsec_micros());
    }
    trace(emu, &format!("gettimeofday() -> {}", now.as_secs()));
    abi.set_ret(emu, 0);
}

/// struct timespec: { long tv_sec; long tv_nsec; }
fn api_clock_gettime(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let ts = abi.arg(emu, 1);
    let now = host_now();
    if ts != 0 {
        emu.maps.write_qword(ts, now.as_secs());
        emu.maps.write_qword(ts + 8, now.subsec_nanos() as u64);
    }
    trace(emu, &format!("clock_gettime() -> {}", now.as_secs()));
    abi.set_ret(emu, 0);
}

fn api_getppid(emu: &mut Emu) {
    trace(emu, "getppid() -> 1");
    ApiAbi::from_emu(emu).set_ret(emu, 1);
}

/// PRNG state lives in a tiny emulated map so it survives between calls
/// and is serialized with the rest of the emulator memory.
fn rand_state_addr(emu: &mut Emu) -> u64 {
    if let Some(m) = emu.maps.get_map_by_name(RAND_STATE_MAP) {
        return m.get_base();
    }
    let base = emu.maps.alloc(8).expect("macOS rand: out of memory");
    emu.maps
        .create_map(RAND_STATE_MAP, base, 8, Permission::READ_WRITE)
        .expect("macOS rand: cannot create map");
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
    let seed = ApiAbi::from_emu(emu).arg(emu, 0);
    let addr = rand_state_addr(emu);
    emu.maps.write_qword(addr, seed);
    trace(emu, &format!("srand({})", seed));
}

fn api_rand(emu: &mut Emu, max: u64) {
    let r = next_rand(emu) & max;
    ApiAbi::from_emu(emu).set_ret(emu, r);
}

fn api_arc4random_uniform(emu: &mut Emu) {
    let bound = ApiAbi::from_emu(emu).arg(emu, 0) as u32 as u64;
    let r = if bound == 0 {
        0
    } else {
        next_rand(emu) % bound
    };
    ApiAbi::from_emu(emu).set_ret(emu, r);
}

fn api_arc4random_buf(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let buf = abi.arg(emu, 0);
    let n = abi.arg(emu, 1);
    for i in 0..n {
        let b = next_rand(emu) as u8;
        emu.maps.write_byte(buf + i, b);
    }
}

fn api_fopen(emu: &mut Emu, name: &str) {
    let path = emu.maps.read_string(ApiAbi::from_emu(emu).arg(emu, 0));
    trace(emu, &format!("{}(\"{}\") -> NULL", name, path));
    ApiAbi::from_emu(emu).set_ret(emu, 0);
}

/// dup(fd) hands back fd itself; dup2(fd, target) returns target.
fn api_dup(emu: &mut Emu, is_dup2: bool) {
    let abi = ApiAbi::from_emu(emu);
    let fd = abi.arg(emu, 0);
    let r = if is_dup2 { abi.arg(emu, 1) } else { fd };
    trace(emu, &format!("dup({}) -> {}", fd, r));
    abi.set_ret(emu, r);
}

fn api_perror(emu: &mut Emu) {
    let s = emu.maps.read_string(ApiAbi::from_emu(emu).arg(emu, 0));
    let line = if s.is_empty() {
        "Unknown error\n".to_string()
    } else {
        format!("{}: Unknown error\n", s)
    };
    trace(emu, &format!("perror(\"{}\")", s));
    emu.emulated_stdout.extend_from_slice(line.as_bytes());
}

fn api_gethostname(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let buf = abi.arg(emu, 0);
    let len = abi.arg(emu, 1) as usize;
    if buf == 0 || len <= FAKE_HOSTNAME.len() {
        abi.set_ret(emu, -1i64 as u64);
        return;
    }
    write_cstring(emu, buf, FAKE_HOSTNAME.as_bytes());
    trace(emu, &format!("gethostname() -> \"{}\"", FAKE_HOSTNAME));
    abi.set_ret(emu, 0);
}

/// struct utsname on Darwin: five char[256] fields.
fn api_uname(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let buf = abi.arg(emu, 0);
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
    abi.set_ret(emu, 0);
}

/// pthread_once(once, init): run `init` the first time only. Darwin's
/// pthread_once_t is { long sig; char opaque[8]; }; the opaque word marks done.
fn api_pthread_once(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let once = abi.arg(emu, 0);
    let init = abi.arg(emu, 1);
    let done_flag = once + 8;
    if emu.maps.read_qword(done_flag).unwrap_or(0) != 0 {
        abi.set_ret(emu, 0);
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
    abi.set_ret(emu, 0);
}
