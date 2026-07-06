use crate::emu;

// macOS BSD syscall numbers (aarch64: x16 = syscall number, x0-x5 = args, x0 = return)
const SYS_EXIT: u64 = 1;
const SYS_FORK: u64 = 2;
const SYS_READ: u64 = 3;
const SYS_WRITE: u64 = 4;
const SYS_OPEN: u64 = 5;
const SYS_CLOSE: u64 = 6;
const SYS_MPROTECT: u64 = 74;
const SYS_MUNMAP: u64 = 73;
const SYS_MMAP: u64 = 197;
const SYS_IOCTL: u64 = 54;
const SYS_ISSETUGID: u64 = 327;

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = emu.regs_aarch64().x[16];
    if !emu.call_syscall_hook(syscall_nr) {
        return;
    }

    match syscall_nr {
        SYS_EXIT => {
            let status = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} macos syscall exit({}) {}",
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
                "{}** {} macos syscall write(fd={}, buf=0x{:x}, count={}) {}",
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

            // Return bytes written
            emu.regs_aarch64_mut().x[0] = count;
        }

        SYS_READ => {
            let fd = emu.regs_aarch64().x[0];
            let buf = emu.regs_aarch64().x[1];
            let count = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} macos syscall read(fd={}, buf=0x{:x}, count={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                buf,
                count,
                emu.colors.nc
            );
            todo!("macos read syscall");
        }

        SYS_OPEN => {
            let path_addr = emu.regs_aarch64().x[0];
            let flags = emu.regs_aarch64().x[1];
            let path = emu.maps.read_string(path_addr);
            log::info!(
                "{}** {} macos syscall open(\"{}\", 0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                path,
                flags,
                emu.colors.nc
            );
            todo!("macos open syscall");
        }

        SYS_CLOSE => {
            let fd = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} macos syscall close(fd={}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                emu.colors.nc
            );
            todo!("macos close syscall");
        }

        SYS_FORK => {
            log::info!(
                "{}** {} macos syscall fork() {}",
                emu.colors.light_red,
                emu.pos,
                emu.colors.nc
            );
            todo!("macos fork syscall");
        }

        SYS_MMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            let flags = emu.regs_aarch64().x[3];
            let fd = emu.regs_aarch64().x[4];
            let off = emu.regs_aarch64().x[5];
            log::info!(
                "{}** {} macos syscall mmap(addr=0x{:x}, len=0x{:x}, prot=0x{:x}, flags=0x{:x}, fd={}, off=0x{:x}) {}",
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
            todo!("macos mmap syscall");
        }

        SYS_MUNMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} macos syscall munmap(addr=0x{:x}, len=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                len,
                emu.colors.nc
            );
            todo!("macos munmap syscall");
        }

        SYS_MPROTECT => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} macos syscall mprotect(addr=0x{:x}, len=0x{:x}, prot=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                addr,
                len,
                prot,
                emu.colors.nc
            );
            todo!("macos mprotect syscall");
        }

        SYS_IOCTL => {
            let fd = emu.regs_aarch64().x[0];
            let request = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} macos syscall ioctl(fd={}, request=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                fd,
                request,
                emu.colors.nc
            );
            todo!("macos ioctl syscall");
        }

        SYS_ISSETUGID => {
            log::info!(
                "{}** {} macos syscall issetugid() => 0 {}",
                emu.colors.light_red,
                emu.pos,
                emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        _ => {
            log::warn!(
                "{}** {} macos unimplemented syscall {} (x0=0x{:x}, x1=0x{:x}, x2=0x{:x}) {}",
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
