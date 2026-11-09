use crate::emu;

// macOS BSD syscall numbers (x86_64: RAX = 0x2000000 | syscall number)
const SYS_EXIT: u64 = 0x2000001;
const SYS_FORK: u64 = 0x2000002;
const SYS_READ: u64 = 0x2000003;
const SYS_WRITE: u64 = 0x2000004;
const SYS_OPEN: u64 = 0x2000005;
const SYS_CLOSE: u64 = 0x2000006;
const SYS_MUNMAP: u64 = 0x2000049;
const SYS_MPROTECT: u64 = 0x200004a;
const SYS_MMAP: u64 = 0x20000c5;
const SYS_IOCTL: u64 = 0x2000036;
const SYS_ISSETUGID: u64 = 0x2000147;

#[inline]
fn syscall_nr(emu: &emu::Emu) -> u64 {
    let nr = emu.regs().rax;
    if nr & 0x2000000 != 0 {
        nr
    } else {
        0x2000000 | nr
    }
}

#[inline]
fn log_syscall(emu: &emu::Emu, msg: &str) {
    log::info!(
        "{}** {} macos syscall {} {}",
        emu.colors.light_red,
        emu.pos,
        msg,
        emu.colors.nc
    );
}

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = syscall_nr(emu);
    if !emu.call_syscall_hook(syscall_nr) {
        return;
    }

    match syscall_nr {
        SYS_EXIT => {
            let status = emu.regs().rdi;
            log_syscall(emu, &format!("exit({})", status));
            emu.stop();
        }

        SYS_WRITE => {
            let fd = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let count = emu.regs().rdx;

            log_syscall(
                emu,
                &format!("write(fd={}, buf=0x{:x}, count={})", fd, buf, count),
            );

            if fd == 1 || fd == 2 {
                let s = emu.maps.read_string(buf);
                log::info!("{}:  \"{}\"", if fd == 1 { "stdout" } else { "stderr" }, s);
            }

            // Return bytes written
            emu.regs_mut().rax = count;
        }

        SYS_READ => {
            let fd = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let count = emu.regs().rdx;
            log_syscall(
                emu,
                &format!("read(fd={}, buf=0x{:x}, count={})", fd, buf, count),
            );
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_OPEN => {
            let path_addr = emu.regs().rdi;
            let flags = emu.regs().rsi;
            let path = emu.maps.read_string(path_addr);
            log_syscall(emu, &format!("open(\"{}\", 0x{:x})", path, flags));
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_CLOSE => {
            let fd = emu.regs().rdi;
            log_syscall(emu, &format!("close(fd={})", fd));
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_FORK => {
            log_syscall(emu, "fork()");
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_MMAP => {
            let addr = emu.regs().rdi;
            let len = emu.regs().rsi;
            let prot = emu.regs().rdx;
            let flags = emu.regs().r10;
            let fd = emu.regs().r8;
            let off = emu.regs().r9;
            log_syscall(
                emu,
                &format!(
                    "mmap(addr=0x{:x}, len=0x{:x}, prot=0x{:x}, flags=0x{:x}, fd={}, off=0x{:x})",
                    addr, len, prot, flags, fd as i64, off
                ),
            );
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_MUNMAP => {
            let addr = emu.regs().rdi;
            let len = emu.regs().rsi;
            log_syscall(emu, &format!("munmap(addr=0x{:x}, len=0x{:x})", addr, len));
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_MPROTECT => {
            let addr = emu.regs().rdi;
            let len = emu.regs().rsi;
            let prot = emu.regs().rdx;
            log_syscall(
                emu,
                &format!(
                    "mprotect(addr=0x{:x}, len=0x{:x}, prot=0x{:x})",
                    addr, len, prot
                ),
            );
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_IOCTL => {
            let fd = emu.regs().rdi;
            let request = emu.regs().rsi;
            log_syscall(emu, &format!("ioctl(fd={}, request=0x{:x})", fd, request));
            emu.regs_mut().rax = u64::MAX;
        }

        SYS_ISSETUGID => {
            log_syscall(emu, "issetugid() => 0");
            emu.regs_mut().rax = 0;
        }

        _ => {
            log::warn!(
                "{}** {} macos unimplemented syscall 0x{:x} (rdi=0x{:x}, rsi=0x{:x}, rdx=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                syscall_nr,
                emu.regs().rdi,
                emu.regs().rsi,
                emu.regs().rdx,
                emu.colors.nc
            );
            emu.regs_mut().rax = u64::MAX;
        }
    }
}
