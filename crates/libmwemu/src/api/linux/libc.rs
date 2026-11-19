use crate::api::abi::unix::{ArgumentError, UnixApi, VariadicConvention::SystemV};
use crate::api::printf;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;

const ERRNO_MAP: &str = "linux_errno";
const FAKE_PID: u64 = 1234;
const FAKE_UID: u64 = 1000;

pub fn gateway(symbol: &str, emu: &mut Emu) {
    let call = UnixApi;
    match symbol {
        "__libc_start_main" => api_libc_start_main(emu),
        "__cxa_finalize" => api_cxa_finalize(emu),
        "__cxa_atexit" | "atexit" => api_cxa_atexit(emu),
        "__gmon_start__" => api_gmon_start(emu),
        "__stack_chk_fail" => api_stack_chk_fail(emu),
        "__errno_location" => api_errno_location(emu),
        "printf" => api_printf(emu),
        "fprintf" => api_fprintf(emu),
        "sprintf" => api_sprintf(emu),
        "snprintf" => api_snprintf(emu),
        "puts" => api_puts(emu),
        "putchar" => api_putchar(emu),
        "fputs" => api_fputs(emu),
        "fputc" | "putc" => api_fputc(emu),
        "fwrite" => api_fwrite(emu),
        "fflush" => api_fflush(emu),
        "exit" | "_exit" => api_exit(emu),
        "abort" => api_abort(emu),
        "malloc" => api_malloc(emu),
        "calloc" => api_calloc(emu),
        "realloc" => api_realloc(emu),
        "free" => api_free(emu),
        "write" => api_write(emu),
        "read" => api_read(emu),
        "open" | "open64" => api_open(emu),
        "close" => api_close(emu),
        "strlen" => api_strlen(emu),
        "strcmp" => api_strcmp(emu),
        "strncmp" => api_strncmp(emu),
        "strcpy" => api_strcpy(emu),
        "strncpy" => api_strncpy(emu),
        "strcat" => api_strcat(emu),
        "strchr" => api_strchr(emu),
        "strrchr" => api_strrchr(emu),
        "strstr" => api_strstr(emu),
        "strdup" => api_strdup(emu),
        "memcpy" => api_memcpy(emu),
        "memmove" => api_memmove(emu),
        "memset" => api_memset(emu),
        "memcmp" => api_memcmp(emu),
        "memchr" => api_memchr(emu),
        "bzero" => api_bzero(emu),
        "atoi" | "atol" | "atoll" => api_atoi(emu),
        "strtol" | "strtoll" => api_strtol(emu),
        "strtoul" | "strtoull" => api_strtoul(emu),
        "strerror" => api_strerror(emu),
        "getenv" | "secure_getenv" => api_getenv(emu),
        "setlocale" => api_setlocale(emu),
        "getpid" | "getppid" => api_getpid(emu),
        "getuid" | "geteuid" | "getgid" | "getegid" => api_getuid(emu),
        "isatty" => api_isatty(emu),
        "ioctl" => api_ioctl(emu),
        "signal" => api_signal(emu),
        "time" => api_time(emu),
        "getopt" | "getopt_long" => api_getopt(emu),
        "mmap" | "mmap64" => api_mmap(emu),
        "munmap" => api_munmap(emu),
        _ => {
            log::warn!("linuxapi libc: unimplemented API {} -- returning 0", symbol);
            call.set_return(emu, 0);
        }
    }
}

fn trace(emu: &Emu, call: &str) {
    log::info!(
        "{}** {} Linux API {} {}",
        emu.colors.light_red,
        emu.pos,
        call,
        emu.colors.nc
    );
}

fn format_printf(emu: &Emu, fmt_addr: u64, first: usize) -> Result<String, ArgumentError> {
    let call = UnixApi;
    let fmt = emu.maps.read_string(fmt_addr);
    printf::format(emu, &fmt, &|e, n| call.integer_vararg(e, first, n, SystemV))
}

fn write_cstring(emu: &mut Emu, addr: u64, s: &[u8]) {
    emu.maps.write_bytes(addr, s);
    emu.maps.write_byte(addr + s.len() as u64, 0);
}

