use crate::emu;
use crate::maps::mem64::Permission;

// Linux aarch64 syscall numbers (from asm-generic/unistd.h)
// Convention: x8 = syscall number, x0-x5 = args, x0 = return
const SYS_CLOSE: u64 = 57;
const SYS_READ: u64 = 63;
const SYS_WRITE: u64 = 64;
const SYS_OPENAT: u64 = 56;
const SYS_WRITEV: u64 = 66;
const SYS_MMAP: u64 = 222;
const SYS_MUNMAP: u64 = 215;
const SYS_MPROTECT: u64 = 226;
const SYS_BRK: u64 = 214;
const SYS_IOCTL: u64 = 29;
const SYS_EXIT: u64 = 93;
const SYS_EXIT_GROUP: u64 = 94;
const SYS_SET_TID_ADDRESS: u64 = 96;
const SYS_FUTEX: u64 = 98;
const SYS_CLOCK_GETTIME: u64 = 113;
const SYS_GETPID: u64 = 172;
const SYS_GETTID: u64 = 178;
const SYS_UNAME: u64 = 160;
const SYS_GETRANDOM: u64 = 278;
const SYS_FSTAT: u64 = 80;
const SYS_NEWFSTATAT: u64 = 79;
const SYS_RT_SIGACTION: u64 = 134;
const SYS_RT_SIGPROCMASK: u64 = 135;
const SYS_LSEEK: u64 = 62;
const SYS_FCNTL: u64 = 25;

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = emu.regs_aarch64().x[8];
    if !emu.call_syscall_hook(syscall_nr) {
        return;
    }

    match syscall_nr {
        SYS_EXIT | SYS_EXIT_GROUP => {
            let status = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall exit({}) {}",
                emu.colors.light_red,
                emu.pos,
                status,
                emu.colors.nc
            );
            emu.stop();
        }

        SYS_WRITE => {
            let fd = emu.regs_aarch64().x[0];
            let buf = emu.regs_aarch64().x[1];
            let count = emu.regs_aarch64().x[2];

            log::info!(
                "{}** {} linux aarch64 syscall write(fd={}, buf=0x{:x}, count={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                buf,
                count,
                emu.colors.nc
            );

            if fd == 1 || fd == 2 {
                let s = emu.maps.read_string(buf);
                log::info!("{}:  \"{}\"", if fd == 1 { "stdout" } else { "stderr" }, s);
            }

            emu.regs_aarch64_mut().x[0] = count;
        }

        SYS_READ => {
            let fd = emu.regs_aarch64().x[0];
            let buf = emu.regs_aarch64().x[1];
            let count = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall read(fd={}, buf=0x{:x}, count={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                buf,
                count,
                emu.colors.nc
            );
            // Stub: return 0 (EOF) -- no real file backing
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_OPENAT => {
            let dirfd = emu.regs_aarch64().x[0] as i64;
            let path_addr = emu.regs_aarch64().x[1];
            let flags = emu.regs_aarch64().x[2];
            let path = emu.maps.read_string(path_addr);
            log::info!(
                "{}** {} linux aarch64 syscall openat(dirfd={}, \"{}\", 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                dirfd,
                path,
                flags,
                emu.colors.nc
            );
            // Return fake fd 3
            emu.regs_aarch64_mut().x[0] = 3;
        }

        SYS_CLOSE => {
            let fd = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall close(fd={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_LSEEK => {
            let fd = emu.regs_aarch64().x[0];
            let offset = emu.regs_aarch64().x[1] as i64;
            let whence = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall lseek(fd={}, offset={}, whence={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                offset,
                whence,
                emu.colors.nc
            );
            // Stub: return 0 (beginning of file)
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_FCNTL => {
            let fd = emu.regs_aarch64().x[0];
            let cmd = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall fcntl(fd={}, cmd={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                cmd,
                emu.colors.nc
            );
            // Stub: return 0
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_WRITEV => {
            let fd = emu.regs_aarch64().x[0];
            let iov_addr = emu.regs_aarch64().x[1];
            let iovcnt = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall writev(fd={}, iov=0x{:x}, iovcnt={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                iov_addr,
                iovcnt,
                emu.colors.nc
            );
            // Walk the iovec array: each entry is (base: u64, len: u64)
            let mut total: u64 = 0;
            for i in 0..iovcnt {
                let entry = iov_addr + i * 16;
                let base = emu.maps.read_qword(entry).unwrap_or(0);
                let len = emu.maps.read_qword(entry + 8).unwrap_or(0);
                if (fd == 1 || fd == 2) && base != 0 && len > 0 {
                    let s = emu.maps.read_string(base);
                    log::info!("{}:  \"{}\"", if fd == 1 { "stdout" } else { "stderr" }, s);
                }
                total += len;
            }
            emu.regs_aarch64_mut().x[0] = total;
        }

        SYS_BRK => {
            let addr = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall brk(0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                emu.colors.nc
            );
            if addr == 0 {
                // Query: return current program break
                // If heap_addr is not set yet, allocate an initial heap region
                if emu.heap_addr == 0 {
                    let initial_heap_sz: u64 = 0x100000; // 1MB initial heap
                    let base = emu
                        .maps
                        .alloc(initial_heap_sz)
                        .expect("linux aarch64 brk: cannot allocate initial heap");
                    emu.maps
                        .create_map(".heap", base, initial_heap_sz, Permission::READ_WRITE)
                        .expect("linux aarch64 brk: cannot create heap map");
                    emu.heap_addr = base + initial_heap_sz;
                    log::info!(
                        "  brk: initial heap at 0x{:x}, break=0x{:x}",
                        base,
                        emu.heap_addr
                    );
                }
                emu.regs_aarch64_mut().x[0] = emu.heap_addr;
            } else if addr > emu.heap_addr && emu.heap_addr != 0 {
                // Extend the break: allocate additional memory
                let extend_sz = addr - emu.heap_addr;
                let alloc_sz = (extend_sz + 0xFFF) & !0xFFF; // page-align
                if let Some(base) = emu.maps.alloc(alloc_sz) {
                    let _ = emu.maps.create_map(
                        &format!("brk_ext_{:x}", base),
                        base,
                        alloc_sz,
                        Permission::READ_WRITE,
                    );
                    emu.heap_addr = addr;
                    log::info!("  brk: extended to 0x{:x}", addr);
                }
                emu.regs_aarch64_mut().x[0] = addr;
            } else {
                // addr <= current break or heap_addr not set: just accept it
                if emu.heap_addr == 0 {
                    emu.heap_addr = addr;
                }
                emu.regs_aarch64_mut().x[0] = addr;
            }
        }

        SYS_MMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            let flags = emu.regs_aarch64().x[3];
            let fd = emu.regs_aarch64().x[4];
            let off = emu.regs_aarch64().x[5];
            log::info!(
                "{}** {} linux aarch64 syscall mmap(0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, {}, 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                len,
                prot,
                flags,
                fd as i64,
                off,
                emu.colors.nc
            );
            if len == 0 {
                // MAP_FAILED = -1
                emu.regs_aarch64_mut().x[0] = (-1i64) as u64;
                return;
            }
            let permission = prot_to_permission(prot);
            let base = emu
                .maps
                .alloc(len)
                .expect("linux aarch64 mmap: out of memory");
            emu.maps
                .create_map(&format!("mmap_{:x}", base), base, len, permission)
                .expect("linux aarch64 mmap: cannot create map");
            // zero-fill
            for i in 0..std::cmp::min(len, 0x10000) {
                emu.maps.write_byte(base + i, 0);
            }
            log::info!("  mmap -> 0x{:x}", base);
            emu.regs_aarch64_mut().x[0] = base;
        }

        SYS_MUNMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall munmap(0x{:x}, 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                len,
                emu.colors.nc
            );
            // Stub: return success
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_MPROTECT => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall mprotect(0x{:x}, 0x{:x}, 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                len,
                prot,
                emu.colors.nc
            );
            // Stub: return success
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_IOCTL => {
            let fd = emu.regs_aarch64().x[0];
            let request = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall ioctl(fd={}, 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                request,
                emu.colors.nc
            );
            // Stub: return 0 (success)
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_FSTAT => {
            let fd = emu.regs_aarch64().x[0];
            let statbuf = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall fstat(fd={}, buf=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                statbuf,
                emu.colors.nc
            );
            // Zero out the stat buffer (144 bytes on aarch64 linux)
            for i in 0..144u64 {
                emu.maps.write_byte(statbuf + i, 0);
            }
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_NEWFSTATAT => {
            let dirfd = emu.regs_aarch64().x[0] as i64;
            let path_addr = emu.regs_aarch64().x[1];
            let statbuf = emu.regs_aarch64().x[2];
            let flags = emu.regs_aarch64().x[3];
            let path = emu.maps.read_string(path_addr);
            log::info!(
                "{}** {} linux aarch64 syscall newfstatat(dirfd={}, \"{}\", buf=0x{:x}, flags=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                dirfd,
                path,
                statbuf,
                flags,
                emu.colors.nc
            );
            // Zero out the stat buffer
            for i in 0..144u64 {
                emu.maps.write_byte(statbuf + i, 0);
            }
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_SET_TID_ADDRESS => {
            let tidptr = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall set_tid_address(tidptr=0x{:x}) => 1 {}",
                emu.colors.light_red,
                emu.pos,
                tidptr,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1; // fake tid
        }

        SYS_FUTEX => {
            let uaddr = emu.regs_aarch64().x[0];
            let op = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall futex(0x{:x}, op={}) => 0 {}",
                emu.colors.light_red,
                emu.pos,
                uaddr,
                op,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_GETPID => {
            log::info!(
                "{}** {} linux aarch64 syscall getpid() => 1000 {}",
                emu.colors.light_red,
                emu.pos,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1000;
        }

        SYS_GETTID => {
            log::info!(
                "{}** {} linux aarch64 syscall gettid() => 1000 {}",
                emu.colors.light_red,
                emu.pos,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1000;
        }

        SYS_RT_SIGACTION => {
            let signum = emu.regs_aarch64().x[0];
            let act = emu.regs_aarch64().x[1];
            let oldact = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall rt_sigaction(sig={}, act=0x{:x}, oldact=0x{:x}) => 0 {}",
                emu.colors.light_red,
                emu.pos,
                signum,
                act,
                oldact,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_RT_SIGPROCMASK => {
            let how = emu.regs_aarch64().x[0];
            let set = emu.regs_aarch64().x[1];
            let oldset = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall rt_sigprocmask(how={}, set=0x{:x}, oldset=0x{:x}) => 0 {}",
                emu.colors.light_red,
                emu.pos,
                how,
                set,
                oldset,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_CLOCK_GETTIME => {
            let clockid = emu.regs_aarch64().x[0];
            let tp = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall clock_gettime(clockid={}, tp=0x{:x}) => 0 {}",
                emu.colors.light_red,
                emu.pos,
                clockid,
                tp,
                emu.colors.nc
            );
            // Write fake time: tv_sec=1000, tv_nsec=0
            if tp != 0 {
                emu.maps.write_qword(tp, 1000);
                emu.maps.write_qword(tp + 8, 0);
            }
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_GETRANDOM => {
            let buf = emu.regs_aarch64().x[0];
            let count = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall getrandom(0x{:x}, {}) {}",
                emu.colors.light_red,
                emu.pos,
                buf,
                count,
                emu.colors.nc
            );
            // Write zeros (not random, but functional)
            for i in 0..count {
                emu.maps.write_byte(buf + i, 0);
            }
            emu.regs_aarch64_mut().x[0] = count;
        }

        SYS_UNAME => {
            let buf = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall uname(buf=0x{:x}) => 0 {}",
                emu.colors.light_red,
                emu.pos,
                buf,
                emu.colors.nc
            );
            if buf != 0 && emu.maps.is_valid_ptr(buf) {
                // utsname struct: 6 fields of 65 bytes each = 390 bytes
                // sysname, nodename, release, version, machine, domainname
                let fields: [&[u8]; 6] = [
                    b"Linux", b"mwemu", b"5.15.0", b"#1 SMP", b"aarch64", b"(none)",
                ];
                for (i, field) in fields.iter().enumerate() {
                    let offset = buf + (i as u64) * 65;
                    // zero out the 65-byte field first
                    for j in 0..65u64 {
                        emu.maps.write_byte(offset + j, 0);
                    }
                    // write the string bytes
                    for (j, &byte) in field.iter().enumerate() {
                        emu.maps.write_byte(offset + j as u64, byte);
                    }
                }
            }
            emu.regs_aarch64_mut().x[0] = 0;
        }

        _ => {
            log::warn!(
                "{}** {} linux aarch64 unimplemented syscall {} (x0=0x{:x}, x1=0x{:x}, x2=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                syscall_nr,
                emu.regs_aarch64().x[0],
                emu.regs_aarch64().x[1],
                emu.regs_aarch64().x[2],
                emu.colors.nc
            );
        }
    }
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
