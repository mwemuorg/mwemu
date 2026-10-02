use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::maps::mem64::Permission;

/// Read argument register by index (0-7), arch-agnostic.
/// AArch64: x0-x7, x86_64: rdi, rsi, rdx, rcx, r8, r9 (SysV ABI)
fn arg(emu: &Emu, idx: usize) -> u64 {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64().x[idx]
    } else {
        match idx {
            0 => emu.regs().rdi,
            1 => emu.regs().rsi,
            2 => emu.regs().rdx,
            3 => emu.regs().rcx,
            4 => emu.regs().r8,
            5 => emu.regs().r9,
            _ => unreachable!("arg index {} not supported for x86_64 SysV ABI", idx),
        }
    }
}

/// Set return value, arch-agnostic.
/// AArch64: x0, x86_64: rax
pub(super) fn set_ret_pub(emu: &mut Emu, val: u64) {
    set_ret(emu, val);
}

fn set_ret(emu: &mut Emu, val: u64) {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().x[0] = val;
    } else {
        emu.regs_mut().rax = val;
    }
}

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
fn allocate_memory(emu: &mut Emu, size: u64) -> Option<u64> {
    if size < LARGE_ALLOC_THRESHOLD {
        let heap_manage = emu.heap_mut();
        return heap_manage.allocate(size as usize);
    }

    let allocation = emu.maps.alloc(size)?;
    emu.maps
        .create_map(
            &format!("alloc_{:x}", allocation),
            allocation,
            size,
            Permission::READ_WRITE,
        )
        .ok()?;
    Some(allocation)
}

/// Classification of a pointer allocated by `allocate_memory`.
enum AllocKind {
    Arena { size: usize },
    Map { base: u64, size: usize },
    Invalid,
}

fn classify(emu: &Emu, addr: u64) -> AllocKind {
    if let Some(heap) = emu.heap_arenas.first()
        && let Some(size) = heap.allocation_size(addr)
    {
        return AllocKind::Arena { size };
    }

    match emu.maps.get_mem_by_addr(addr) {
        Some(mem) if mem.get_base() == addr && mem.get_name().starts_with("alloc_") => {
            AllocKind::Map {
                base: addr,
                size: mem.size(),
            }
        }
        _ => AllocKind::Invalid,
    }
}

/// Releases a pointer allocated by `allocate_memory` (or any alloc_ map).
fn release(emu: &mut Emu, addr: u64) {
    match classify(emu, addr) {
        AllocKind::Arena { .. } => {
            if let Some(heap) = emu.heap_arenas.first_mut() {
                heap.free(addr);
            }
        }
        AllocKind::Map { base, .. } => emu.maps.dealloc(base),
        AllocKind::Invalid => {}
    }
}

pub fn gateway(symbol: &str, emu: &mut Emu) {
    match symbol {
        "_printf" | "printf" => api_printf(emu),
        "_fprintf" | "fprintf" => api_fprintf(emu),
        "_sprintf" | "sprintf" => api_sprintf(emu),
        "_snprintf" | "snprintf" => api_snprintf(emu),
        "_puts" | "puts" => api_puts(emu),
        "_putchar" | "putchar" => api_putchar(emu),
        "_exit" | "exit" | "__exit" => api_exit(emu),
        "_abort" | "abort" => api_abort(emu),
        "_malloc" | "malloc" => api_malloc(emu),
        "_calloc" | "calloc" => api_calloc(emu),
        "_realloc" | "realloc" => api_realloc(emu),
        "_free" | "free" => api_free(emu),
        "_atexit" | "atexit" => api_atexit(emu),
        "_write" | "write" => api_write(emu),
        "_read" | "read" => api_read(emu),
        "_open" | "open" => api_open(emu),
        "_close" | "close" => api_close(emu),
        "_memcpy" | "memcpy" => api_memcpy(emu),
        "_memmove" | "memmove" => api_memmove(emu),
        "_memset" | "memset" => api_memset(emu),
        "_memcmp" | "memcmp" => api_memcmp(emu),
        "_strlen" | "strlen" => api_strlen(emu),
        "_strcmp" | "strcmp" => api_strcmp(emu),
        "_strncmp" | "strncmp" => api_strncmp(emu),
        "_strcpy" | "strcpy" => api_strcpy(emu),
        "_strncpy" | "strncpy" => api_strncpy(emu),
        "_strcat" | "strcat" => api_strcat(emu),
        "_strchr" | "strchr" => api_strchr(emu),
        "_strrchr" | "strrchr" => api_strrchr(emu),
        "_strstr" | "strstr" => api_strstr(emu),
        "_strdup" | "strdup" => api_strdup(emu),
        "_mmap" | "mmap" => api_mmap(emu),
        "_munmap" | "munmap" => api_munmap(emu),
        "_mprotect" | "mprotect" => api_mprotect(emu),
        "_madvise" | "madvise" => api_madvise(emu),
        "_strncat" | "strncat" => api_strncat(emu),
        "_strlcpy" | "strlcpy" => api_strlcpy(emu),
        "_strlcat" | "strlcat" => api_strlcat(emu),
        "_bzero" | "bzero" => api_bzero(emu),
        "_memchr" | "memchr" => api_memchr(emu),

        // --- macOS-specific / ls-required APIs ---
        "_setlocale" | "setlocale" => api_setlocale(emu),
        "_getenv" | "getenv" => api_getenv(emu),
        "_setenv" | "setenv" => api_setenv(emu),
        "_isatty" | "isatty" => api_isatty(emu),
        "_ioctl" | "ioctl" => api_ioctl(emu),
        "_getopt_long" | "getopt_long" => api_getopt_long(emu),
        "_signal" | "signal" => api_signal(emu),
        "_kill" | "kill" => api_kill(emu),
        "_getuid" | "getuid" => api_getuid(emu),
        "_getpid" | "getpid" => api_getpid(emu),
        "___error" | "__error" => api___error(emu),
        "___stack_chk_fail" | "__stack_chk_fail" => api___stack_chk_fail(emu),
        "___maskrune" | "__maskrune" => api___maskrune(emu),
        "___tolower" | "__tolower" => api___tolower(emu),
        "___assert_rtn" | "__assert_rtn" => api___assert_rtn(emu),
        "_err" | "err" => api_err(emu),
        "_errx" | "errx" => api_errx(emu),
        "_warn" | "warn" => api_warn(emu),
        "_warnx" | "warnx" => api_warnx(emu),
        "_strerror" | "strerror" => api_strerror(emu),
        "_fflush" | "fflush" => api_fflush(emu),
        "___swbuf" | "__swbuf" => api___swbuf(emu),
        "_fputc" | "fputc" => api_fputc(emu),
        "_fputs" | "fputs" => api_fputs(emu),
        "_fwrite" | "fwrite" => api_fwrite(emu),
        "_ferror" | "ferror" => api_ferror(emu),
        "_strcoll" | "strcoll" => api_strcoll(emu),
        "_strtoul" | "strtoul" => api_strtoul(emu),
        "_time" | "time" => api_time(emu),
        "_localtime" | "localtime" => api_localtime(emu),
        "_strftime" | "strftime" => api_strftime(emu),
        "_readlink" | "readlink" => api_readlink(emu),
        "_strmode" | "strmode" => api_strmode(emu),
        "_user_from_uid" | "user_from_uid" => api_user_from_uid(emu),
        "_group_from_gid" | "group_from_gid" => api_group_from_gid(emu),
        "_reallocf" | "reallocf" => api_realloc(emu),
        "_nl_langinfo" | "nl_langinfo" => api_nl_langinfo(emu),
        "_mbrtowc" | "mbrtowc" => api_mbrtowc(emu),
        "_wcwidth" | "wcwidth" => api_wcwidth(emu),
        "_getbsize" | "getbsize" => api_getbsize(emu),
        "_fflagstostr" | "fflagstostr" => api_fflagstostr(emu),
        "_compat_mode" | "compat_mode" => api_compat_mode(emu),
        "_sysctlbyname" | "sysctlbyname" => api_sysctlbyname(emu),
        "_getxattr" | "getxattr" => api_getxattr(emu),
        "_listxattr" | "listxattr" => api_listxattr(emu),
        "_humanize_number" | "humanize_number" => api_humanize_number(emu),
        "_strtonum" | "strtonum" => api_strtonum(emu),
        "_uuid_unparse_upper" | "uuid_unparse_upper" => api_uuid_unparse_upper(emu),
        "_mbr_identifier_translate" | "mbr_identifier_translate" => {
            api_mbr_identifier_translate(emu)
        }

        // termcap
        "_tgetent" | "tgetent" | "_tgetstr" | "tgetstr" | "_tgoto" | "tgoto" | "_tputs"
        | "tputs" => {
            log::info!(
                "{}** {} macOS API {}() -> 0 {}",
                emu.colors.light_red,
                emu.pos,
                symbol,
                emu.colors.nc
            );
            set_ret(emu, 0);
        }

        // ACL stubs
        s if s.starts_with("_acl_") || s.starts_with("acl_") => {
            log::info!(
                "{}** {} macOS API {}() -> 0 {}",
                emu.colors.light_red,
                emu.pos,
                symbol,
                emu.colors.nc
            );
            set_ret(emu, 0);
        }

        // FTS filesystem traversal ($INODE64 variants)
        "_fts_open$INODE64" | "_fts_open" | "fts_open$INODE64" | "fts_open" => api_fts_open(emu),
        "_fts_read$INODE64" | "_fts_read" | "fts_read$INODE64" | "fts_read" => api_fts_read(emu),
        "_fts_close$INODE64" | "_fts_close" | "fts_close$INODE64" | "fts_close" => {
            api_fts_close(emu)
        }
        "_fts_set$INODE64" | "_fts_set" | "fts_set$INODE64" | "fts_set" => api_fts_set(emu),
        "_fts_children$INODE64" | "_fts_children" | "fts_children$INODE64" | "fts_children" => {
            api_fts_children(emu)
        }

        // stat family ($INODE64 variants)
        "_stat$INODE64" | "_stat" | "stat$INODE64" | "stat" | "_stat64" | "stat64" => api_stat(emu),
        "_lstat$INODE64" | "_lstat" | "lstat$INODE64" | "lstat" | "_lstat64" | "lstat64" => {
            api_lstat(emu)
        }
        "_fstat$INODE64" | "_fstat" | "fstat$INODE64" | "fstat" | "_fstat64" | "fstat64" => {
            api_fstat(emu)
        }

        _ => {
            log::warn!("libsystem: unimplemented API {} -- returning 0", symbol);
            set_ret(emu, 0);
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let fmt_addr = arg(emu, 0);
    let fmt = emu.maps.read_string(fmt_addr);
    let result = format_printf(emu, &fmt, 1);
    log::info!(
        "{}** {} macOS API printf(\"{}\") -> \"{}\" {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        result,
        emu.colors.nc
    );
    emu.emulated_stdout.extend_from_slice(result.as_bytes());
    set_ret(emu, result.len() as u64);
}

fn api_fprintf(emu: &mut Emu) {
    let _stream = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    let result = format_printf(emu, &fmt, 2);
    log::info!(
        "{}** {} macOS API fprintf(\"{}\") -> \"{}\" {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        result,
        emu.colors.nc
    );
    emu.emulated_stdout.extend_from_slice(result.as_bytes());
    set_ret(emu, result.len() as u64);
}

fn api_sprintf(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    let result = format_printf(emu, &fmt, 2);
    log::info!(
        "{}** {} macOS API sprintf(0x{:x}, \"{}\") -> \"{}\" {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        fmt,
        result,
        emu.colors.nc
    );
    let bytes = result.as_bytes();
    emu.maps.write_bytes(dst_addr, bytes);
    emu.maps.write_byte(dst_addr + bytes.len() as u64, 0);
    set_ret(emu, bytes.len() as u64);
}

fn api_snprintf(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let size = arg(emu, 1);
    let fmt_addr = arg(emu, 2);
    let fmt = emu.maps.read_string(fmt_addr);
    let result = format_printf(emu, &fmt, 3);
    log::info!(
        "{}** {} macOS API snprintf(0x{:x}, {}, \"{}\") -> \"{}\" {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        size,
        fmt,
        result,
        emu.colors.nc
    );
    let bytes = result.as_bytes();
    if size > 0 {
        let write_len = std::cmp::min(bytes.len(), (size - 1) as usize);
        emu.maps.write_bytes(dst_addr, &bytes[..write_len]);
        emu.maps.write_byte(dst_addr + write_len as u64, 0);
    }
    set_ret(emu, bytes.len() as u64);
}

fn format_printf(emu: &Emu, fmt: &str, first_vararg: usize) -> String {
    let mut out = String::new();
    let mut vararg_num: usize = 0;
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] != '%' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        i += 1;
        if i >= chars.len() {
            break;
        }
        if chars[i] == '%' {
            out.push('%');
            i += 1;
            continue;
        }

        // Parse flags
        let mut left_align = false;
        let mut zero_pad = false;
        let mut plus_sign = false;
        let mut space_sign = false;
        loop {
            if i >= chars.len() {
                break;
            }
            match chars[i] {
                '-' => left_align = true,
                '0' => zero_pad = true,
                '+' => plus_sign = true,
                ' ' => space_sign = true,
                '#' => {}
                _ => break,
            }
            i += 1;
        }

        // Parse width
        let mut width: Option<usize> = None;
        if i < chars.len() && chars[i] == '*' {
            width = Some(get_printf_arg(emu, vararg_num, first_vararg) as usize);
            vararg_num += 1;
            i += 1;
        } else {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                width = chars[start..i]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .ok();
            }
        }

        // Parse precision
        let mut _precision: Option<usize> = None;
        if i < chars.len() && chars[i] == '.' {
            i += 1;
            if i < chars.len() && chars[i] == '*' {
                _precision = Some(get_printf_arg(emu, vararg_num, first_vararg) as usize);
                vararg_num += 1;
                i += 1;
            } else {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                if i > start {
                    _precision = chars[start..i]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .ok();
                }
            }
        }

        // Parse length modifier
        let mut _long_count = 0;
        while i < chars.len() {
            match chars[i] {
                'l' => {
                    _long_count += 1;
                    i += 1;
                }
                'h' | 'j' | 'z' | 't' | 'q' => {
                    i += 1;
                }
                _ => break,
            }
        }

        if i >= chars.len() {
            break;
        }

        // Conversion
        let w = width.unwrap_or(0);
        let val = get_printf_arg(emu, vararg_num, first_vararg);
        vararg_num += 1;
        let formatted = match chars[i] {
            'd' | 'i' => {
                let v = val as i64;
                let s = if plus_sign && v >= 0 {
                    format!("+{}", v)
                } else if space_sign && v >= 0 {
                    format!(" {}", v)
                } else {
                    format!("{}", v)
                };
                pad_string(&s, w, left_align, zero_pad)
            }
            'u' => pad_string(&format!("{}", val), w, left_align, false),
            'x' => pad_string(&format!("{:x}", val), w, left_align, zero_pad),
            'X' => pad_string(&format!("{:X}", val), w, left_align, zero_pad),
            'o' => pad_string(&format!("{:o}", val), w, left_align, zero_pad),
            's' => {
                let s = if val != 0 {
                    emu.maps.read_string(val)
                } else {
                    "(null)".to_string()
                };
                pad_string(&s, w, left_align, false)
            }
            'c' => {
                let ch = if val > 0 && val < 128 {
                    (val as u8) as char
                } else {
                    '?'
                };
                pad_string(&ch.to_string(), w, left_align, false)
            }
            'p' => pad_string(&format!("0x{:x}", val), w, left_align, false),
            _ => {
                format!("%{}", chars[i])
            }
        };
        out.push_str(&formatted);
        i += 1;
    }
    out
}