fn alloc_cstring(emu: &mut Emu, s: &str) -> u64 {
    let addr = linux_allocate(emu, s.len() as u64 + 1);
    write_cstring(emu, addr, s.as_bytes());
    addr
}

fn api_libc_start_main(emu: &mut Emu) {
    let call = UnixApi;
    let main_fn = api_try!(emu, call.arg(emu, 0));
    let argc = api_try!(emu, call.arg(emu, 1));
    let argv = api_try!(emu, call.arg(emu, 2));
    let envp = argv.wrapping_add((argc + 1) * 8);

    trace(
        emu,
        &format!(
            "__libc_start_main(main=0x{:x}, argc={}, argv=0x{:x})",
            main_fn, argc, argv
        ),
    );

    let call_result = if emu.cfg.arch.is_aarch64() {
        emu.aarch64_call64(main_fn, &[argc, argv, envp])
    } else {
        emu.linux_call64(main_fn, &[argc, argv, envp])
    };

    match call_result {
        Ok(status) => {
            // Model glibc/musl startup minimally: after main returns, set the
            // exit status in the first argument register and terminate.
            if emu.cfg.arch.is_aarch64() {
                emu.regs_aarch64_mut().x[0] = status;
            } else {
                emu.regs_mut().rdi = status;
            }
            api_exit(emu);
        }
        Err(err) => {
            log::warn!(
                "linuxapi libc: __libc_start_main failed to run main: {}",
                err
            );
            emu.stop();
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let call = UnixApi;
    let out = api_try!(emu, format_printf(emu, api_try!(emu, call.arg(emu, 0)), 1));
    trace(emu, &format!("printf -> \"{}\"", out));
    emu.emulated_stdout.extend_from_slice(out.as_bytes());
    call.set_return(emu, out.len() as u64);
}

fn api_fprintf(emu: &mut Emu) {
    let call = UnixApi;
    let out = api_try!(emu, format_printf(emu, api_try!(emu, call.arg(emu, 1)), 2));
    trace(emu, &format!("fprintf -> \"{}\"", out));
    emu.emulated_stdout.extend_from_slice(out.as_bytes());
    call.set_return(emu, out.len() as u64);
}

fn api_sprintf(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, format_printf(emu, api_try!(emu, call.arg(emu, 1)), 2));
    trace(emu, &format!("sprintf(0x{:x}) -> \"{}\"", dst, out));
    write_cstring(emu, dst, out.as_bytes());
    call.set_return(emu, out.len() as u64);
}

fn api_snprintf(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1)) as usize;
    let out = api_try!(emu, format_printf(emu, api_try!(emu, call.arg(emu, 2)), 3));
    trace(
        emu,
        &format!("snprintf(0x{:x}, {}) -> \"{}\"", dst, size, out),
    );
    if size > 0 {
        let n = out.len().min(size - 1);
        write_cstring(emu, dst, &out.as_bytes()[..n]);
    }
    call.set_return(emu, out.len() as u64);
}

fn api_puts(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    trace(emu, &format!("puts(\"{}\")", s));
    emu.emulated_stdout.extend_from_slice(s.as_bytes());
    emu.emulated_stdout.push(b'\n');
    call.set_return(emu, s.len() as u64 + 1);
}

fn api_putchar(emu: &mut Emu) {
    let call = UnixApi;
    let c = api_try!(emu, call.arg_u8(emu, 0));
    trace(emu, &format!("putchar('{}')", c as char));
    emu.emulated_stdout.push(c);
    call.set_return(emu, c as u64);
}

fn api_fputs(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    trace(emu, &format!("fputs(\"{}\")", s));
    emu.emulated_stdout.extend_from_slice(s.as_bytes());
    call.set_return(emu, s.len() as u64);
}

fn api_fputc(emu: &mut Emu) {
    let call = UnixApi;
    let c = api_try!(emu, call.arg_u8(emu, 0));
    trace(emu, &format!("fputc('{}')", c as char));
    emu.emulated_stdout.push(c);
    call.set_return(emu, c as u64);
}

