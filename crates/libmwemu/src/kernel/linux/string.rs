//! `lib/string.c` and the `kstrto*` / `*printf` helpers.
//!
//! Kernel modules do not link a libc: `memcpy`, `strlen` and friends are
//! ordinary exported kernel symbols, so they arrive here as imports. Routing
//! them through the guard matters as much as implementing them — a `memcpy()`
//! into a freed object is a use-after-free that no instruction-level check
//! would ever see, because the copy happens inside the kernel, not in the
//! driver's own code.

use crate::emu::Emu;
use crate::kernel::linux::printk;

/// Copy `len` bytes, reporting either end that lands in a freed or
/// out-of-bounds chunk.
fn checked_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) {
    if len == 0 {
        return;
    }
    let rip = emu.pc();
    emu.kernel_guard_access(rip, src, len as u32, false);
    emu.kernel_guard_access(rip, src + len - 1, 1, false);
    emu.kernel_guard_access(rip, dst, len as u32, true);
    emu.kernel_guard_access(rip, dst + len - 1, 1, true);

    let mut buf = vec![0u8; len as usize];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
    }
    emu.maps.write_bytes(dst, &buf);
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        "memcpy" | "__memcpy" | "memmove" | "__memmove" | "memcpy_toio" | "memcpy_fromio" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            checked_copy(emu, dst, src, len);
            emu.set_kernel_ret(dst);
        }
        "memset" | "__memset" | "memset_io" => {
            let dst = emu.kernel_arg(0);
            let byte = emu.kernel_arg(1) as u8;
            let len = emu.kernel_arg(2);
            if len > 0 {
                let rip = emu.pc();
                emu.kernel_guard_access(rip, dst, len as u32, true);
                emu.kernel_guard_access(rip, dst + len - 1, 1, true);
                emu.maps.write_bytes(dst, &vec![byte; len as usize]);
            }
            emu.set_kernel_ret(dst);
        }
        "memcmp" | "bcmp" => {
            let a = emu.kernel_arg(0);
            let b = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, a, len as u32, false);
            emu.kernel_guard_access(rip, b, len as u32, false);
            let mut result: i64 = 0;
            for i in 0..len {
                let x = emu.maps.read_byte(a + i).unwrap_or(0) as i64;
                let y = emu.maps.read_byte(b + i).unwrap_or(0) as i64;
                if x != y {
                    result = x - y;
                    break;
                }
            }
            emu.set_kernel_ret(result as u64);
        }
        "strlen" => {
            let p = emu.kernel_arg(0);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, p, 1, false);
            let n = emu.maps.read_string(p).len() as u64;
            emu.set_kernel_ret(n);
        }
        "strnlen" => {
            let p = emu.kernel_arg(0);
            let max = emu.kernel_arg(1);
            let n = (emu.maps.read_string(p).len() as u64).min(max);
            emu.set_kernel_ret(n);
        }
        "strcmp" | "strncmp" | "strcasecmp" | "strncasecmp" => {
            let a = emu.maps.read_string(emu.kernel_arg(0));
            let b = emu.maps.read_string(emu.kernel_arg(1));
            let (a, b) = if symbol.starts_with("strn") {
                let n = emu.kernel_arg(2) as usize;
                (
                    a.chars().take(n).collect::<String>(),
                    b.chars().take(n).collect::<String>(),
                )
            } else {
                (a, b)
            };
            let (a, b) = if symbol.contains("case") {
                (a.to_lowercase(), b.to_lowercase())
            } else {
                (a, b)
            };
            let r: i64 = match a.cmp(&b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            emu.set_kernel_ret(r as u64);
        }
        "strcpy" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let s = emu.maps.read_string(src);
            checked_copy(emu, dst, src, s.len() as u64 + 1);
            emu.set_kernel_ret(dst);
        }
        "strncpy" | "strscpy" | "sized_strscpy" | "strlcpy" | "strscpy_pad" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let max = emu.kernel_arg(2);
            let mut s = emu.maps.read_string(src);
            s.truncate(max.saturating_sub(1) as usize);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
            emu.maps.write_string(dst, &s);
            // strscpy returns the length copied, strncpy/strlcpy return dest.
            let ret = if symbol.contains("strscpy") {
                s.len() as u64
            } else {
                dst
            };
            emu.set_kernel_ret(ret);
        }
        "strcat" | "strncat" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let existing = emu.maps.read_string(dst);
            let s = emu.maps.read_string(src);
            let end = dst + existing.len() as u64;
            let rip = emu.pc();
            emu.kernel_guard_access(rip, end, s.len() as u32 + 1, true);
            emu.maps.write_string(end, &s);
            emu.set_kernel_ret(dst);
        }
        "strchr" | "strrchr" => {
            let p = emu.kernel_arg(0);
            let c = emu.kernel_arg(1) as u8 as char;
            let s = emu.maps.read_string(p);
            let found = if symbol == "strchr" {
                s.find(c)
            } else {
                s.rfind(c)
            };
            emu.set_kernel_ret(found.map(|i| p + i as u64).unwrap_or(0));
        }
        "strstr" => {
            let p = emu.kernel_arg(0);
            let hay = emu.maps.read_string(p);
            let needle = emu.maps.read_string(emu.kernel_arg(1));
            emu.set_kernel_ret(hay.find(&needle).map(|i| p + i as u64).unwrap_or(0));
        }

        // --- formatted output ------------------------------------------------
        "snprintf" | "scnprintf" => {
            let dst = emu.kernel_arg(0);
            let max = emu.kernel_arg(1);
            let fmt = emu.kernel_arg(2);
            let mut s = printk::format(emu, fmt, 3);
            s.truncate(max.saturating_sub(1) as usize);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
            emu.maps.write_string(dst, &s);
            emu.set_kernel_ret(s.len() as u64);
        }
        "sprintf" => {
            let dst = emu.kernel_arg(0);
            let fmt = emu.kernel_arg(1);
            let s = printk::format(emu, fmt, 2);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, dst, s.len() as u32 + 1, true);
            emu.maps.write_string(dst, &s);
            emu.set_kernel_ret(s.len() as u64);
        }

        // --- string to number -------------------------------------------------
        "simple_strtoul" | "simple_strtol" | "simple_strtoull" => {
            let s = emu.maps.read_string(emu.kernel_arg(0));
            let base = emu.kernel_arg(2) as u32;
            let base = if base == 0 { 10 } else { base };
            let trimmed: String = s
                .chars()
                .take_while(|c| c.is_digit(base) || *c == '-')
                .collect();
            emu.set_kernel_ret(i64::from_str_radix(&trimmed, base).unwrap_or(0) as u64);
        }
        "kstrtoint" | "kstrtol" | "kstrtoul" | "kstrtouint" | "kstrtou32" | "kstrtou64"
        | "kstrtoll" | "kstrtoull" => {
            let s = emu.maps.read_string(emu.kernel_arg(0)).trim().to_string();
            let base = emu.kernel_arg(1) as u32;
            let base = if base == 0 { 10 } else { base };
            let out = emu.kernel_arg(2);
            match i64::from_str_radix(&s, base) {
                Ok(v) => {
                    if symbol.ends_with("int") || symbol.ends_with("u32") {
                        emu.maps.write_dword(out, v as u32);
                    } else {
                        emu.maps.write_qword(out, v as u64);
                    }
                    emu.set_kernel_ret(0);
                }
                // -EINVAL
                Err(_) => emu.set_kernel_ret((-22i64) as u64),
            }
        }

        _ => return false,
    }
    true
}