fn get_printf_arg(emu: &Emu, vararg_num: usize, first_vararg: usize) -> u64 {
    if emu.cfg.arch.is_aarch64() {
        // AArch64 Apple ABI: variadic args are always on the stack.
        let sp = emu.regs_aarch64().sp;
        emu.maps
            .read_qword(sp + (vararg_num as u64) * 8)
            .unwrap_or(0)
    } else {
        let idx = first_vararg + vararg_num;
        if idx < 6 {
            match idx {
                0 => emu.regs().rdi,
                1 => emu.regs().rsi,
                2 => emu.regs().rdx,
                3 => emu.regs().rcx,
                4 => emu.regs().r8,
                5 => emu.regs().r9,
                _ => unreachable!(),
            }
        } else {
            let sp = emu.regs().rsp;
            emu.maps
                .read_qword(sp + ((idx - 6) as u64) * 8)
                .unwrap_or(0)
        }
    }
}

fn pad_string(s: &str, width: usize, left_align: bool, zero_pad: bool) -> String {
    if width == 0 || s.len() >= width {
        return s.to_string();
    }
    let pad_char = if zero_pad && !left_align { '0' } else { ' ' };
    let padding = width - s.len();
    if left_align {
        format!("{}{}", s, " ".repeat(padding))
    } else {
        format!(
            "{}{}",
            std::iter::repeat_n(pad_char, padding).collect::<String>(),
            s
        )
    }
}

fn api_puts(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API puts(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );
    emu.emulated_stdout.extend_from_slice(s.as_bytes());
    emu.emulated_stdout.push(b'\n');
    set_ret(emu, 0);
}

fn api_putchar(emu: &mut Emu) {
    let c = arg(emu, 0) as u8;
    log::info!(
        "{}** {} macOS API putchar('{}') {}",
        emu.colors.light_red,
        emu.pos,
        c as char,
        emu.colors.nc
    );
    emu.emulated_stdout.push(c);
    set_ret(emu, c as u64);
}

fn api_exit(emu: &mut Emu) {
    let status = arg(emu, 0);
    // Flush stdout and stderr buffers before exiting
    flush_stdio_on_exit(emu);
    log::info!(
        "{}** {} macOS API exit({}) {}",
        emu.colors.light_red,
        emu.pos,
        status,
        emu.colors.nc
    );
    emu.stop();
}

fn flush_stdio_on_exit(emu: &mut Emu) {
    flush_file_buffer_to_stdout(emu);
    if !emu.emulated_stdout.is_empty() {
        let s = String::from_utf8_lossy(&emu.emulated_stdout).to_string();
        print!("{}", s);
        emu.emulated_stdout.clear();
    }
}

fn api_abort(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API abort() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.stop();
}