fn api_fwrite(emu: &mut Emu) {
    let call = UnixApi;
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let nmemb = api_try!(emu, call.arg(emu, 2));
    let total = size.saturating_mul(nmemb) as usize;
    let data = emu.maps.read_bytes(buf, total).to_vec();
    trace(emu, &format!("fwrite(0x{:x}, {}, {})", buf, size, nmemb));
    emu.emulated_stdout.extend_from_slice(&data);
    call.set_return(emu, nmemb);
}

fn api_fflush(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, "fflush()");
    call.set_return(emu, 0);
}

fn flush_stdout(emu: &mut Emu) {
    if !emu.emulated_stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&emu.emulated_stdout));
        emu.emulated_stdout.clear();
    }
}

fn api_exit(emu: &mut Emu) {
    let call = UnixApi;
    let status = api_try!(emu, call.arg(emu, 0));
    flush_stdout(emu);
    trace(emu, &format!("exit({})", status));
    emu.stop();
}

fn api_abort(emu: &mut Emu) {
    flush_stdout(emu);
    trace(emu, "abort()");
    emu.stop();
}

fn api_stack_chk_fail(emu: &mut Emu) {
    flush_stdout(emu);
    trace(emu, "__stack_chk_fail() -- stack smashing detected");
    emu.stop();
}

fn api_cxa_finalize(emu: &mut Emu) {
    let call = UnixApi;
    let dso_handle = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("__cxa_finalize(0x{:x})", dso_handle));
    call.set_return(emu, 0);
}

fn api_cxa_atexit(emu: &mut Emu) {
    let call = UnixApi;
    let func = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("atexit(func=0x{:x})", func));
    call.set_return(emu, 0);
}

fn api_gmon_start(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, "__gmon_start__()");
    call.set_return(emu, 0);
}

fn api_errno_location(emu: &mut Emu) {
    let call = UnixApi;
    let addr = match emu.maps.get_map_by_name(ERRNO_MAP) {
        Some(m) => m.get_base(),
        None => {
            let base = match emu.maps.alloc(8) {
                Some(a) => a,
                None => {
                    log::error!("Linux errno: out of memory");
                    call.set_return(emu, 0);
                    return;
                }
            };
            if emu
                .maps
                .create_map(ERRNO_MAP, base, 8, Permission::READ_WRITE)
                .is_err()
            {
                log::error!("Linux errno: cannot create map");
                call.set_return(emu, 0);
                return;
            }
            base
        }
    };
    trace(emu, &format!("__errno_location() -> 0x{:x}", addr));
    call.set_return(emu, addr);
}

fn linux_allocate(emu: &mut Emu, size: u64) -> u64 {
    let alloc = match emu.maps.alloc(size) {
        Some(a) => a,
        None => {
            log::error!("Linux malloc: out of memory (size={})", size);
            return 0;
        }
    };
    if emu
        .maps
        .create_map(
            &format!("alloc_{:x}", alloc),
            alloc,
            size,
            Permission::READ_WRITE,
        )
        .is_err()
    {
        log::error!("Linux malloc: cannot create map at 0x{:x}", alloc);
        return 0;
    }
    alloc
}

fn linux_release(emu: &mut Emu, addr: u64) {
    if addr == 0 {
        return;
    }
    if let Some(mem) = emu.maps.get_mem_by_addr(addr)
        && mem.get_base() == addr
        && mem.get_name().starts_with("alloc_")
    {
        let name = mem.get_name().to_string();
        emu.maps.free(&name);
    }
}

fn api_malloc(emu: &mut Emu) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("malloc({})", size));
    if size > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
        } else {
            linux_allocate(emu, size)
        };
        log::info!("  -> 0x{:x}", base);
        call.set_return(emu, base);
    } else {
        call.set_return(emu, 0);
    }
}

