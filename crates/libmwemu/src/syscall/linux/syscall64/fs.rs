use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_READ => handle_syscall64_read(emu),
        constants::NR64_PREAD64 => handle_syscall64_pread64(emu),
        constants::NR64_WRITEV => handle_syscall64_writev(emu),
        constants::NR64_GETDENTS64 => handle_syscall64_getdents64(emu),
        constants::NR64_IOCTL => handle_syscall64_ioctl(emu),
        constants::NR64_WRITE => handle_syscall64_write(emu),
        constants::NR64_OPEN => handle_syscall64_open(emu),
        constants::NR64_OPENAT => handle_syscall64_openat(emu),
        constants::NR64_CLOSE => handle_syscall64_close(emu),
        constants::NR64_EXECVE => handle_syscall64_execve(emu),
        constants::NR64_CHDIR => handle_syscall64_chdir(emu),
        constants::NR64_CHMOD => handle_syscall64_chmod(emu),
        constants::NR64_LSEEK => handle_syscall64_lseek(emu),
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

pub(super) fn handle_syscall64_writev(emu: &mut emu::Emu) {
    use std::io::Write;
    let fd = emu.regs().rdi;
    let iov = emu.regs().rsi;
    let iovcnt = emu.regs().rdx;

    let mut data: Vec<u8> = Vec::new();
    for i in 0..iovcnt {
        let entry = iov + i * 16;
        let ptr = emu.maps.read_qword(entry).unwrap_or(0);
        let len = emu.maps.read_qword(entry + 8).unwrap_or(0);
        if ptr != 0 {
            for k in 0..len {
                data.push(emu.maps.read_byte(ptr + k).unwrap_or(0));
            }
        }
    }

    let total = data.len() as u64;
    match fd {
        1 => {
            let _ = std::io::stdout().write_all(&data);
            let _ = std::io::stdout().flush();
        }
        2 => {
            let _ = std::io::stderr().write_all(&data);
            let _ = std::io::stderr().flush();
        }
        _ => {}
    }
    log::trace!(
        "{}** {} syscall writev(fd:{} iovcnt:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        iovcnt,
        total,
        emu.colors.nc
    );
    emu.regs_mut().rax = total;
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

    if helper::handler_exist(fd) {
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
    } else if fd == 0 {
        // stdin with no backing input: report EOF so a shell's read loop ends
        // cleanly instead of spinning on fabricated data.
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = sz;
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

pub(super) fn handle_syscall64_write(emu: &mut emu::Emu) {
    use std::io::Write;
    let fd = emu.regs().rdi;
    let buff = emu.regs().rsi;
    let sz = emu.regs().rdx;

    // Read exactly `sz` bytes (the data may contain NULs/newlines) and forward
    // stdout/stderr to the host so the program's real output is visible.
    let mut data: Vec<u8> = Vec::with_capacity(sz.min(0x100000) as usize);
    for i in 0..sz {
        data.push(emu.maps.read_byte(buff + i).unwrap_or(0));
    }
    match fd {
        1 => {
            let _ = std::io::stdout().write_all(&data);
            let _ = std::io::stdout().flush();
        }
        2 => {
            let _ = std::io::stderr().write_all(&data);
            let _ = std::io::stderr().flush();
        }
        _ => {}
    }
    log::trace!(
        "{}** {} syscall write(fd:{} buf:0x{:x} sz:{}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.colors.nc
    );
    emu.regs_mut().rax = sz;
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
    let fd = emu.regs().rdi;

    if fd <= 2 {
        // Standard streams (stdin/stdout/stderr) are always open; glibc's
        // close_stdout atexit hook calls close(1) during cleanup and reports a
        // bogus "write error" if it sees a failure. Return success.
        emu.regs_mut().rax = 0;
    } else if helper::handler_exist(fd) {
        helper::handler_close(fd);
        // fd ids get recycled — drop per-fd cursors so a future open() of the
        // same id starts fresh (a stale offset corrupts e.g. the next ELF read).
        READ_CURSOR.with(|c| {
            c.borrow_mut().remove(&fd);
        });
        GETDENTS_CURSOR.with(|c| {
            c.borrow_mut().remove(&fd);
        });
        emu.regs_mut().rax = 0;
    } else {
        helper::socket_close(fd);
        emu.regs_mut().rax = 0xffffffff_ffffffff;
    }

    log::trace!(
        "{}** {} syscall close(fd:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_execve(emu: &mut emu::Emu) {
    let cmd = emu.maps.read_string(emu.regs().rdi);
    log::trace!(
        "{}** {} syscall execve()  cmd: {} {}",
        emu.colors.light_red,
        emu.pos,
        cmd,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn handle_syscall64_chdir(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rdi);
    log::trace!(
        "{}** {} syscall chdir() path: {} {}",
        emu.colors.light_red,
        emu.pos,
        path,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_chmod(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rdi);
    let perm = emu.regs().rsi;
    log::trace!(
        "{}** {} syscall chmod() file: {} perm: {} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        perm,
        emu.colors.nc
    );
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
