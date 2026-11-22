use crate::api::abi::set_integer_return;
use crate::arch::Arch;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::ENOSYS;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::time::{Timespec, clock_gettime};
use crate::syscall::linux::{self, io};

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = emu.regs_aarch64().x[8];
    if !emu.call_syscall_hook(syscall_nr) {
        return;
    }

    match syscall_nr {
        NRA64_EXIT | NRA64_EXIT_GROUP => {
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

        NRA64_WRITE => io::write(emu, Arch::Aarch64),

        NRA64_READ => io::read(emu, Arch::Aarch64),

        NRA64_OPENAT => {
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

        NRA64_CLOSE => io::close(emu, Arch::Aarch64),
        NRA64_EVENTFD2 => io::eventfd(emu, Arch::Aarch64, true),
        NRA64_PIPE2 => io::pipe(emu, Arch::Aarch64, true),

        NRA64_LSEEK => {
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

        NRA64_FCNTL => io::fcntl(emu, Arch::Aarch64),

        NRA64_WRITEV => io::writev(emu, Arch::Aarch64),

        NRA64_BRK => {
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
                    let Some(base) = emu.maps.alloc(initial_heap_sz) else {
                        log::error!("linux aarch64 brk: cannot allocate initial heap");
                        emu.regs_aarch64_mut().x[0] = 0;
                        return;
                    };
                    if emu
                        .maps
                        .create_map(".heap", base, initial_heap_sz, Permission::READ_WRITE)
                        .is_err()
                    {
                        log::error!("linux aarch64 brk: cannot create heap map");
                        emu.regs_aarch64_mut().x[0] = 0;
                        return;
                    }
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

        NRA64_MMAP => {
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
            let Some(base) = emu.maps.alloc(len) else {
                log::error!("linux aarch64 mmap: out of memory");
                emu.regs_aarch64_mut().x[0] = (-1i64) as u64;
                return;
            };
            if emu
                .maps
                .create_map(&format!("mmap_{:x}", base), base, len, permission)
                .is_err()
            {
                log::error!("linux aarch64 mmap: cannot create map");
                emu.regs_aarch64_mut().x[0] = (-1i64) as u64;
                return;
            }
            // zero-fill
            for i in 0..std::cmp::min(len, 0x10000) {
                emu.maps.write_byte(base + i, 0);
            }
            log::info!("  mmap -> 0x{:x}", base);
            emu.regs_aarch64_mut().x[0] = base;
        }

        NRA64_MUNMAP => {
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

        NRA64_MPROTECT => {
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

        NRA64_IOCTL => {
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

        NRA64_FSTAT => {
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

        NRA64_NEWFSTATAT => {
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

        NRA64_SET_TID_ADDRESS => linux::set_tid_address(emu, Arch::Aarch64),

        NRA64_FUTEX => {
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

        NRA64_GETUID | NRA64_GETEUID | NRA64_GETGID | NRA64_GETEGID => linux::get_identity(emu),
        NRA64_GETRESUID | NRA64_GETRESGID => linux::get_res_ids(emu, Arch::Aarch64, false),
        NRA64_GETGROUPS => linux::get_groups(emu, Arch::Aarch64, false),
        NRA64_GETPID => linux::getpid(emu),
        NRA64_GETPPID => linux::getppid(emu),

        NRA64_GETTID => linux::gettid(emu),

        NRA64_RT_SIGACTION => {
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

        NRA64_RT_SIGPROCMASK => {
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

        NRA64_CLOCK_GETTIME => clock_gettime(emu, Arch::Aarch64, Timespec::Time64),

        NRA64_GETRANDOM => linux::getrandom(emu, Arch::Aarch64),
        NRA64_UNAME => linux::uname(emu, Arch::Aarch64),
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
            set_integer_return(emu, ENOSYS);
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