fn api_calloc(emu: &mut Emu) {
    let call = UnixApi;
    let count = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let total = count.saturating_mul(size);
    trace(emu, &format!("calloc({}, {})", count, size));
    if total > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, total, "malloc", "calloc", true)
        } else {
            let b = linux_allocate(emu, total);
            emu.maps.memset(b, 0, total as usize);
            b
        };
        log::info!("  -> 0x{:x}", base);
        call.set_return(emu, base);
    } else {
        call.set_return(emu, 0);
    }
}

fn api_realloc(emu: &mut Emu) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    trace(emu, &format!("realloc(0x{:x}, {})", ptr, size));
    if size == 0 {
        if ptr != 0 && emu.cfg.memory_guard {
            emu.kernel_free(ptr, "realloc");
        }
        call.set_return(emu, 0);
        return;
    }
    let base = if emu.cfg.memory_guard {
        emu.kernel_alloc(Region::Slab, size, "malloc", "realloc", false)
    } else {
        linux_allocate(emu, size)
    };
    if ptr != 0 {
        for i in 0..size {
            match emu.maps.read_byte(ptr + i) {
                Some(b) => emu.maps.write_byte(base + i, b),
                None => break,
            };
        }
        if emu.cfg.memory_guard {
            emu.kernel_free(ptr, "realloc");
        } else {
            linux_release(emu, ptr);
        }
    }
    log::info!("  -> 0x{:x}", base);
    call.set_return(emu, base);
}

fn api_free(emu: &mut Emu) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("free(0x{:x})", ptr));
    if emu.cfg.memory_guard {
        emu.kernel_free(ptr, "free");
    } else {
        linux_release(emu, ptr);
    }
}

fn api_write(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let data = emu.maps.read_bytes(buf, count as usize).to_vec();
    trace(
        emu,
        &format!(
            "write(fd={}, \"{}\", {})",
            fd,
            String::from_utf8_lossy(&data),
            count
        ),
    );
    if fd == 1 || fd == 2 {
        emu.emulated_stdout.extend_from_slice(&data);
    }
    call.set_return(emu, count);
}

fn api_read(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg(emu, 2));
    trace(emu, &format!("read(fd={}, count={}) -> 0 (EOF)", fd, count));
    call.set_return(emu, 0);
}

fn api_open(emu: &mut Emu) {
    let call = UnixApi;
    let path = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let flags = api_try!(emu, call.arg(emu, 1));
    trace(emu, &format!("open(\"{}\", 0x{:x}) -> 3", path, flags));
    call.set_return(emu, 3);
}

fn api_close(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("close(fd={})", fd));
    call.set_return(emu, 0);
}

fn api_strlen(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    call.set_return(emu, s.len() as u64);
}

/// C-style comparison of at most `limit` bytes of two NUL-terminated strings.
fn compare_cstrings(a: &[u8], b: &[u8], limit: usize) -> i64 {
    for i in 0..limit {
        let ca = a.get(i).copied().unwrap_or(0);
        let cb = b.get(i).copied().unwrap_or(0);
        if ca != cb || ca == 0 {
            return ca as i64 - cb as i64;
        }
    }
    0
}

fn api_strcmp(emu: &mut Emu) {
    let call = UnixApi;
    let a = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let b = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let r = compare_cstrings(a.as_bytes(), b.as_bytes(), usize::MAX);
    call.set_return(emu, r as u64);
}

fn api_strncmp(emu: &mut Emu) {
    let call = UnixApi;
    let a = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let b = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let r = compare_cstrings(a.as_bytes(), b.as_bytes(), n);
    call.set_return(emu, r as u64);
}

fn api_strcpy(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    write_cstring(emu, dst, s.as_bytes());
    call.set_return(emu, dst);
}

fn api_strncpy(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let mut buf = s.into_bytes();
    buf.resize(n, 0);
    emu.maps.write_bytes(dst, &buf);
    call.set_return(emu, dst);
}

fn api_strcat(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let end = dst + emu.maps.read_string(dst).len() as u64;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    write_cstring(emu, end, s.as_bytes());
    call.set_return(emu, dst);
}

