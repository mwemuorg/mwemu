use crate::emu::Emu;
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
            _ => panic!("arg index {} not supported for x86_64 SysV ABI", idx),
        }
    }
}

/// Set return value, arch-agnostic.
/// AArch64: x0, x86_64: rax
fn set_ret(emu: &mut Emu, val: u64) {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().x[0] = val;
    } else {
        emu.regs_mut().rax = val;
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
        _ => {
            log::warn!("libsystem: unimplemented API {}", symbol);
            todo!("libSystem API: {}", symbol);
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let fmt_addr = arg(emu, 0);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API printf(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
    set_ret(emu, fmt.len() as u64);
}

fn api_fprintf(emu: &mut Emu) {
    let _stream = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API fprintf(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
    set_ret(emu, fmt.len() as u64);
}

fn api_sprintf(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let fmt_addr = arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API sprintf(0x{:x}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        fmt,
        emu.colors.nc
    );
    // Write the format string as-is (no vararg substitution)
    let bytes = fmt.as_bytes();
    emu.maps.write_bytes(dst_addr, bytes);
    emu.maps.write_byte(dst_addr + bytes.len() as u64, 0);
    set_ret(emu, bytes.len() as u64);
}

fn api_snprintf(emu: &mut Emu) {
    let dst_addr = arg(emu, 0);
    let size = arg(emu, 1);
    let fmt_addr = arg(emu, 2);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API snprintf(0x{:x}, {}, \"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        dst_addr,
        size,
        fmt,
        emu.colors.nc
    );
    // Write the format string truncated to size-1 (no vararg substitution)
    let bytes = fmt.as_bytes();
    if size > 0 {
        let write_len = std::cmp::min(bytes.len(), (size - 1) as usize);
        emu.maps.write_bytes(dst_addr, &bytes[..write_len]);
        emu.maps.write_byte(dst_addr + write_len as u64, 0);
    }
    set_ret(emu, bytes.len() as u64);
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
    set_ret(emu, 0);
}

fn api_putchar(emu: &mut Emu) {
    let c = arg(emu, 0) as u8 as char;
    log::info!(
        "{}** {} macOS API putchar('{}') {}",
        emu.colors.light_red,
        emu.pos,
        c,
        emu.colors.nc
    );
    set_ret(emu, c as u64);
}

fn api_exit(emu: &mut Emu) {
    let status = arg(emu, 0);
    log::info!(
        "{}** {} macOS API exit({}) {}",
        emu.colors.light_red,
        emu.pos,
        status,
        emu.colors.nc
    );
    emu.stop();
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
        let base = emu.maps.alloc(size).expect("macOS malloc: out of memory");
        emu.maps
            .create_map(
                &format!("alloc_{:x}", base),
                base,
                size,
                Permission::READ_WRITE,
            )
            .expect("macOS malloc: cannot create map");
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
        let base = emu.maps.alloc(total).expect("macOS calloc: out of memory");
        emu.maps
            .create_map(
                &format!("alloc_{:x}", base),
                base,
                total,
                Permission::READ_WRITE,
            )
            .expect("macOS calloc: cannot create map");
        // zero-fill (calloc contract)
        for i in 0..total {
            emu.maps.write_byte(base + i, 0);
        }
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
        // realloc(ptr, 0) acts like free
        set_ret(emu, 0);
        return;
    }
    // Allocate new block
    let base = emu.maps.alloc(size).expect("macOS realloc: out of memory");
    emu.maps
        .create_map(
            &format!("alloc_{:x}", base),
            base,
            size,
            Permission::READ_WRITE,
        )
        .expect("macOS realloc: cannot create map");
    // Copy old data if ptr != NULL
    if ptr != 0 {
        // Copy min(old_size, new_size) bytes; we don't track old size precisely,
        // so copy up to new size, byte by byte, stopping if read fails.
        for i in 0..size {
            match emu.maps.read_byte(ptr + i) {
                Some(b) => emu.maps.write_byte(base + i, b),
                None => break,
            };
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
    // no-op: we don't reclaim memory in the emulator
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
    let base = emu.maps.alloc(len).expect("macOS strdup: out of memory");
    emu.maps
        .create_map(
            &format!("alloc_{:x}", base),
            base,
            len,
            Permission::READ_WRITE,
        )
        .expect("macOS strdup: cannot create map");
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
