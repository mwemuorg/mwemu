use crate::api::abi::set_integer_return;
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::errors::ENOSYS;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::files;
use crate::syscall::linux::time::{Timespec, clock_gettime, gettimeofday};
use crate::syscall::linux::{self, io};

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = emu.regs_aarch64().x[8];
    if !emu.call_syscall_hook(syscall_nr) {
        return;
    }
    if files::dispatch(emu, Arch::Aarch64, syscall_nr) {
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

        NRA64_BRK => linux::mappings::brk64(emu, Arch::Aarch64),
        NRA64_MMAP => linux::mappings::mmap(emu, Arch::Aarch64),
        NRA64_MSYNC => linux::mappings::msync(emu, Arch::Aarch64),
        NRA64_MUNMAP => linux::mappings::munmap(emu, Arch::Aarch64),
        NRA64_MPROTECT => linux::mappings::mprotect(emu, Arch::Aarch64),

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

        NRA64_GETTIMEOFDAY => gettimeofday(emu, Arch::Aarch64, Timespec::Time64),
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