fn api_strchr(emu: &mut Emu) {
    let call = UnixApi;
    let s_addr = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let s = emu.maps.read_string(s_addr);
    let r = if c == 0 {
        s_addr + s.len() as u64
    } else {
        s.bytes()
            .position(|b| b == c)
            .map_or(0, |i| s_addr + i as u64)
    };
    call.set_return(emu, r);
}

fn api_strrchr(emu: &mut Emu) {
    let call = UnixApi;
    let s_addr = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let s = emu.maps.read_string(s_addr);
    let r = if c == 0 {
        s_addr + s.len() as u64
    } else {
        s.bytes()
            .rposition(|b| b == c)
            .map_or(0, |i| s_addr + i as u64)
    };
    call.set_return(emu, r);
}

fn api_strstr(emu: &mut Emu) {
    let call = UnixApi;
    let hay_addr = api_try!(emu, call.arg(emu, 0));
    let hay = emu.maps.read_string(hay_addr);
    let needle = emu.maps.read_string(api_try!(emu, call.arg(emu, 1)));
    let r = hay.find(&needle).map_or(0, |i| hay_addr + i as u64);
    call.set_return(emu, r);
}

fn api_strdup(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let r = alloc_cstring(emu, &s);
    trace(emu, &format!("strdup(\"{}\") -> 0x{:x}", s, r));
    call.set_return(emu, r);
}

fn api_memcpy(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    if !emu.maps.memcpy(dst, src, n) {
        log::warn!("linuxapi libc: memcpy from unmapped 0x{:x}", src);
    }
    call.set_return(emu, dst);
}

fn api_memmove(emu: &mut Emu) {
    // Maps::memcpy copies through a temporary buffer, so overlap is safe.
    api_memcpy(emu);
}

fn api_memset(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    emu.maps.memset(dst, c, n);
    call.set_return(emu, dst);
}

fn api_memcmp(emu: &mut Emu) {
    let call = UnixApi;
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let a = emu
        .maps
        .read_bytes(api_try!(emu, call.arg(emu, 0)), n)
        .to_vec();
    let b = emu.maps.read_bytes(api_try!(emu, call.arg(emu, 1)), n);
    let r = a
        .iter()
        .zip(b.iter())
        .find(|(x, y)| x != y)
        .map_or(0, |(x, y)| *x as i64 - *y as i64);
    call.set_return(emu, r as u64);
}

fn api_memchr(emu: &mut Emu) {
    let call = UnixApi;
    let addr = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let r = emu
        .maps
        .read_bytes(addr, n)
        .iter()
        .position(|&b| b == c)
        .map_or(0, |i| addr + i as u64);
    call.set_return(emu, r);
}

fn api_bzero(emu: &mut Emu) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let n = api_try!(emu, call.arg(emu, 1)) as usize;
    emu.maps.memset(dst, 0, n);
}

fn strtol_common(emu: &mut Emu) {
    let call = UnixApi;
    let s_addr = api_try!(emu, call.arg(emu, 0));
    let endptr = api_try!(emu, call.arg(emu, 1));
    let base = api_try!(emu, call.arg_u32(emu, 2));
    let s = emu.maps.read_string(s_addr);
    let (val, used) = crate::api::strconv::parse_c_integer(s.as_bytes(), base);
    if endptr != 0 {
        emu.maps.write_qword(endptr, s_addr + used as u64);
    }
    call.set_return(emu, val as u64);
}

fn api_strtol(emu: &mut Emu) {
    strtol_common(emu);
}

fn api_strtoul(emu: &mut Emu) {
    strtol_common(emu);
}

fn api_atoi(emu: &mut Emu) {
    let call = UnixApi;
    let s = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    let (val, _) = crate::api::strconv::parse_c_integer(s.as_bytes(), 10);
    call.set_return(emu, val as u64);
}

fn api_strerror(emu: &mut Emu) {
    let call = UnixApi;
    let errnum = api_try!(emu, call.arg_i32(emu, 0));
    let msg = match errnum {
        0 => "Success",
        1 => "Operation not permitted",
        2 => "No such file or directory",
        9 => "Bad file descriptor",
        12 => "Cannot allocate memory",
        13 => "Permission denied",
        22 => "Invalid argument",
        _ => "Unknown error",
    };
    let r = alloc_cstring(emu, msg);
    call.set_return(emu, r);
}

