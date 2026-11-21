use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::io;
use crate::syscall::linux::unsupported;

use crate::winapi::helper;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        NR64_FCNTL => io::fcntl(emu, Arch::X86_64),
        NR64_READ => handle_syscall64_read(emu),
        NR64_PREAD64 => handle_syscall64_pread64(emu),
        NR64_WRITEV => io::writev(emu, Arch::X86_64),
        NR64_GETDENTS64 => handle_syscall64_getdents64(emu),
        NR64_IOCTL => handle_syscall64_ioctl(emu),
        NR64_WRITE => io::write(emu, Arch::X86_64),
        NR64_OPEN => handle_syscall64_open(emu),
        NR64_OPENAT => handle_syscall64_openat(emu),
        NR64_CLOSE => handle_syscall64_close(emu),
        NR64_CHDIR => unsupported(emu, "chdir"),
        NR64_CHMOD => unsupported(emu, "chmod"),
        NR64_LSEEK => handle_syscall64_lseek(emu),
        _ => return false,
    }

    true
}

use std::cell::RefCell;
use std::collections::HashMap;
thread_local! {
    // Per-directory-fd read cursor (entry index already returned).
    static GETDENTS_CURSOR: RefCell<HashMap<u64, usize>> = RefCell::new(HashMap::new());
    // Per-fd byte offset for sequential read().
    static READ_CURSOR: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

pub(super) fn handle_syscall64_ioctl(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let cmd = emu.regs().rsi;
    // Report stdout/stderr as "not a terminal" so programs (ls) choose the
    // simple one-entry-per-line output instead of terminal column formatting.
    emu.regs_mut().rax = 0xffffffffffffffe6; // -ENOTTY
    log::trace!(
        "{}** {} syscall ioctl(fd:{} cmd:0x{:x}) =-ENOTTY {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        cmd,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_getdents64(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let buf = emu.regs().rsi;
    let count = emu.regs().rdx;

    if !helper::handler_exist(fd) {
        emu.regs_mut().rax = 0xfffffffffffffff7; // -EBADF
        return;
    }
    let dir_path = helper::handler_get_uri(fd);

    // Read (and cache nothing — re-read each call but skip already-returned ones).
    let entries: Vec<std::fs::DirEntry> = match std::fs::read_dir(&dir_path) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => {
            emu.regs_mut().rax = 0xffffffffffffffec; // -ENOTDIR
            return;
        }
    };

    let start = GETDENTS_CURSOR.with(|c| *c.borrow().get(&fd).unwrap_or(&0));

    let mut out: Vec<u8> = Vec::new();
    let mut idx = start;
    while idx < entries.len() {
        let name = entries[idx].file_name();
        let name_bytes = name.as_encoded_bytes();
        // d_ino(8) + d_off(8) + d_reclen(2) + d_type(1) + name + NUL, 8-aligned.
        let reclen = (8 + 8 + 2 + 1 + name_bytes.len() + 1 + 7) & !7;
        if out.len() + reclen > count as usize {
            break;
        }
        let d_type: u8 = match entries[idx].file_type() {
            Ok(ft) if ft.is_dir() => 4,      // DT_DIR
            Ok(ft) if ft.is_symlink() => 10, // DT_LNK
            Ok(ft) if ft.is_file() => 8,     // DT_REG
            _ => 0,                          // DT_UNKNOWN
        };
        let ino = (idx as u64) + 1;
        let d_off = (idx as u64) + 1;
        let mut rec = Vec::with_capacity(reclen);
        rec.extend_from_slice(&ino.to_le_bytes());
        rec.extend_from_slice(&d_off.to_le_bytes());
        rec.extend_from_slice(&(reclen as u16).to_le_bytes());
        rec.push(d_type);
        rec.extend_from_slice(name_bytes);
        rec.push(0);
        rec.resize(reclen, 0);
        out.extend_from_slice(&rec);
        idx += 1;
    }

    GETDENTS_CURSOR.with(|c| {
        c.borrow_mut().insert(fd, idx);
    });

    let n = out.len() as u64;
    if let Some(map) = emu.maps.get_mem_by_addr_mut(buf) {
        let room = (map.get_base() + map.size() as u64).saturating_sub(buf);
        let w = (n.min(room)) as usize;
        map.force_write_bytes(buf, &out[..w]);
    }
    log::trace!(
        "{}** {} syscall getdents64(fd:{} '{}') ={} ({} entries) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        dir_path,
        n,
        idx - start,
        emu.colors.nc
    );
    emu.regs_mut().rax = n;
}

pub(super) fn handle_syscall64_pread64(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let buff = emu.regs().rsi;
    let sz = emu.regs().rdx;
    let off = emu.regs().r10;

    let mut data: Vec<u8> = Vec::new();
    if helper::handler_exist(fd) {
        let filepath = helper::handler_get_uri(fd);
        if let Ok(f) = File::open(&filepath) {
            let mut reader = BufReader::new(&f);
            if reader.seek(SeekFrom::Start(off)).is_ok() {
                let mut limited = reader.take(sz);
                let _ = limited.read_to_end(&mut data);
            }
        }
    }

    // Clamp to the destination map so a short buffer can't be overrun.
    if let Some(map) = emu.maps.get_mem_by_addr_mut(buff) {
        let room = (map.get_base() + map.size() as u64).saturating_sub(buff);
        if data.len() as u64 > room {
            data.truncate(room as usize);
        }
        map.force_write_bytes(buff, &data);
    }

    let n = data.len() as u64;
    log::trace!(
        "{}** {} syscall pread64(fd:{} buf:0x{:x} sz:{} off:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd as i32,
        buff,
        sz,
        off,
        n,
        emu.colors.nc
    );
    emu.regs_mut().rax = n;
}

pub(super) fn handle_syscall64_read(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let buff = emu.regs().rsi;
    let sz = emu.regs().rdx;

    if let Some(result) = io::read_descriptor(emu, Arch::X86_64, u64::from(fd as u32), buff, sz) {
        emu.regs_mut().rax = result;
    } else if helper::handler_exist(fd) {
        let filepath = helper::handler_get_uri(fd);
        // Sequential read of any host file, honouring a per-fd cursor so
        // successive read()s advance (config files, ELF headers, etc.).
        let off = READ_CURSOR.with(|c| *c.borrow().get(&fd).unwrap_or(&0));
        let mut data: Vec<u8> = Vec::new();
        if let Ok(f) = File::open(&filepath) {
            let mut reader = BufReader::new(&f);
            if reader.seek(SeekFrom::Start(off)).is_ok() {
                let _ = reader.take(sz).read_to_end(&mut data);
            }
        }

        if let Some(map) = emu.maps.get_mem_by_addr_mut(buff) {
            let room = (map.get_base() + map.size() as u64).saturating_sub(buff);
            if data.len() as u64 > room {
                data.truncate(room as usize);
            }
            map.force_write_bytes(buff, &data);
        }
        let n = data.len() as u64;
        READ_CURSOR.with(|c| {
            c.borrow_mut().insert(fd, off + n);
        });
        emu.regs_mut().rax = n;
    } else {
        emu.regs_mut().rax = io::unbacked_read(fd as u32);
    }

    log::trace!(
        "{}** {} syscall read(fd:{} buf:0x{:x} sz:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_open(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rdi);
    let fd = helper::handler_create(&file_path);

    emu.regs_mut().rax = fd;
    log::trace!(
        "{}** {} syscall open({}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        fd,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_openat(emu: &mut emu::Emu) {
    let dirfd = emu.regs().rdi;
    let file_path = emu.maps.read_string(emu.regs().rsi);
    let mut fd: u64 = 0xffffffff_ffffffff;

    // Deny the systemd/dbus runtime IPC endpoints. They exist on a host that
    // runs systemd, so opening them succeeds and drags glibc's NSS into
    // libnss_systemd's userdb protocol (varlink/epoll), which can't be emulated
    // and otherwise hangs or crashes `id`/`ls -l`. Failing the open makes NSS
    // fall back to the `files` source (/etc/passwd, /etc/group).
    let blocked = file_path.starts_with("/run/systemd/")
        || file_path.starts_with("/run/dbus/")
        || file_path.starts_with("/var/run/systemd/")
        || file_path.starts_with("/run/nscd/");

    let path = Path::new(&file_path);
    if !blocked && path.exists() {
        fd = helper::handler_create(&file_path);
    }

    log::trace!(
        "{}** {} syscall openat({} '{}') ={} {}",
        emu.colors.light_red,
        emu.pos,
        dirfd,
        file_path,
        fd as i64,
        emu.colors.nc
    );

    emu.regs_mut().rax = fd;
}

pub(super) fn handle_syscall64_close(emu: &mut emu::Emu) {
    let call = LinuxSyscall;
    let [fd, ..] = call.args(emu);
    io::close(emu, Arch::X86_64);
    if emu.regs().rax == 0 {
        let fd = u64::from(fd as u32);
        READ_CURSOR.with(|c| c.borrow_mut().remove(&fd));
        GETDENTS_CURSOR.with(|c| c.borrow_mut().remove(&fd));
    }
}

pub(super) fn handle_syscall64_lseek(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let offset = emu.regs().rsi as i64;
    let whence = emu.regs().rdx;
    const SEEK_SET: u64 = 0;
    const SEEK_CUR: u64 = 1;
    const SEEK_END: u64 = 2;

    // Keep lseek and read() consistent: both use the per-fd READ_CURSOR. A
    // no-op lseek would leave read() at the wrong offset, feeding glibc garbage
    // (e.g. when stdio repositions while parsing /etc/passwd or /etc/group),
    // which then corrupts its heap.
    let cur = READ_CURSOR.with(|c| *c.borrow().get(&fd).unwrap_or(&0)) as i64;
    let size = if helper::handler_exist(fd) {
        std::fs::metadata(helper::handler_get_uri(fd))
            .map(|m| m.len() as i64)
            .unwrap_or(0)
    } else {
        0
    };
    let new = match whence {
        SEEK_SET => offset,
        SEEK_CUR => cur + offset,
        SEEK_END => size + offset,
        _ => offset,
    }
    .max(0) as u64;

    READ_CURSOR.with(|c| {
        c.borrow_mut().insert(fd, new);
    });
    emu.regs_mut().rax = new;
    log::trace!(
        "{}** {} syscall lseek(fd:{} off:{} whence:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        offset,
        whence,
        new,
        emu.colors.nc
    );
}