fn api_malloc(emu: &mut Emu) {
    let size = arg(emu, 0);
    log::info!(
        "{}** {} macOS API malloc({}) {}",
        emu.colors.light_red,
        emu.pos,
        size,
        emu.colors.nc
    );
    if size > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, size, "malloc", "malloc", false)
        } else {
            allocate_memory(emu, size).expect("macOS malloc: out of memory")
        };
        log::info!("  -> 0x{:x}", base);
        set_ret(emu, base);
    } else {
        set_ret(emu, 0);
    }
}

fn api_calloc(emu: &mut Emu) {
    let count = arg(emu, 0);
    let size = arg(emu, 1);
    let total = count.saturating_mul(size);
    log::info!(
        "{}** {} macOS API calloc({}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        count,
        size,
        emu.colors.nc
    );
    if total > 0 {
        let base = if emu.cfg.memory_guard {
            emu.kernel_alloc(Region::Slab, total, "malloc", "calloc", true)
        } else {
            let b = allocate_memory(emu, total).expect("macOS calloc: out of memory");
            for i in 0..total {
                emu.maps.write_byte(b + i, 0);
            }
            b
        };
        log::info!("  -> 0x{:x}", base);
        set_ret(emu, base);
    } else {
        set_ret(emu, 0);
    }
}

fn api_realloc(emu: &mut Emu) {
    let ptr = arg(emu, 0);
    let size = arg(emu, 1);
    log::info!(
        "{}** {} macOS API realloc(0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        size,
        emu.colors.nc
    );
    if size == 0 {
        if ptr != 0 && emu.cfg.memory_guard {
            emu.kernel_free(ptr, "realloc");
        }
        set_ret(emu, 0);
        return;
    }
    let base = if emu.cfg.memory_guard {
        emu.kernel_alloc(Region::Slab, size, "malloc", "realloc", false)
    } else {
        allocate_memory(emu, size).expect("macOS realloc: out of memory")
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
            release(emu, ptr);
        }
    }
    log::info!("  -> 0x{:x}", base);
    set_ret(emu, base);
}