fn api_getenv(emu: &mut Emu) {
    let call = UnixApi;
    let name = emu.maps.read_string(api_try!(emu, call.arg(emu, 0)));
    trace(emu, &format!("getenv(\"{}\") -> NULL", name));
    call.set_return(emu, 0);
}

fn api_setlocale(emu: &mut Emu) {
    let call = UnixApi;
    let r = alloc_cstring(emu, "C");
    trace(emu, "setlocale() -> \"C\"");
    call.set_return(emu, r);
}

fn api_getpid(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, &format!("getpid() -> {}", FAKE_PID));
    call.set_return(emu, FAKE_PID);
}

fn api_getuid(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, &format!("getuid() -> {}", FAKE_UID));
    call.set_return(emu, FAKE_UID);
}

fn api_isatty(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg(emu, 0));
    let r = u64::from(fd <= 2);
    trace(emu, &format!("isatty({}) -> {}", fd, r));
    call.set_return(emu, r);
}

fn api_ioctl(emu: &mut Emu) {
    let call = UnixApi;
    const TIOCGWINSZ: u64 = 0x5413;
    let fd = api_try!(emu, call.arg(emu, 0));
    let request = api_try!(emu, call.arg(emu, 1));
    let buf = api_try!(emu, call.arg(emu, 2));
    trace(emu, &format!("ioctl({}, 0x{:x})", fd, request));
    if request == TIOCGWINSZ && buf != 0 {
        emu.maps.write_word(buf, 24);
        emu.maps.write_word(buf + 2, 80);
        emu.maps.write_dword(buf + 4, 0);
        call.set_return(emu, 0);
        return;
    }
    call.set_return(emu, -1i64 as u64);
}

fn api_signal(emu: &mut Emu) {
    let call = UnixApi;
    let sig = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("signal({}) -> SIG_DFL", sig));
    call.set_return(emu, 0);
}

fn api_time(emu: &mut Emu) {
    let call = UnixApi;
    let tloc = api_try!(emu, call.arg(emu, 0));
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    if tloc != 0 {
        emu.maps.write_qword(tloc, now);
    }
    trace(emu, &format!("time() -> {}", now));
    call.set_return(emu, now);
}

fn api_getopt(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, "getopt() -> -1");
    call.set_return(emu, -1i64 as u64);
}

fn prot_to_permission(prot: u64) -> Permission {
    match (prot & 1 != 0, prot & 2 != 0, prot & 4 != 0) {
        (_, true, true) => Permission::READ_WRITE_EXECUTE,
        (_, false, true) => Permission::READ_EXECUTE,
        _ => Permission::READ_WRITE,
    }
}

fn api_mmap(emu: &mut Emu) {
    let call = UnixApi;
    let len = api_try!(emu, call.arg(emu, 1));
    let prot = api_try!(emu, call.arg(emu, 2));
    trace(emu, &format!("mmap(len=0x{:x}, prot=0x{:x})", len, prot));
    if len == 0 {
        call.set_return(emu, u64::MAX);
        return;
    }
    let Some(base) = emu.maps.alloc(len) else {
        call.set_return(emu, u64::MAX);
        return;
    };
    if emu
        .maps
        .create_map(
            &format!("mmap_{:x}", base),
            base,
            len,
            prot_to_permission(prot),
        )
        .is_err()
    {
        call.set_return(emu, u64::MAX);
        return;
    }
    log::info!("  -> 0x{:x}", base);
    call.set_return(emu, base);
}

fn api_munmap(emu: &mut Emu) {
    let call = UnixApi;
    let addr = api_try!(emu, call.arg(emu, 0));
    trace(emu, &format!("munmap(0x{:x})", addr));
    if let Some(mem) = emu.maps.get_mem_by_addr(addr)
        && mem.get_base() == addr
        && mem.get_name().starts_with("mmap_")
    {
        emu.maps.dealloc(addr);
    }
    call.set_return(emu, 0);
}
