use crate::api::abi::unix::{ArgumentError, UnixApi, VariadicConvention::SystemV};
use crate::api::dispatch_api;
use crate::api::printf;
use crate::api::strconv::integer;
use crate::api::unix::{self, api_try, memory};
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{EFAULT, ENAMETOOLONG, ENOENT, ENOSYS};
use crate::syscall::linux::{current_tid, io};

use crate::syscall::linux::GUEST_ID;

pub fn gateway(symbol: &str, emu: &mut Emu) {
    if memory::dispatch(emu, symbol, false) {
        return;
    }
    dispatch_api!(symbol, {
        "__libc_start_main" => api_libc_start_main(emu),
        "__cxa_finalize" => api_cxa_finalize(emu),
        "__cxa_atexit" | "atexit" => api_cxa_atexit(emu),
        "__gmon_start__" => api_gmon_start(emu),
        "__stack_chk_fail" => api_stack_chk_fail(emu),
        "sysconf"=>unix::sysconf(emu),
        "getpagesize"=>unix::getpagesize(emu),
        "__errno_location" => unix::errno_location(emu, symbol),
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
        "write" | "__write" => api_write(emu),
        "read" | "__read" => api_read(emu),
        "open" | "open64" | "__open" | "__open64" => api_open(emu),
        "close" | "__close" => api_close(emu),
        "atoi" | "atol" | "atoll" | "strtol" | "strtoll" | "strtoimax" | "strtoul" | "strtoull" | "strtoumax" => integer(emu, symbol),
        "strdup" => api_strdup(emu),
        "strerror" => api_strerror(emu),
        "getenv" | "secure_getenv" => api_getenv(emu),
        "setlocale" => api_setlocale(emu),
        "getpid" => unix::getpid(emu),
        "getppid" => unix::getppid(emu),
        "gettid" => api_gettid(emu),
        "getuid" | "geteuid" | "getgid" | "getegid" => api_getuid(emu),
        "isatty" => api_isatty(emu),
        "ioctl" => api_ioctl(emu),
        "signal" => api_signal(emu),
        "time" => api_time(emu),
        "getopt" | "getopt_long" => api_getopt(emu),
        "mmap" | "mmap64" => api_mmap(emu),
        "munmap" => api_munmap(emu),
    },
        handled(()) => (),
        fallback => unsupported(emu, symbol),
    )
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

fn set_io_result(emu: &mut Emu, result: u64) {
    let call = UnixApi;
    if (result as i64) < 0 {
        let errno = result.wrapping_neg() as u32;
        if let Some(address) = unix::errno_address(emu) {
            api_try!(emu, memory::write(emu, address, &errno.to_le_bytes()));
        }
        call.set_return(emu, u64::MAX);
    } else {
        call.set_return(emu, result);
    }
}

fn api_write(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg_u32(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let result = io::write_buffers(
        emu,
        emu.cfg.arch,
        fd,
        &[(buffer, count)],
        |emu, _, bytes| {
            emu.emulated_stdout.extend_from_slice(bytes);
        },
    )
    .unwrap_or_else(|error| error);
    trace(
        emu,
        &format!(
            "write(fd={fd}, buffer={buffer:#x}, count={count}) = {}",
            result as i64
        ),
    );
    set_io_result(emu, result);
}

fn api_read(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg_u32(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let result = io::read_fd(emu, emu.cfg.arch, fd, buffer, count);
    trace(
        emu,
        &format!(
            "read(fd={fd}, buffer={buffer:#x}, count={count}) = {}",
            result as i64
        ),
    );
    set_io_result(emu, result);
}

fn api_open(emu: &mut Emu) {
    const PATH_MAX: u64 = 4096;
    let call = UnixApi;
    let path = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let error = match memory::length(emu, path, PATH_MAX) {
        Err(_) => EFAULT,
        Ok(0) => ENOENT,
        Ok(PATH_MAX) => ENAMETOOLONG,
        Ok(_) => ENOSYS,
    };
    trace(
        emu,
        &format!("open(path={path:#x}, flags={flags:#x}) = {}", error as i64),
    );
    set_io_result(emu, error);
}

fn api_close(emu: &mut Emu) {
    let call = UnixApi;
    let fd = api_try!(emu, call.arg_u32(emu, 0));
    let result = io::close_fd(fd);
    trace(emu, &format!("close(fd={fd}) = {}", result as i64));
    set_io_result(emu, result);
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

fn api_strdup(emu: &mut Emu) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, memory::length(emu, source, u64::MAX)) + 1;
    let destination = if emu.cfg.memory_guard {
        emu.kernel_alloc(Region::Slab, size, "malloc", "strdup", false)
    } else {
        linux_allocate(emu, size)
    };
    trace(emu, &format!("strdup(0x{source:x}) -> 0x{destination:x}"));
    if destination != 0 {
        api_try!(emu, memory::copy(emu, destination, source, size));
    }
    call.set_return(emu, destination);
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

fn api_gettid(emu: &mut Emu) {
    let call = UnixApi;
    let tid = current_tid(emu);
    trace(emu, &format!("gettid() -> {tid}"));
    call.set_return(emu, tid);
}

fn api_getuid(emu: &mut Emu) {
    let call = UnixApi;
    trace(emu, &format!("getuid() -> {}", GUEST_ID));
    call.set_return(emu, GUEST_ID.into());
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

fn unsupported(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    log::warn!("linuxapi libc: unimplemented API {} -- returning 0", symbol);
    call.set_return(emu, 0);
}