fn api_free(emu: &mut Emu) {
    let ptr = arg(emu, 0);
    log::info!(
        "{}** {} macOS API free(0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );
    if emu.cfg.memory_guard {
        emu.kernel_free(ptr, "free");
    } else {
        release(emu, ptr);
    }
}

fn api_atexit(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API atexit() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_write(emu: &mut Emu) {
    let fd = arg(emu, 0);
    let buf = arg(emu, 1);
    let count = arg(emu, 2);
    let s = emu.maps.read_string(buf);
    log::info!(
        "{}** {} macOS API write(fd={}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        s,
        count,
        emu.colors.nc
    );
    set_ret(emu, count);
}

fn api_read(emu: &mut Emu) {
    let fd = arg(emu, 0);
    let buf = arg(emu, 1);
    let count = arg(emu, 2);
    log::info!(
        "{}** {} macOS API read(fd={}, buf=0x{:x}, count={}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buf,
        count,
        emu.colors.nc
    );
    // Stub: return 0 bytes read (EOF)
    set_ret(emu, 0);
}

fn api_open(emu: &mut Emu) {
    let path_addr = arg(emu, 0);
    let flags = arg(emu, 1);
    let path = emu.maps.read_string(path_addr);
    log::info!(
        "{}** {} macOS API open(\"{}\", 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        path,
        flags,
        emu.colors.nc
    );
    // Stub: return fd 3 (fake file descriptor)
    set_ret(emu, 3);
}

fn api_close(emu: &mut Emu) {
    let fd = arg(emu, 0);
    log::info!(
        "{}** {} macOS API close(fd={}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_memcpy(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let src = arg(emu, 1);
    let n = arg(emu, 2);
    log::info!(
        "{}** {} macOS API memcpy(0x{:x}, 0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        src,
        n,
        emu.colors.nc
    );
    for i in 0..n {
        let b = emu.maps.read_byte(src + i).unwrap_or(0);
        emu.maps.write_byte(dst + i, b);
    }
    set_ret(emu, dst);
}

fn api_memmove(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let src = arg(emu, 1);
    let n = arg(emu, 2);
    log::info!(
        "{}** {} macOS API memmove(0x{:x}, 0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        src,
        n,
        emu.colors.nc
    );
    // Read all bytes first to handle overlapping regions
    let mut tmp = vec![0u8; n as usize];
    for i in 0..n {
        tmp[i as usize] = emu.maps.read_byte(src + i).unwrap_or(0);
    }
    for i in 0..n {
        emu.maps.write_byte(dst + i, tmp[i as usize]);
    }
    set_ret(emu, dst);
}

fn api_memset(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let c = (arg(emu, 1) & 0xff) as u8;
    let n = arg(emu, 2);
    log::info!(
        "{}** {} macOS API memset(0x{:x}, 0x{:02x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        c,
        n,
        emu.colors.nc
    );
    for i in 0..n {
        emu.maps.write_byte(dst + i, c);
    }
    set_ret(emu, dst);
}

fn api_memcmp(emu: &mut Emu) {
    let s1 = arg(emu, 0);
    let s2 = arg(emu, 1);
    let n = arg(emu, 2);
    log::info!(
        "{}** {} macOS API memcmp(0x{:x}, 0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        s1,
        s2,
        n,
        emu.colors.nc
    );
    let mut result: i32 = 0;
    for i in 0..n {
        let a = emu.maps.read_byte(s1 + i).unwrap_or(0);
        let b = emu.maps.read_byte(s2 + i).unwrap_or(0);
        if a != b {
            result = (a as i32) - (b as i32);
            break;
        }
    }
    set_ret(emu, result as u64);
}

fn api_memchr(emu: &mut Emu) {
    let s = arg(emu, 0);
    let c = (arg(emu, 1) & 0xff) as u8;
    let n = arg(emu, 2);
    log::info!(
        "{}** {} macOS API memchr(0x{:x}, 0x{:02x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        s,
        c,
        n,
        emu.colors.nc
    );
    let mut found: u64 = 0; // NULL = not found
    for i in 0..n {
        let b = emu.maps.read_byte(s + i).unwrap_or(0);
        if b == c {
            found = s + i;
            break;
        }
    }
    set_ret(emu, found);
}

fn api_strlen(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API strlen(0x{:x}) = {} {}",
        emu.colors.light_red,
        emu.pos,
        s_addr,
        s.len(),
        emu.colors.nc
    );
    set_ret(emu, s.len() as u64);
}

fn api_strcmp(emu: &mut Emu) {
    let s1_addr = arg(emu, 0);
    let s2_addr = arg(emu, 1);
    let s1 = emu.maps.read_string(s1_addr);
    let s2 = emu.maps.read_string(s2_addr);
    log::info!(
        "{}** {} macOS API strcmp(\"{}\", \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s1,
        s2,
        emu.colors.nc
    );
    let result = match s1.cmp(&s2) {
        std::cmp::Ordering::Less => -1i64 as u64,
        std::cmp::Ordering::Equal => 0u64,
        std::cmp::Ordering::Greater => 1u64,
    };
    set_ret(emu, result);
}

fn api_strncmp(emu: &mut Emu) {
    let s1_addr = arg(emu, 0);
    let s2_addr = arg(emu, 1);
    let n = arg(emu, 2) as usize;
    let s1 = emu.maps.read_string(s1_addr);
    let s2 = emu.maps.read_string(s2_addr);
    log::info!(
        "{}** {} macOS API strncmp(\"{}\", \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        s1,
        s2,
        n,
        emu.colors.nc
    );
    let s1_trunc: String = s1.chars().take(n).collect();
    let s2_trunc: String = s2.chars().take(n).collect();
    let result = match s1_trunc.cmp(&s2_trunc) {
        std::cmp::Ordering::Less => -1i64 as u64,
        std::cmp::Ordering::Equal => 0u64,
        std::cmp::Ordering::Greater => 1u64,
    };
    set_ret(emu, result);
}

fn api_strcpy(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strcpy(0x{:x}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        s,
        emu.colors.nc
    );
    let bytes = s.as_bytes();
    emu.maps.write_bytes(dst, bytes);
    emu.maps.write_byte(dst + bytes.len() as u64, 0);
    set_ret(emu, dst);
}

fn api_strncpy(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let n = arg(emu, 2);
    let s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strncpy(0x{:x}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        s,
        n,
        emu.colors.nc
    );
    let bytes = s.as_bytes();
    let copy_len = std::cmp::min(bytes.len(), n as usize);
    emu.maps.write_bytes(dst, &bytes[..copy_len]);
    // Pad with NULs up to n
    for i in copy_len..(n as usize) {
        emu.maps.write_byte(dst + i as u64, 0);
    }
    set_ret(emu, dst);
}

fn api_strcat(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let dst_s = emu.maps.read_string(dst_addr);
    let src_s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strcat(0x{:x}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        src_s,
        emu.colors.nc
    );
    let dst_len = dst_s.len() as u64;
    let src_bytes = src_s.as_bytes();
    emu.maps.write_bytes(dst_addr + dst_len, src_bytes);
    emu.maps
        .write_byte(dst_addr + dst_len + src_bytes.len() as u64, 0);
    set_ret(emu, dst_addr);
}

fn api_strncat(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let n = arg(emu, 2) as usize;
    let dst_s = emu.maps.read_string(dst_addr);
    let src_s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strncat(0x{:x}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        src_s,
        n,
        emu.colors.nc
    );
    let dst_len = dst_s.len() as u64;
    let src_bytes = src_s.as_bytes();
    let copy_len = std::cmp::min(src_bytes.len(), n);
    emu.maps
        .write_bytes(dst_addr + dst_len, &src_bytes[..copy_len]);
    emu.maps.write_byte(dst_addr + dst_len + copy_len as u64, 0);
    set_ret(emu, dst_addr);
}

fn api_strlcpy(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let size = arg(emu, 2) as usize;
    let s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strlcpy(0x{:x}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        s,
        size,
        emu.colors.nc
    );
    let bytes = s.as_bytes();
    if size > 0 {
        let copy_len = std::cmp::min(bytes.len(), size - 1);
        emu.maps.write_bytes(dst, &bytes[..copy_len]);
        emu.maps.write_byte(dst + copy_len as u64, 0);
    }
    // strlcpy returns strlen(src)
    set_ret(emu, bytes.len() as u64);
}

fn api_strlcat(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let src_addr = arg(emu, 1);
    let size = arg(emu, 2) as usize;
    let dst_s = emu.maps.read_string(dst_addr);
    let src_s = emu.maps.read_string(src_addr);
    log::info!(
        "{}** {} macOS API strlcat(0x{:x}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        src_s,
        size,
        emu.colors.nc
    );
    let dst_len = dst_s.len();
    let src_bytes = src_s.as_bytes();
    if dst_len < size {
        let remaining = size - dst_len - 1;
        let copy_len = std::cmp::min(src_bytes.len(), remaining);
        emu.maps
            .write_bytes(dst_addr + dst_len as u64, &src_bytes[..copy_len]);
        emu.maps
            .write_byte(dst_addr + (dst_len + copy_len) as u64, 0);
    }
    // strlcat returns min(size, dst_len) + strlen(src)
    let ret = std::cmp::min(size, dst_len) + src_bytes.len();
    set_ret(emu, ret as u64);
}

fn api_strchr(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let c = (arg(emu, 1) & 0xff) as u8;
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API strchr(\"{}\", '{}') {}",
        emu.colors.light_red,
        emu.pos,
        s,
        c as char,
        emu.colors.nc
    );
    let result = if c == 0 {
        // strchr for NUL returns pointer to terminator
        s_addr + s.len() as u64
    } else {
        match s.find(c as char) {
            Some(pos) => s_addr + pos as u64,
            None => 0, // NULL
        }
    };
    set_ret(emu, result);
}

fn api_strrchr(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let c = (arg(emu, 1) & 0xff) as u8;
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API strrchr(\"{}\", '{}') {}",
        emu.colors.light_red,
        emu.pos,
        s,
        c as char,
        emu.colors.nc
    );
    let result = if c == 0 {
        s_addr + s.len() as u64
    } else {
        match s.rfind(c as char) {
            Some(pos) => s_addr + pos as u64,
            None => 0,
        }
    };
    set_ret(emu, result);
}

fn api_strstr(emu: &mut Emu) {
    let haystack_addr = arg(emu, 0);
    let needle_addr = arg(emu, 1);
    let haystack = emu.maps.read_string(haystack_addr);
    let needle = emu.maps.read_string(needle_addr);
    log::info!(
        "{}** {} macOS API strstr(\"{}\", \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        haystack,
        needle,
        emu.colors.nc
    );
    let result = if needle.is_empty() {
        haystack_addr
    } else {
        match haystack.find(&needle) {
            Some(pos) => haystack_addr + pos as u64,
            None => 0,
        }
    };
    set_ret(emu, result);
}

fn api_strdup(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API strdup(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );
    let len = s.len() as u64 + 1; // include NUL
    let base = if emu.cfg.memory_guard {
        emu.kernel_alloc(Region::Slab, len, "malloc", "strdup", false)
    } else {
        allocate_memory(emu, len).expect("macOS strdup: out of memory")
    };
    let bytes = s.as_bytes();
    emu.maps.write_bytes(base, bytes);
    emu.maps.write_byte(base + bytes.len() as u64, 0);
    set_ret(emu, base);
}

fn api_bzero(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let n = arg(emu, 1);
    log::info!(
        "{}** {} macOS API bzero(0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        dst,
        n,
        emu.colors.nc
    );
    for i in 0..n {
        emu.maps.write_byte(dst + i, 0);
    }
    // bzero returns void; x0 is undefined but leave dst for convenience
}

fn api_mmap(emu: &mut Emu) {
    let addr = arg(emu, 0);
    let len = arg(emu, 1);
    let prot = arg(emu, 2);
    let flags = arg(emu, 3);
    let fd = arg(emu, 4);
    let offset = arg(emu, 5);
    log::info!(
        "{}** {} macOS API mmap(0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, {}, 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        addr,
        len,
        prot,
        flags,
        fd as i64,
        offset,
        emu.colors.nc
    );
    if len == 0 {
        // MAP_FAILED
        set_ret(emu, u64::MAX);
        return;
    }
    let permission = prot_to_permission(prot);
    let base = emu.maps.alloc(len).expect("macOS mmap: out of memory");
    emu.maps
        .create_map(&format!("mmap_{:x}", base), base, len, permission)
        .expect("macOS mmap: cannot create map");
    // zero-fill
    for i in 0..len {
        emu.maps.write_byte(base + i, 0);
    }
    log::info!("  -> 0x{:x}", base);
    set_ret(emu, base);
}

fn api_munmap(emu: &mut Emu) {
    let addr = arg(emu, 0);
    let len = arg(emu, 1);
    log::info!(
        "{}** {} macOS API munmap(0x{:x}, 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        addr,
        len,
        emu.colors.nc
    );
    // Stub: return success. We don't reclaim memory.
    set_ret(emu, 0);
}

fn api_mprotect(emu: &mut Emu) {
    let addr = arg(emu, 0);
    let len = arg(emu, 1);
    let prot = arg(emu, 2);
    log::info!(
        "{}** {} macOS API mprotect(0x{:x}, 0x{:x}, 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        addr,
        len,
        prot,
        emu.colors.nc
    );
    // Stub: return success
    set_ret(emu, 0);
}

fn api_madvise(emu: &mut Emu) {
    let addr = arg(emu, 0);
    let len = arg(emu, 1);
    let advice = arg(emu, 2);
    log::info!(
        "{}** {} macOS API madvise(0x{:x}, 0x{:x}, {}) {}",
        emu.colors.light_red,
        emu.pos,
        addr,
        len,
        advice,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

// ==================== macOS / ls-required API stubs ====================

fn api_setlocale(emu: &mut Emu) {
    let category = arg(emu, 0);
    let locale_addr = arg(emu, 1);
    let locale = if locale_addr != 0 {
        emu.maps.read_string(locale_addr)
    } else {
        String::new()
    };
    log::info!(
        "{}** {} macOS API setlocale({}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        category,
        locale,
        emu.colors.nc
    );
    let ret = alloc_string(emu, "C");
    set_ret(emu, ret);
}

fn api_getenv(emu: &mut Emu) {
    let name_addr = arg(emu, 0);
    let name = emu.maps.read_string(name_addr);
    log::info!(
        "{}** {} macOS API getenv(\"{}\") -> NULL {}",
        emu.colors.light_red,
        emu.pos,
        name,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_setenv(emu: &mut Emu) {
    let name_addr = arg(emu, 0);
    let val_addr = arg(emu, 1);
    let name = emu.maps.read_string(name_addr);
    let val = if val_addr != 0 {
        emu.maps.read_string(val_addr)
    } else {
        String::new()
    };
    log::info!(
        "{}** {} macOS API setenv(\"{}\", \"{}\") -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        name,
        val,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_isatty(emu: &mut Emu) {
    let fd = arg(emu, 0);
    log::info!(
        "{}** {} macOS API isatty({}) -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_ioctl(emu: &mut Emu) {
    let fd = arg(emu, 0);
    let request = arg(emu, 1);
    log::info!(
        "{}** {} macOS API ioctl({}, 0x{:x}) -> -1 {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        request,
        emu.colors.nc
    );
    set_ret(emu, -1i64 as u64);
}

fn find_global_addr(emu: &Emu, name: &str) -> Option<u64> {
    emu.macho64.as_ref().and_then(|m| {
        m.addr_to_symbol
            .iter()
            .find(|(_, n)| n.as_str() == name)
            .map(|(&a, _)| a)
    })
}

fn api_getopt_long(emu: &mut Emu) {
    let argc = arg(emu, 0) as i32;
    let argv_ptr = arg(emu, 1);
    let optstring_ptr = arg(emu, 2);

    let optstring = emu.maps.read_string(optstring_ptr);

    let optind_addr = find_global_addr(emu, "_optind").unwrap_or(0);
    let optarg_addr = find_global_addr(emu, "_optarg").unwrap_or(0);

    let optind = if optind_addr != 0 {
        emu.maps.read_dword(optind_addr).unwrap_or(1) as i32
    } else {
        1
    };

    // If we're mid-way through a grouped flag string (e.g. "-la"),
    // continue from where we left off.
    let char_idx = emu.getopt_char_index;

    if optind >= argc {
        emu.getopt_char_index = 0;
        log::info!(
            "{}** {} macOS API getopt_long() -> -1 (done) {}",
            emu.colors.light_red,
            emu.pos,
            emu.colors.nc
        );
        set_ret(emu, -1i64 as u64);
        return;
    }

    let arg_addr = emu
        .maps
        .read_qword(argv_ptr + (optind as u64) * 8)
        .unwrap_or(0);
    let arg_str = emu.maps.read_string(arg_addr);

    if !arg_str.starts_with('-') || arg_str == "-" {
        emu.getopt_char_index = 0;
        log::info!(
            "{}** {} macOS API getopt_long() -> -1 (non-option: {}) {}",
            emu.colors.light_red,
            emu.pos,
            arg_str,
            emu.colors.nc
        );
        set_ret(emu, -1i64 as u64);
        return;
    }

    if arg_str == "--" {
        emu.getopt_char_index = 0;
        if optind_addr != 0 {
            emu.maps.write_dword(optind_addr, (optind + 1) as u32);
        }
        set_ret(emu, -1i64 as u64);
        return;
    }

    let flags = &arg_str[1..];
    let flags_chars: Vec<char> = flags.chars().collect();
    let pos = if char_idx > 0 && char_idx < flags_chars.len() {
        char_idx
    } else {
        0
    };
    let ch = flags_chars[pos];
    let is_last = pos + 1 >= flags_chars.len();

    let ch_pos = optstring.find(ch);
    match ch_pos {
        Some(opos) => {
            let needs_arg = optstring.as_bytes().get(opos + 1) == Some(&b':');
            if needs_arg || is_last {
                // Advance optind: done with this argv element
                let new_optind = optind + 1;
                if optind_addr != 0 {
                    emu.maps.write_dword(optind_addr, new_optind as u32);
                }
                emu.getopt_char_index = 0;

                if needs_arg {
                    if new_optind < argc {
                        let next_arg_addr = emu
                            .maps
                            .read_qword(argv_ptr + (new_optind as u64) * 8)
                            .unwrap_or(0);
                        if optarg_addr != 0 {
                            emu.maps.write_qword(optarg_addr, next_arg_addr);
                        }
                        if optind_addr != 0 {
                            emu.maps.write_dword(optind_addr, (new_optind + 1) as u32);
                        }
                    }
                } else if optarg_addr != 0 {
                    emu.maps.write_qword(optarg_addr, 0);
                }
            } else {
                // More flags remain in this argv element
                emu.getopt_char_index = pos + 1;
                if optarg_addr != 0 {
                    emu.maps.write_qword(optarg_addr, 0);
                }
            }
            log::info!(
                "{}** {} macOS API getopt_long() -> '{}' {}",
                emu.colors.light_red,
                emu.pos,
                ch,
                emu.colors.nc
            );
            set_ret(emu, ch as u64);
        }
        None => {
            if is_last {
                let new_optind = optind + 1;
                if optind_addr != 0 {
                    emu.maps.write_dword(optind_addr, new_optind as u32);
                }
                emu.getopt_char_index = 0;
            } else {
                emu.getopt_char_index = pos + 1;
            }
            log::info!(
                "{}** {} macOS API getopt_long() -> '?' (unknown '{}') {}",
                emu.colors.light_red,
                emu.pos,
                ch,
                emu.colors.nc
            );
            set_ret(emu, b'?' as u64);
        }
    }
}

fn api_signal(emu: &mut Emu) {
    let sig = arg(emu, 0);
    log::info!(
        "{}** {} macOS API signal({}) -> SIG_DFL {}",
        emu.colors.light_red,
        emu.pos,
        sig,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_kill(emu: &mut Emu) {
    let pid = arg(emu, 0);
    let sig = arg(emu, 1);
    log::info!(
        "{}** {} macOS API kill({}, {}) -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        pid,
        sig,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_getuid(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API getuid() -> 501 {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    set_ret(emu, 501);
}

fn api_getpid(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API getpid() -> 1234 {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    set_ret(emu, 1234);
}

fn api___error(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API __error() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    let errno_ptr = alloc_string(emu, "\0\0\0\0");
    set_ret(emu, errno_ptr);
}

fn api___stack_chk_fail(emu: &mut Emu) {
    log::error!(
        "{}** {} macOS API __stack_chk_fail() — stack smash detected {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.stop();
}

fn api___maskrune(emu: &mut Emu) {
    let _c = arg(emu, 0);
    let _mask = arg(emu, 1);
    set_ret(emu, 0);
}

fn api___tolower(emu: &mut Emu) {
    let c = arg(emu, 0) as u8;
    let lower = if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c
    };
    set_ret(emu, lower as u64);
}

fn api___assert_rtn(emu: &mut Emu) {
    let func_addr = arg(emu, 0);
    let file_addr = arg(emu, 1);
    let line = arg(emu, 2);
    let expr_addr = arg(emu, 3);
    let func = emu.maps.read_string(func_addr);
    let file = emu.maps.read_string(file_addr);
    let expr = emu.maps.read_string(expr_addr);
    log::error!(
        "{}** {} macOS API __assert_rtn(\"{}\", \"{}\", {}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        func,
        file,
        line,
        expr,
        emu.colors.nc
    );
    emu.stop();
}

fn api_err(emu: &mut Emu) {
    let eval = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    log::error!(
        "{}** {} macOS API err({}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        eval,
        fmt,
        emu.colors.nc
    );
    emu.stop();
}

fn api_errx(emu: &mut Emu) {
    let eval = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    log::error!(
        "{}** {} macOS API errx({}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        eval,
        fmt,
        emu.colors.nc
    );
    emu.stop();
}

fn api_warn(emu: &mut Emu) {
    let fmt_addr = arg(emu, 0);
    let fmt = emu.maps.read_string(fmt_addr);
    log::warn!(
        "{}** {} macOS API warn(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
}

fn api_warnx(emu: &mut Emu) {
    let fmt_addr = arg(emu, 0);
    let fmt = emu.maps.read_string(fmt_addr);
    log::warn!(
        "{}** {} macOS API warnx(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
}

fn api_strerror(emu: &mut Emu) {
    let errnum = arg(emu, 0);
    log::info!(
        "{}** {} macOS API strerror({}) {}",
        emu.colors.light_red,
        emu.pos,
        errnum,
        emu.colors.nc
    );
    let ret = alloc_string(emu, "Unknown error");
    set_ret(emu, ret);
}

fn api_fflush(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API fflush() -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // Drain any data that the inline putc macro wrote into the FILE buffer
    flush_file_buffer_to_stdout(emu);
    // Print and clear the buffered emulated output
    if !emu.emulated_stdout.is_empty() {
        let s = String::from_utf8_lossy(&emu.emulated_stdout).to_string();
        print!("{}", s);
        emu.emulated_stdout.clear();
    }
    set_ret(emu, 0);
}

fn flush_file_buffer_to_stdout(emu: &mut Emu) {
    if let Some(ref macho) = emu.macho64 {
        let stdoutp_addr = macho
            .addr_to_symbol
            .iter()
            .find(|(_, s)| s.as_str() == "___stdoutp")
            .map(|(a, _)| *a);
        if let Some(addr) = stdoutp_addr
            && let Some(fp) = emu.maps.read_qword(addr)
            && fp != 0
        {
            let p = emu.maps.read_qword(fp).unwrap_or(0);
            let base = emu.maps.read_qword(fp + 24).unwrap_or(0);
            let buf_size = emu.maps.read_dword(fp + 32).unwrap_or(0) as u64;
            if base != 0 && p > base {
                let count = p - base;
                for i in 0..count {
                    if let Some(b) = emu.maps.read_byte(base + i) {
                        emu.emulated_stdout.push(b);
                    }
                }
                emu.maps.write_qword(fp, base);
                emu.maps.write_dword(fp + 12, buf_size as u32);
            }
        }
    }
}

fn api___swbuf(emu: &mut Emu) {
    let c = arg(emu, 0) as u8;
    let fp = arg(emu, 1);
    // Drain any data in the FILE buffer, then write the new char
    flush_file_buffer_to_stdout(emu);
    if fp != 0 {
        let base = emu.maps.read_qword(fp + 24).unwrap_or(0);
        if base != 0 {
            emu.maps.write_byte(base, c);
            emu.maps.write_qword(fp, base + 1);
            let buf_size = emu.maps.read_dword(fp + 32).unwrap_or(0);
            emu.maps.write_dword(fp + 12, buf_size.saturating_sub(1));
        }
    }
    set_ret(emu, c as u64);
}

fn api_fputc(emu: &mut Emu) {
    let c = arg(emu, 0) as u8;
    log::info!(
        "{}** {} macOS API fputc('{}') {}",
        emu.colors.light_red,
        emu.pos,
        c as char,
        emu.colors.nc
    );
    emu.emulated_stdout.push(c);
    set_ret(emu, c as u64);
}

fn api_fputs(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API fputs(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );
    emu.emulated_stdout.extend_from_slice(s.as_bytes());
    set_ret(emu, 0);
}

fn api_fwrite(emu: &mut Emu) {
    let buf = arg(emu, 0);
    let size = arg(emu, 1);
    let nmemb = arg(emu, 2);
    let total = size.saturating_mul(nmemb);
    let mut data = Vec::with_capacity(total as usize);
    for i in 0..total {
        match emu.maps.read_byte(buf + i) {
            Some(b) => data.push(b),
            None => break,
        }
    }
    let s = String::from_utf8_lossy(&data);
    log::info!(
        "{}** {} macOS API fwrite(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );
    emu.emulated_stdout.extend_from_slice(&data);
    set_ret(emu, nmemb);
}

fn api_ferror(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API ferror() -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_strcoll(emu: &mut Emu) {
    let s1_addr = arg(emu, 0);
    let s2_addr = arg(emu, 1);
    let s1 = emu.maps.read_string(s1_addr);
    let s2 = emu.maps.read_string(s2_addr);
    let result = match s1.cmp(&s2) {
        std::cmp::Ordering::Less => -1i64 as u64,
        std::cmp::Ordering::Equal => 0u64,
        std::cmp::Ordering::Greater => 1u64,
    };
    set_ret(emu, result);
}

fn api_strtoul(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let _endptr = arg(emu, 1);
    let base = arg(emu, 2) as u32;
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API strtoul(\"{}\", _, {}) {}",
        emu.colors.light_red,
        emu.pos,
        s,
        base,
        emu.colors.nc
    );
    let val = u64::from_str_radix(s.trim(), if base == 0 { 10 } else { base });
    set_ret(emu, val.unwrap_or(0));
}

fn api_time(emu: &mut Emu) {
    let tloc = arg(emu, 0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    log::info!(
        "{}** {} macOS API time() -> {} {}",
        emu.colors.light_red,
        emu.pos,
        now,
        emu.colors.nc
    );
    if tloc != 0 {
        emu.maps.write_qword(tloc, now);
    }
    set_ret(emu, now);
}

fn api_localtime(emu: &mut Emu) {
    let _timep = arg(emu, 0);
    log::info!(
        "{}** {} macOS API localtime() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // Allocate a fake struct tm (at least 56 bytes on macOS)
    let tm = allocate_memory(emu, 64).expect("localtime: alloc");
    // tm_sec=0, tm_min=0, tm_hour=12, tm_mday=1, tm_mon=0, tm_year=125 (2025),
    // tm_wday=3, tm_yday=0, tm_isdst=0
    emu.maps.write_dword(tm, 0); // tm_sec
    emu.maps.write_dword(tm + 4, 0); // tm_min
    emu.maps.write_dword(tm + 8, 12); // tm_hour
    emu.maps.write_dword(tm + 12, 1); // tm_mday
    emu.maps.write_dword(tm + 16, 0); // tm_mon
    emu.maps.write_dword(tm + 20, 125); // tm_year (2025-1900)
    emu.maps.write_dword(tm + 24, 3); // tm_wday
    emu.maps.write_dword(tm + 28, 0); // tm_yday
    emu.maps.write_dword(tm + 32, 0); // tm_isdst
    set_ret(emu, tm);
}

fn api_strftime(emu: &mut Emu) {
    let dst = arg(emu, 0);
    let maxsize = arg(emu, 1);
    let fmt_addr = arg(emu, 2);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API strftime(_, {}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        maxsize,
        fmt,
        emu.colors.nc
    );
    let result = "Jan  1 12:00";
    let bytes = result.as_bytes();
    let write_len = std::cmp::min(bytes.len(), (maxsize.saturating_sub(1)) as usize);
    emu.maps.write_bytes(dst, &bytes[..write_len]);
    emu.maps.write_byte(dst + write_len as u64, 0);
    set_ret(emu, write_len as u64);
}

fn api_readlink(emu: &mut Emu) {
    let path_addr = arg(emu, 0);
    let path = emu.maps.read_string(path_addr);
    log::info!(
        "{}** {} macOS API readlink(\"{}\") -> -1 {}",
        emu.colors.light_red,
        emu.pos,
        path,
        emu.colors.nc
    );
    set_ret(emu, -1i64 as u64);
}

fn api_strmode(emu: &mut Emu) {
    let mode = arg(emu, 0) as u32;
    let buf = arg(emu, 1);
    log::info!(
        "{}** {} macOS API strmode(0o{:o}, 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        mode,
        buf,
        emu.colors.nc
    );
    let mut s = [b'-'; 12];
    // File type
    s[0] = match mode & 0o170000 {
        0o040000 => b'd',
        0o120000 => b'l',
        0o010000 => b'p',
        0o060000 => b'b',
        0o020000 => b'c',
        0o140000 => b's',
        _ => b'-',
    };
    // Owner
    if mode & 0o400 != 0 {
        s[1] = b'r';
    }
    if mode & 0o200 != 0 {
        s[2] = b'w';
    }
    if mode & 0o100 != 0 {
        s[3] = if mode & 0o4000 != 0 { b's' } else { b'x' };
    } else if mode & 0o4000 != 0 {
        s[3] = b'S';
    }
    // Group
    if mode & 0o040 != 0 {
        s[4] = b'r';
    }
    if mode & 0o020 != 0 {
        s[5] = b'w';
    }
    if mode & 0o010 != 0 {
        s[6] = if mode & 0o2000 != 0 { b's' } else { b'x' };
    } else if mode & 0o2000 != 0 {
        s[6] = b'S';
    }
    // Other
    if mode & 0o004 != 0 {
        s[7] = b'r';
    }
    if mode & 0o002 != 0 {
        s[8] = b'w';
    }
    if mode & 0o001 != 0 {
        s[9] = if mode & 0o1000 != 0 { b't' } else { b'x' };
    } else if mode & 0o1000 != 0 {
        s[9] = b'T';
    }
    s[10] = b' ';
    s[11] = 0;
    emu.maps.write_bytes(buf, &s);
}

fn api_user_from_uid(emu: &mut Emu) {
    let uid = arg(emu, 0);
    log::info!(
        "{}** {} macOS API user_from_uid({}) {}",
        emu.colors.light_red,
        emu.pos,
        uid,
        emu.colors.nc
    );
    let ret = alloc_string(emu, "user");
    set_ret(emu, ret);
}

fn api_group_from_gid(emu: &mut Emu) {
    let gid = arg(emu, 0);
    log::info!(
        "{}** {} macOS API group_from_gid({}) {}",
        emu.colors.light_red,
        emu.pos,
        gid,
        emu.colors.nc
    );
    let ret = alloc_string(emu, "staff");
    set_ret(emu, ret);
}

fn api_nl_langinfo(emu: &mut Emu) {
    let item = arg(emu, 0);
    log::info!(
        "{}** {} macOS API nl_langinfo({}) {}",
        emu.colors.light_red,
        emu.pos,
        item,
        emu.colors.nc
    );
    let ret = alloc_string(emu, "UTF-8");
    set_ret(emu, ret);
}

fn api_mbrtowc(emu: &mut Emu) {
    let _pwc = arg(emu, 0);
    let s = arg(emu, 1);
    let _n = arg(emu, 2);
    if s == 0 {
        set_ret(emu, 0);
        return;
    }
    let b = emu.maps.read_byte(s).unwrap_or(0);
    if _pwc != 0 {
        emu.maps.write_dword(_pwc, b as u32);
    }
    set_ret(emu, if b == 0 { 0 } else { 1 });
}

fn api_wcwidth(emu: &mut Emu) {
    let _wc = arg(emu, 0);
    set_ret(emu, 1);
}

fn api_getbsize(emu: &mut Emu) {
    let headerlenp = arg(emu, 0);
    let blocksizep = arg(emu, 1);
    log::info!(
        "{}** {} macOS API getbsize() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    if headerlenp != 0 {
        emu.maps.write_dword(headerlenp, 0);
    }
    if blocksizep != 0 {
        emu.maps.write_qword(blocksizep, 512);
    }
    let ret = alloc_string(emu, "512");
    set_ret(emu, ret);
}

fn api_fflagstostr(emu: &mut Emu) {
    let _flags = arg(emu, 0);
    let ret = alloc_string(emu, "");
    set_ret(emu, ret);
}

fn api_compat_mode(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API compat_mode() -> NULL {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    set_ret(emu, 0);
}

fn api_sysctlbyname(emu: &mut Emu) {
    let name_addr = arg(emu, 0);
    let name = emu.maps.read_string(name_addr);
    log::info!(
        "{}** {} macOS API sysctlbyname(\"{}\") -> -1 {}",
        emu.colors.light_red,
        emu.pos,
        name,
        emu.colors.nc
    );
    set_ret(emu, -1i64 as u64);
}

fn api_getxattr(emu: &mut Emu) {
    set_ret(emu, -1i64 as u64);
}

fn api_listxattr(emu: &mut Emu) {
    set_ret(emu, -1i64 as u64);
}

fn api_humanize_number(emu: &mut Emu) {
    set_ret(emu, -1i64 as u64);
}

fn api_strtonum(emu: &mut Emu) {
    let s_addr = arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    let val: i64 = s.trim().parse().unwrap_or(0);
    set_ret(emu, val as u64);
}

fn api_uuid_unparse_upper(emu: &mut Emu) {
    let _uu = arg(emu, 0);
    let out = arg(emu, 1);
    let fake = "00000000-0000-0000-0000-000000000000";
    let bytes = fake.as_bytes();
    emu.maps.write_bytes(out, bytes);
    emu.maps.write_byte(out + bytes.len() as u64, 0);
}

fn api_mbr_identifier_translate(emu: &mut Emu) {
    set_ret(emu, -1i64 as u64);
}

// ==================== FTS (filesystem traversal) ====================

fn api_fts_open(emu: &mut Emu) {
    let argv_addr = arg(emu, 0);
    let options = arg(emu, 1);

    // Read the path list from argv (array of char* pointers, NULL-terminated)
    let mut paths: Vec<String> = Vec::new();
    let mut ptr_addr = argv_addr;
    loop {
        let p = emu.maps.read_qword(ptr_addr).unwrap_or(0);
        if p == 0 {
            break;
        }
        paths.push(emu.maps.read_string(p));
        ptr_addr += 8;
    }
    if paths.is_empty() {
        paths.push(".".to_string());
    }

    log::info!(
        "{}** {} macOS API fts_open({:?}, options=0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        paths,
        options,
        emu.colors.nc
    );

    // Read the real host filesystem and collect entries
    let mut entries: Vec<FtsEntry> = Vec::new();
    for path in &paths {
        let p = std::path::Path::new(path);
        if let Ok(meta) = p.symlink_metadata() {
            entries.push(FtsEntry::from_metadata(path.clone(), &meta, true));
        }
        if p.is_dir()
            && let Ok(rd) = std::fs::read_dir(p)
        {
            for de in rd.flatten() {
                let name = de.file_name().to_string_lossy().to_string();
                let full = format!("{}/{}", path.trim_end_matches('/'), name);
                if let Ok(meta) = de.metadata() {
                    entries.push(FtsEntry::from_metadata(full, &meta, false));
                }
            }
        }
    }

    // Allocate a fake FTS handle (just a small block to serve as an opaque pointer)
    let handle = allocate_memory(emu, 64).expect("fts_open: alloc");

    // Create the root FTSENT upfront so children can reference it as fts_parent
    let root_entry = entries.iter().find(|e| e.is_root).cloned();
    let root_ftsent = match root_entry {
        Some(ref e) => write_ftsent(emu, e),
        None => 0,
    };

    // Store state in the emulator's auxiliary map
    let state = FtsState {
        entries,
        index: 0,
        last_ftsent: 0,
        root_ftsent,
        children_head: 0,
        root_returned: false,
        options,
    };
    emu.fts_handles.insert(handle, state);

    log::info!("  -> handle 0x{:x}", handle);
    set_ret(emu, handle);
}

fn api_fts_read(emu: &mut Emu) {
    let handle = arg(emu, 0);

    let state = match emu.fts_handles.get_mut(&handle) {
        Some(s) => s,
        None => {
            log::warn!("fts_read: invalid handle 0x{:x}", handle);
            set_ret(emu, 0);
            return;
        }
    };

    if state.root_returned {
        log::info!(
            "{}** {} macOS API fts_read(0x{:x}) -> NULL (end) {}",
            emu.colors.light_red,
            emu.pos,
            handle,
            emu.colors.nc
        );
        set_ret(emu, 0);
        return;
    }

    state.root_returned = true;

    // If children were already returned via fts_children, skip fts_read
    // to avoid ls displaying the same entries twice.
    if state.children_head != 0 {
        log::info!(
            "{}** {} macOS API fts_read(0x{:x}) -> NULL (children already displayed) {}",
            emu.colors.light_red,
            emu.pos,
            handle,
            emu.colors.nc
        );
        set_ret(emu, 0);
        return;
    }

    // Find the root entry
    let entry = match state.entries.iter().find(|e| e.is_root) {
        Some(e) => e.clone(),
        None => {
            set_ret(emu, 0);
            return;
        }
    };

    log::info!(
        "{}** {} macOS API fts_read(0x{:x}) -> \"{}\" {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        entry.path,
        emu.colors.nc
    );

    let root_addr = emu.fts_handles.get(&handle).unwrap().root_ftsent;
    let ftsent_addr = if root_addr != 0 {
        root_addr
    } else {
        write_ftsent(emu, &entry)
    };
    emu.fts_handles.get_mut(&handle).unwrap().last_ftsent = ftsent_addr;
    set_ret(emu, ftsent_addr);
}

fn api_fts_close(emu: &mut Emu) {
    let handle = arg(emu, 0);
    log::info!(
        "{}** {} macOS API fts_close(0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        emu.colors.nc
    );
    emu.fts_handles.remove(&handle);
    set_ret(emu, 0);
}

fn api_fts_set(emu: &mut Emu) {
    set_ret(emu, 0);
}

fn api_fts_children(emu: &mut Emu) {
    let handle = arg(emu, 0);
    let instr = arg(emu, 1);

    let state = match emu.fts_handles.get_mut(&handle) {
        Some(s) => s,
        None => {
            set_ret(emu, 0);
            return;
        }
    };

    // Already returned children once; subsequent calls return NULL
    if state.children_head != 0 {
        log::info!(
            "{}** {} macOS API fts_children(0x{:x}, instr=0x{:x}) -> NULL (already returned) {}",
            emu.colors.light_red,
            emu.pos,
            handle,
            instr,
            emu.colors.nc
        );
        set_ret(emu, 0);
        return;
    }

    let mut children: Vec<FtsEntry> = state
        .entries
        .iter()
        .filter(|e| !e.is_root)
        .cloned()
        .collect();

    // Mark all children as FTS_F so ls displays them all inline.
    // The stat struct (fts_statp) carries the real mode, so strmode()
    // will still show drwxr-xr-x for directories.
    for child in &mut children {
        child.fts_info = FTS_F;
    }

    if children.is_empty() {
        set_ret(emu, 0);
        return;
    }

    let root_ftsent = state.root_ftsent;

    // Build linked list of FTSENT structs via fts_link field
    let mut addrs: Vec<u64> = Vec::with_capacity(children.len());
    for child in &children {
        addrs.push(write_ftsent(emu, child));
    }
    for i in 0..addrs.len() {
        if root_ftsent != 0 {
            emu.maps
                .write_qword(addrs[i] + FTSENT_FTS_PARENT, root_ftsent);
        }
        if i + 1 < addrs.len() {
            emu.maps
                .write_qword(addrs[i] + FTSENT_FTS_LINK, addrs[i + 1]);
        }
    }

    let head = addrs[0];
    emu.fts_handles.get_mut(&handle).unwrap().children_head = head;

    log::info!(
        "{}** {} macOS API fts_children(0x{:x}, instr=0x{:x}) -> {} entries {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        instr,
        children.len(),
        emu.colors.nc
    );
    set_ret(emu, head);
}

// macOS FTSENT offsets (arm64, from <fts.h>):
const FTSENT_FTS_CYCLE: u64 = 0x00; // *fts_cycle
const FTSENT_FTS_PARENT: u64 = 0x08; // *fts_parent
const FTSENT_FTS_LINK: u64 = 0x10; // *fts_link
const FTSENT_FTS_NUMBER: u64 = 0x18; // fts_number (int64)
const FTSENT_FTS_POINTER: u64 = 0x20; // *fts_pointer
const FTSENT_FTS_ACCPATH: u64 = 0x28; // *fts_accpath
const FTSENT_FTS_PATH: u64 = 0x30; // *fts_path
const FTSENT_FTS_ERRNO: u64 = 0x38; // fts_errno (int)
const FTSENT_FTS_SYMFD: u64 = 0x3c; // fts_symfd (int)
const FTSENT_FTS_PATHLEN: u64 = 0x40; // fts_pathlen (u_short)
const FTSENT_FTS_NAMELEN: u64 = 0x42; // fts_namelen (u_short)
const FTSENT_FTS_INO: u64 = 0x48; // fts_ino (ino_t, u64)
const FTSENT_FTS_DEV: u64 = 0x50; // fts_dev (dev_t, i32)
const FTSENT_FTS_NLINK: u64 = 0x54; // fts_nlink (nlink_t, u16)
const FTSENT_FTS_LEVEL: u64 = 0x56; // fts_level (short)
const FTSENT_FTS_INFO: u64 = 0x58; // fts_info (u_short)
const FTSENT_FTS_FLAGS: u64 = 0x5a; // fts_flags (u_short)
const FTSENT_FTS_STATP: u64 = 0x60; // *fts_statp
const FTSENT_FTS_NAME: u64 = 0x68; // fts_name[1] (flexible array)

const FTS_D: u16 = 1; // directory (pre-order)
const FTS_F: u16 = 8; // regular file
const FTS_SL: u16 = 12; // symbolic link
const FTS_DP: u16 = 6; // directory (post-order) — we skip these
const FTS_NSOK: u16 = 11; // no stat requested (FTS_NOSTAT)
const FTS_NOSTAT: u64 = 0x08; // fts_open option: don't stat entries
const FTS_NAMEONLY: u64 = 0x100; // fts_children option: names only

fn write_ftsent(emu: &mut Emu, entry: &FtsEntry) -> u64 {
    let name_bytes = entry.name.as_bytes();
    let path_bytes = entry.path.as_bytes();
    let ftsent_size = FTSENT_FTS_NAME + (name_bytes.len() as u64) + 1;
    let ftsent = allocate_memory(emu, ftsent_size + 256).expect("ftsent: alloc");

    // Zero-fill
    for i in 0..ftsent_size {
        emu.maps.write_byte(ftsent + i, 0);
    }

    // Write name at the end (fts_name field)
    emu.maps.write_bytes(ftsent + FTSENT_FTS_NAME, name_bytes);
    emu.maps
        .write_byte(ftsent + FTSENT_FTS_NAME + name_bytes.len() as u64, 0);

    // Allocate and write path string
    let path_addr = allocate_memory(emu, (path_bytes.len() + 1) as u64).expect("path: alloc");
    emu.maps.write_bytes(path_addr, path_bytes);
    emu.maps.write_byte(path_addr + path_bytes.len() as u64, 0);

    // fts_accpath and fts_path both point to the path
    emu.maps.write_qword(ftsent + FTSENT_FTS_ACCPATH, path_addr);
    emu.maps.write_qword(ftsent + FTSENT_FTS_PATH, path_addr);

    // Lengths
    emu.maps
        .write_word(ftsent + FTSENT_FTS_PATHLEN, path_bytes.len() as u16);
    emu.maps
        .write_word(ftsent + FTSENT_FTS_NAMELEN, name_bytes.len() as u16);

    // File info
    emu.maps
        .write_word(ftsent + FTSENT_FTS_INFO, entry.fts_info);
    emu.maps.write_qword(ftsent + FTSENT_FTS_INO, entry.ino);
    emu.maps
        .write_dword(ftsent + FTSENT_FTS_DEV, entry.dev as u32);
    emu.maps
        .write_word(ftsent + FTSENT_FTS_NLINK, entry.nlink as u16);
    emu.maps
        .write_word(ftsent + FTSENT_FTS_LEVEL, if entry.is_root { 0 } else { 1 });

    // Write stat struct and set fts_statp
    let stat_addr = write_stat_struct(emu, &entry.stat);
    emu.maps.write_qword(ftsent + FTSENT_FTS_STATP, stat_addr);

    ftsent
}

// ==================== stat ====================

fn api_stat(emu: &mut Emu) {
    let path_addr = arg(emu, 0);
    let buf = arg(emu, 1);
    let path = emu.maps.read_string(path_addr);
    log::info!(
        "{}** {} macOS API stat(\"{}\", 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        path,
        buf,
        emu.colors.nc
    );
    match std::fs::metadata(&path) {
        Ok(meta) => {
            let st = StatData::from_metadata(&meta);
            write_stat_at(emu, buf, &st);
            set_ret(emu, 0);
        }
        Err(_) => set_ret(emu, -1i64 as u64),
    }
}

fn api_lstat(emu: &mut Emu) {
    let path_addr = arg(emu, 0);
    let buf = arg(emu, 1);
    let path = emu.maps.read_string(path_addr);
    log::info!(
        "{}** {} macOS API lstat(\"{}\", 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        path,
        buf,
        emu.colors.nc
    );
    match std::fs::symlink_metadata(&path) {
        Ok(meta) => {
            let st = StatData::from_metadata(&meta);
            write_stat_at(emu, buf, &st);
            set_ret(emu, 0);
        }
        Err(_) => set_ret(emu, -1i64 as u64),
    }
}

fn api_fstat(emu: &mut Emu) {
    let fd = arg(emu, 0);
    let buf = arg(emu, 1);
    log::info!(
        "{}** {} macOS API fstat({}, 0x{:x}) -> 0 {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buf,
        emu.colors.nc
    );
    let st = StatData::default();
    write_stat_at(emu, buf, &st);
    set_ret(emu, 0);
}

// ==================== Helpers ====================

fn alloc_string(emu: &mut Emu, s: &str) -> u64 {
    let len = s.len() as u64 + 1;
    let addr = allocate_memory(emu, len).expect("alloc_string: out of memory");
    emu.maps.write_bytes(addr, s.as_bytes());
    emu.maps.write_byte(addr + s.len() as u64, 0);
    addr
}

#[derive(Clone)]
struct FtsEntry {
    name: String,
    path: String,
    fts_info: u16,
    ino: u64,
    dev: u64,
    nlink: u64,
    is_root: bool,
    stat: StatData,
}

impl FtsEntry {
    fn from_metadata(path: String, meta: &std::fs::Metadata, is_root: bool) -> Self {
        let name = std::path::Path::new(&path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        let fts_info = if meta.is_dir() {
            FTS_D
        } else if meta.file_type().is_symlink() {
            FTS_SL
        } else {
            FTS_F
        };
        #[cfg(unix)]
        let (ino, dev, nlink) = {
            use std::os::unix::fs::MetadataExt;
            (meta.ino(), meta.dev(), meta.nlink())
        };
        #[cfg(not(unix))]
        let (ino, dev, nlink) = (0u64, 0u64, 1u64);
        FtsEntry {
            name,
            path,
            fts_info,
            ino,
            dev,
            nlink,
            is_root,
            stat: StatData::from_metadata(meta),
        }
    }
}

pub struct FtsState {
    entries: Vec<FtsEntry>,
    index: usize,
    last_ftsent: u64,
    root_ftsent: u64,
    children_head: u64,
    root_returned: bool,
    options: u64,
}

#[derive(Clone, Default)]
struct StatData {
    dev: u32,
    mode: u16,
    nlink: u16,
    ino: u64,
    uid: u32,
    gid: u32,
    rdev: u32,
    size: u64,
    blocks: u64,
    blksize: u32,
    atime: u64,
    mtime: u64,
    ctime: u64,
}

impl StatData {
    #[cfg(unix)]
    fn from_metadata(meta: &std::fs::Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        StatData {
            dev: meta.dev() as u32,
            mode: meta.mode() as u16,
            nlink: meta.nlink() as u16,
            ino: meta.ino(),
            uid: meta.uid(),
            gid: meta.gid(),
            rdev: meta.rdev() as u32,
            size: meta.size(),
            blocks: meta.blocks(),
            blksize: meta.blksize() as u32,
            atime: meta.atime() as u64,
            mtime: meta.mtime() as u64,
            ctime: meta.ctime() as u64,
        }
    }

    #[cfg(not(unix))]
    fn from_metadata(meta: &std::fs::Metadata) -> Self {
        StatData {
            dev: 0,
            mode: if meta.is_dir() { 0o40755 } else { 0o100644 },
            nlink: 1,
            ino: 0,
            uid: 501,
            gid: 20,
            rdev: 0,
            size: meta.len(),
            blocks: (meta.len() + 511) / 512,
            blksize: 4096,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }
}

// macOS arm64 struct stat layout (from <sys/stat.h>):
// off 0x00: st_dev (dev_t = i32)
// off 0x04: st_mode (mode_t = u16)
// off 0x06: st_nlink (nlink_t = u16)
// off 0x08: st_ino (ino_t = u64)
// off 0x10: st_uid (uid_t = u32)
// off 0x14: st_gid (gid_t = u32)
// off 0x18: st_rdev (dev_t = i32)
// off 0x20: st_atimespec (16 bytes: tv_sec i64 + tv_nsec i64)
// off 0x30: st_mtimespec (16 bytes)
// off 0x40: st_ctimespec (16 bytes)
// off 0x50: st_birthtimespec (16 bytes)
// off 0x60: st_size (off_t = i64)
// off 0x68: st_blocks (blkcnt_t = i64)
// off 0x70: st_blksize (blksize_t = i32)
// off 0x74: st_flags (u32)
// off 0x78: st_gen (u32)
// total ~0x90 bytes
const STAT_SIZE: u64 = 0x90;

fn write_stat_struct(emu: &mut Emu, st: &StatData) -> u64 {
    let addr = allocate_memory(emu, STAT_SIZE).expect("stat: alloc");
    write_stat_at(emu, addr, st);
    addr
}

fn write_stat_at(emu: &mut Emu, addr: u64, st: &StatData) {
    // Zero-fill first
    for i in 0..STAT_SIZE {
        emu.maps.write_byte(addr + i, 0);
    }
    emu.maps.write_dword(addr, st.dev);
    emu.maps.write_word(addr + 0x04, st.mode);
    emu.maps.write_word(addr + 0x06, st.nlink);
    emu.maps.write_qword(addr + 0x08, st.ino);
    emu.maps.write_dword(addr + 0x10, st.uid);
    emu.maps.write_dword(addr + 0x14, st.gid);
    emu.maps.write_dword(addr + 0x18, st.rdev);
    emu.maps.write_qword(addr + 0x20, st.atime); // st_atimespec.tv_sec
    emu.maps.write_qword(addr + 0x30, st.mtime); // st_mtimespec.tv_sec
    emu.maps.write_qword(addr + 0x40, st.ctime); // st_ctimespec.tv_sec
    emu.maps.write_qword(addr + 0x60, st.size);
    emu.maps.write_qword(addr + 0x68, st.blocks);
    emu.maps.write_dword(addr + 0x70, st.blksize);
}

/// Convert POSIX PROT_* flags to emulator Permission
fn prot_to_permission(prot: u64) -> Permission {
    let r = prot & 0x1 != 0; // PROT_READ
    let w = prot & 0x2 != 0; // PROT_WRITE
    let x = prot & 0x4 != 0; // PROT_EXEC
    match (r, w, x) {
        (true, true, true) => Permission::READ_WRITE_EXECUTE,
        (true, true, false) => Permission::READ_WRITE,
        (true, false, true) => Permission::READ_EXECUTE,
        _ => Permission::READ_WRITE, // default fallback
    }
}
