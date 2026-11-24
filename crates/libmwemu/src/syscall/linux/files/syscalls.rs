use goblin::elf::{
    Elf,
    header::{EM_386, EM_AARCH64, EM_X86_64, ET_DYN, ET_EXEC},
    program_header::PT_LOAD,
};

use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::*;
use crate::syscall::linux::constants::io::{F_GETFD, F_GETFL, F_SETFD, FD_CLOEXEC};
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::{io, process};

use super::proc_maps::proc_maps;
use super::process::PendingExec;
use super::{AT_FDCWD, LinuxFiles, MAX_IO, MAX_PATH, Node, O_ACCMODE, O_CLOEXEC, S_IFDIR, S_IFMT};

const AT_EMPTY_PATH: u64 = 0x1000;
const AT_SYMLINK_NOFOLLOW: u64 = 0x100;
const AT_REMOVEDIR: u64 = 0x200;

#[derive(Clone, Copy, Debug)]
enum Op {
    Open,
    OpenAt,
    Chdir,
    Getcwd,
    Symlink,
    SymlinkAt,
    Lstat,
    Lstat64,
    Read,
    Readv,
    Pread,
    Write,
    Writev,
    Dup,
    Dup2,
    Dup3,
    Close,
    Seek,
    LongSeek,
    Memfd,
    Truncate,
    Truncate64,
    Mkdir,
    MkdirAt,
    Unlink,
    UnlinkAt,
    Rmdir,
    Readlink,
    ReadlinkAt,
    Stat,
    Stat64,
    Fstat,
    Fstat64,
    FstatAt,
    Statx,
    Chmod,
    Fchmod,
    Access,
    AccessAt,
    Exec,
    ExecAt,
    Fcntl,
    GetDents,
}

fn operation(arch: Arch, nr: u64) -> Option<Op> {
    Some(match arch {
        Arch::X86 => match nr {
            NR32_READ => Op::Read,
            NR32_WRITE => Op::Write,
            NR32_OPEN => Op::Open,
            NR32_DUP => Op::Dup,
            NR32_DUP2 => Op::Dup2,
            NR32_DUP3 => Op::Dup3,
            NR32_CLOSE => Op::Close,
            NR32_UNLINK => Op::Unlink,
            NR32_EXECVE => Op::Exec,
            NR32_CHMOD => Op::Chmod,
            NR32_LSEEK => Op::Seek,
            NR32_ACCESS => Op::Access,
            NR32_MKDIR => Op::Mkdir,
            NR32_RMDIR => Op::Rmdir,
            NR32_FCNTL => Op::Fcntl,
            NR32_READLINK => Op::Readlink,
            NR32_FTRUNCATE => Op::Truncate,
            NR32_FCHMOD => Op::Fchmod,
            NR32_STAT => Op::Stat,
            NR32_FSTAT => Op::Fstat,
            NR32_LLSEEK => Op::LongSeek,
            NR32_READV => Op::Readv,
            NR32_WRITEV => Op::Writev,
            NR32_PREAD64 => Op::Pread,
            NR32_FTRUNCATE64 => Op::Truncate64,
            NR32_STAT64 => Op::Stat64,
            NR32_FSTAT64 => Op::Fstat64,
            NR32_GETDENTS64 => Op::GetDents,
            NR32_FCNTL64 => Op::Fcntl,
            NR32_CHDIR => Op::Chdir,
            NR32_GETCWD => Op::Getcwd,
            NR32_SYMLINK => Op::Symlink,
            NR32_SYMLINKAT => Op::SymlinkAt,
            NR32_LSTAT => Op::Lstat,
            NR32_LSTAT64 => Op::Lstat64,
            NR32_OPENAT => Op::OpenAt,
            NR32_MKDIRAT => Op::MkdirAt,
            NR32_NEWFSTATAT => Op::FstatAt,
            NR32_UNLINKAT => Op::UnlinkAt,
            NR32_READLINKAT => Op::ReadlinkAt,
            NR32_FACCESSAT => Op::AccessAt,
            NR32_MEMFD_CREATE => Op::Memfd,
            NR32_EXECVEAT => Op::ExecAt,
            NR32_STATX => Op::Statx,
            _ => return None,
        },
        Arch::X86_64 => match nr {
            NR64_READ => Op::Read,
            NR64_WRITE => Op::Write,
            NR64_OPEN => Op::Open,
            NR64_DUP => Op::Dup,
            NR64_DUP2 => Op::Dup2,
            NR64_DUP3 => Op::Dup3,
            NR64_CLOSE => Op::Close,
            NR64_STAT => Op::Stat,
            NR64_FSTAT => Op::Fstat,
            NR64_LSEEK => Op::Seek,
            NR64_PREAD64 => Op::Pread,
            NR64_READV => Op::Readv,
            NR64_WRITEV => Op::Writev,
            NR64_ACCESS => Op::Access,
            NR64_EXECVE => Op::Exec,
            NR64_FCNTL => Op::Fcntl,
            NR64_FTRUNCATE => Op::Truncate,
            NR64_MKDIR => Op::Mkdir,
            NR64_RMDIR => Op::Rmdir,
            NR64_UNLINK => Op::Unlink,
            NR64_READLINK => Op::Readlink,
            NR64_CHMOD => Op::Chmod,
            NR64_FCHMOD => Op::Fchmod,
            NR64_GETDENTS64 => Op::GetDents,
            NR64_CHDIR => Op::Chdir,
            NR64_GETCWD => Op::Getcwd,
            NR64_SYMLINK => Op::Symlink,
            NR64_SYMLINKAT => Op::SymlinkAt,
            NR64_LSTAT => Op::Lstat,
            NR64_OPENAT => Op::OpenAt,
            NR64_MKDIRAT => Op::MkdirAt,
            NR64_NEWFSTATAT => Op::FstatAt,
            NR64_UNLINKAT => Op::UnlinkAt,
            NR64_READLINKAT => Op::ReadlinkAt,
            NR64_FACCESSAT => Op::AccessAt,
            NR64_MEMFD_CREATE => Op::Memfd,
            NR64_EXECVEAT => Op::ExecAt,
            NR64_STATX => Op::Statx,
            _ => return None,
        },
        Arch::Aarch64 => match nr {
            NRA64_FCNTL => Op::Fcntl,
            NRA64_MKDIRAT => Op::MkdirAt,
            NRA64_UNLINKAT => Op::UnlinkAt,
            NRA64_FTRUNCATE => Op::Truncate,
            NRA64_FACCESSAT => Op::AccessAt,
            NRA64_FCHMOD => Op::Fchmod,
            NRA64_CHDIR => Op::Chdir,
            NRA64_GETCWD => Op::Getcwd,
            NRA64_SYMLINKAT => Op::SymlinkAt,
            NRA64_OPENAT => Op::OpenAt,
            NRA64_DUP => Op::Dup,
            NRA64_DUP3 => Op::Dup3,
            NRA64_CLOSE => Op::Close,
            NRA64_GETDENTS64 => Op::GetDents,
            NRA64_LSEEK => Op::Seek,
            NRA64_READ => Op::Read,
            NRA64_WRITE => Op::Write,
            NRA64_READV => Op::Readv,
            NRA64_WRITEV => Op::Writev,
            NRA64_PREAD64 => Op::Pread,
            NRA64_READLINKAT => Op::ReadlinkAt,
            NRA64_NEWFSTATAT => Op::FstatAt,
            NRA64_FSTAT => Op::Fstat,
            NRA64_EXECVE => Op::Exec,
            NRA64_MEMFD_CREATE => Op::Memfd,
            NRA64_EXECVEAT => Op::ExecAt,
            NRA64_STATX => Op::Statx,
            _ => return None,
        },
    })
}

fn text(emu: &Emu, ptr: u64) -> Result<String, u64> {
    if ptr == 0 {
        return Err(EFAULT);
    }
    let mut bytes = Vec::new();
    for i in 0..MAX_PATH {
        let at = ptr.checked_add(i as u64).ok_or(EFAULT)?;
        let b = emu.maps.read_byte(at).ok_or(EFAULT)?;
        if b == 0 {
            return String::from_utf8(bytes).map_err(|_| EINVAL);
        }
        bytes.push(b);
    }
    Err(ENAMETOOLONG)
}

fn put(emu: &mut Emu, address: u64, bytes: &[u8]) -> Result<u64, u64> {
    if !bytes.is_empty()
        && (!emu
            .maps
            .can_access_range(address, bytes.len(), Permission::WRITE)
            || !emu.maps.write_bytes(address, bytes))
    {
        return Err(EFAULT);
    }
    Ok(bytes.len() as u64)
}

fn validate_exec_image(arch: Arch, elf: &Elf<'_>, file_len: usize) -> Result<(), u64> {
    let compatible = match (elf.is_64, elf.header.e_machine) {
        (false, EM_386) | (true, EM_X86_64) => arch.is_x86(),
        (true, EM_AARCH64) => arch == Arch::Aarch64,
        _ => false,
    };
    if !compatible || !elf.little_endian || !matches!(elf.header.e_type, ET_EXEC | ET_DYN) {
        return Err(ENOEXEC);
    }
    let mut executable_entry = false;
    for segment in elf.program_headers.iter().filter(|p| p.p_type == PT_LOAD) {
        let end = segment
            .p_vaddr
            .checked_add(segment.p_memsz)
            .ok_or(ENOEXEC)?;
        if segment.p_filesz > segment.p_memsz
            || segment
                .p_offset
                .checked_add(segment.p_filesz)
                .is_none_or(|n| n > file_len as u64)
        {
            return Err(ENOEXEC);
        }
        executable_entry |= segment.is_executable() && (segment.p_vaddr..end).contains(&elf.entry);
    }
    if executable_entry {
        Ok(())
    } else {
        Err(ENOEXEC)
    }
}

fn exec_args(emu: &Emu, arch: Arch, address: u64) -> Result<Vec<String>, u64> {
    const MAX_ARGS: usize = 256;
    const MAX_ARG_BYTES: usize = 128 * 1024;
    let mut args = Vec::new();
    if address == 0 {
        return Ok(args);
    }
    let width = if arch == Arch::X86 { 4 } else { 8 };
    let mut size = 0;
    for index in 0..=MAX_ARGS {
        let slot = address.checked_add(index as u64 * width).ok_or(EFAULT)?;
        let pointer = if width == 4 {
            emu.maps.read_dword(slot).map(u64::from)
        } else {
            emu.maps.read_qword(slot)
        }
        .ok_or(EFAULT)?;
        if pointer == 0 {
            return Ok(args);
        }
        let arg = text(emu, pointer)?;
        size += arg.len() + 1;
        if index == MAX_ARGS || size > MAX_ARG_BYTES {
            return Err(E2BIG);
        }
        args.push(arg);
    }
    Err(E2BIG)
}

fn split_offset(low: u64, high: u64) -> u64 {
    ((high as u32 as u64) << 32) | (low as u32 as u64)
}

fn transfer(
    fs: &mut LinuxFiles,
    emu: &mut Emu,
    arch: Arch,
    fd: u32,
    buffers: &[(u64, u64)],
    read: bool,
    offset: Option<u64>,
) -> Result<u64, u64> {
    let flags = fs.open.get(&fd).ok_or(EBADF)?.flags & O_ACCMODE;
    if (read && flags == super::O_WRONLY) || (!read && flags == 0) {
        return Err(EBADF);
    }
    let mut total = 0u64;
    for &(address, size) in buffers {
        let mut done = 0;
        let available = size.min(super::MAX_STORAGE as u64 - total);
        while done < available {
            let count = (available - done).min(MAX_IO as u64) as usize;
            let result = (|| {
                let addr = address.checked_add(done).ok_or(EFAULT)?;
                if read {
                    let pos = match offset {
                        Some(offset) => offset.checked_add(total).ok_or(EOVERFLOW)?,
                        None => fs.open.get(&fd).ok_or(EBADF)?.offset,
                    };
                    if fs.node(fd)?.name == "/dev/urandom" {
                        if offset.is_some() {
                            return Err(ESPIPE);
                        }
                        return emu
                            .fill_random(addr, count as u64)
                            .then_some(count as u64)
                            .ok_or(EFAULT);
                    }
                    let bytes = fs.read_at(fd, pos, count)?;
                    let n = put(emu, addr, bytes)?;
                    if offset.is_none() {
                        fs.open.get_mut(&fd).ok_or(EBADF)?.offset = pos + n;
                    }
                    Ok(n)
                } else {
                    let bytes = io::read_bytes(emu, arch, addr, count)?;
                    match fs.node(fd)?.name.as_str() {
                        "/dev/null" => Ok(bytes.len() as u64),
                        stream @ ("stdout" | "stderr") => {
                            let bytes = bytes.into_owned();
                            let output = if stream == "stdout" { 1 } else { 2 };
                            io::emit(emu, output, &bytes);
                            Ok(bytes.len() as u64)
                        }
                        _ => fs.write(fd, &bytes),
                    }
                }
            })();
            match result {
                Ok(n) => {
                    total += n;
                    done += n;
                    if n < count as u64 {
                        return Ok(total);
                    }
                }
                Err(_) if total > 0 => return Ok(total),
                Err(error) => return Err(error),
            }
        }
        if available < size {
            break;
        }
    }
    Ok(total)
}

fn getdents64(
    fs: &mut LinuxFiles,
    emu: &mut Emu,
    fd: u32,
    address: u64,
    count: u32,
) -> Result<u64, u64> {
    const NAME_OFFSET: usize = 19;
    const RECORD_ALIGNMENT: usize = 8;
    const S_DT_SHIFT: u32 = 12;

    let file = *fs.open.get(&fd).ok_or(EBADF)?;
    let directory = fs.node(fd)?;
    if !directory.directory() {
        return Err(ENOTDIR);
    }
    if fs.paths.get(&directory.name) != Some(&file.inode) {
        return Err(ENOENT);
    }
    let parent = directory
        .name
        .rsplit_once('/')
        .map_or("", |(parent, _)| parent);
    let parent_inode = fs.lookup_raw(if parent.is_empty() { "/" } else { parent })?;
    let dir = directory.name.trim_end_matches('/');
    let dots = [
        (0, file.inode, S_IFDIR, "."),
        (1, parent_inode, S_IFDIR, ".."),
    ];
    // Inode numbers are monotonic. Removing an entry between reads must not
    // shift the cookies for the entries that follow it.
    let children = fs
        .nodes
        .range(file.offset.saturating_sub(2)..)
        .filter_map(|(&inode, node)| {
            let (parent, name) = node.name.rsplit_once('/')?;
            (parent == dir && !name.is_empty() && fs.paths.get(&node.name) == Some(&inode))
                .then_some((inode + 2, inode, node.mode, name))
        });
    let mut written = 0;
    let mut cursor = file.offset;
    let mut record = Vec::new();
    for (cookie, inode, mode, name) in dots
        .into_iter()
        .chain(children)
        .filter(|entry| entry.0 >= file.offset)
    {
        let length = (NAME_OFFSET + name.len() + 1).next_multiple_of(RECORD_ALIGNMENT);
        if length as u64 > count as u64 - written {
            if written == 0 {
                return Err(EINVAL);
            }
            break;
        }
        record.clear();
        record.extend_from_slice(&inode.to_le_bytes());
        record.extend_from_slice(&(cookie + 1).to_le_bytes());
        record.extend_from_slice(&(length as u16).to_le_bytes());
        record.push(((mode & S_IFMT) >> S_DT_SHIFT) as u8);
        record.extend_from_slice(name.as_bytes());
        record.resize(length, 0);
        let copied = address
            .checked_add(written)
            .ok_or(EFAULT)
            .and_then(|at| put(emu, at, &record));
        if let Err(errno) = copied {
            if written == 0 {
                return Err(errno);
            }
            break;
        }
        written += length as u64;
        cursor = cookie + 1;
    }
    fs.open.get_mut(&fd).ok_or(EBADF)?.offset = cursor;
    Ok(written)
}

fn stat(
    emu: &mut Emu,
    arch: Arch,
    node: &Node,
    inode: u64,
    address: u64,
    large: bool,
) -> Result<u64, u64> {
    let size = node.bytes.len() as u64;
    let mut bytes = [0u8; 144];
    let len = match arch {
        Arch::X86_64 => {
            bytes[0..8].copy_from_slice(&1u64.to_le_bytes());
            bytes[8..16].copy_from_slice(&inode.to_le_bytes());
            bytes[16..24].copy_from_slice(&1u64.to_le_bytes());
            bytes[24..28].copy_from_slice(&node.mode.to_le_bytes());
            bytes[48..56].copy_from_slice(&size.to_le_bytes());
            bytes[56..64].copy_from_slice(&4096u64.to_le_bytes());
            bytes[64..72].copy_from_slice(&size.div_ceil(512).to_le_bytes());
            144
        }
        Arch::Aarch64 => {
            bytes[0..8].copy_from_slice(&1u64.to_le_bytes());
            bytes[8..16].copy_from_slice(&inode.to_le_bytes());
            bytes[16..20].copy_from_slice(&node.mode.to_le_bytes());
            bytes[20..24].copy_from_slice(&1u32.to_le_bytes());
            bytes[48..56].copy_from_slice(&size.to_le_bytes());
            bytes[56..60].copy_from_slice(&4096u32.to_le_bytes());
            bytes[64..72].copy_from_slice(&size.div_ceil(512).to_le_bytes());
            128
        }
        Arch::X86 if large => {
            bytes[0..8].copy_from_slice(&1u64.to_le_bytes());
            bytes[12..16].copy_from_slice(&(inode as u32).to_le_bytes());
            bytes[16..20].copy_from_slice(&node.mode.to_le_bytes());
            bytes[20..24].copy_from_slice(&1u32.to_le_bytes());
            bytes[44..52].copy_from_slice(&size.to_le_bytes());
            bytes[52..56].copy_from_slice(&4096u32.to_le_bytes());
            bytes[56..64].copy_from_slice(&size.div_ceil(512).to_le_bytes());
            bytes[88..96].copy_from_slice(&inode.to_le_bytes());
            96
        }
        Arch::X86 => {
            bytes[0..4].copy_from_slice(&1u32.to_le_bytes());
            bytes[4..8].copy_from_slice(&(inode as u32).to_le_bytes());
            bytes[8..10].copy_from_slice(&(node.mode as u16).to_le_bytes());
            bytes[10..12].copy_from_slice(&1u16.to_le_bytes());
            bytes[20..24].copy_from_slice(&(size as u32).to_le_bytes());
            bytes[24..28].copy_from_slice(&4096u32.to_le_bytes());
            bytes[28..32].copy_from_slice(&(size.div_ceil(512) as u32).to_le_bytes());
            64
        }
    };
    put(emu, address, &bytes[..len])?;
    Ok(0)
}

fn handle(
    op: Op,
    fs: &mut LinuxFiles,
    emu: &mut Emu,
    arch: Arch,
    args: [u64; 6],
) -> Result<u64, u64> {
    let [a, b, c, d, e, _] = args;
    let fd = a as u32;
    match op {
        Op::Getcwd => {
            let mut bytes = fs.cwd.as_bytes().to_vec();
            bytes.push(0);
            if b < bytes.len() as u64 {
                return Err(ERANGE);
            }
            put(emu, a, &bytes)
        }
        Op::Chdir => {
            let path = fs.path(AT_FDCWD, &text(emu, a)?)?;
            let inode = fs.lookup(&path)?;
            if !fs.nodes.get(&inode).is_some_and(Node::directory) {
                return Err(ENOTDIR);
            }
            fs.cwd = path;
            Ok(0)
        }
        Op::Symlink | Op::SymlinkAt => {
            let (dir, link) = if matches!(op, Op::SymlinkAt) {
                (b as i32, c)
            } else {
                (AT_FDCWD, b)
            };
            let link = fs.path(dir, &text(emu, link)?)?;
            let target = text(emu, a)?;
            if target.is_empty() {
                return Err(ENOENT);
            }
            if target.len() > super::MAX_STORAGE - fs.allocated {
                return Err(ENOSPC);
            }
            let id = fs.create(link, super::S_IFLNK | 0o777, true)?;
            let node = fs.nodes.get_mut(&id).ok_or(ENOENT)?;
            fs.allocated += target.len();
            node.bytes = target.into_bytes();
            Ok(0)
        }
        Op::Open | Op::OpenAt => {
            let (dir, path, flags, mode) = if matches!(op, Op::Open) {
                (AT_FDCWD, a, b, c)
            } else {
                (a as i32, b, c, d)
            };
            let path = fs.path(dir, &text(emu, path)?)?;
            log::trace!("guest open path={path:?} flags={flags:#x}");
            if path == "/proc/self/maps" {
                return fs.proc_snapshot(path, proc_maps(emu, fs), flags as u32);
            }
            fs.open_path(path, flags as u32, mode as u32)
        }
        Op::Read | Op::Pread | Op::Write => {
            let offset = if matches!(op, Op::Pread) {
                Some(if arch == Arch::X86 {
                    split_offset(d, e)
                } else {
                    d
                })
            } else {
                None
            };
            if offset.is_some_and(|o| o > i64::MAX as u64) {
                return Err(EINVAL);
            }
            transfer(
                fs,
                emu,
                arch,
                fd,
                &[(b, c)],
                !matches!(op, Op::Write),
                offset,
            )
        }
        Op::Readv | Op::Writev => {
            let buffers = io::read_iovecs(emu, arch, b, c)?;
            transfer(fs, emu, arch, fd, &buffers, matches!(op, Op::Readv), None)
        }
        Op::Dup => fs.duplicate(fd, None, false),
        Op::Dup2 => fs.duplicate(fd, Some(b as u32), false),
        Op::Dup3 => {
            if fd == b as u32 || c & !(O_CLOEXEC as u64) != 0 {
                return Err(EINVAL);
            }
            fs.duplicate(fd, Some(b as u32), c != 0)
        }
        Op::Close => fs.close(fd),
        Op::Seek => fs.seek(
            fd,
            if arch == Arch::X86 {
                b as i32 as i64
            } else {
                b as i64
            },
            c,
        ),
        Op::LongSeek => {
            if !emu.maps.can_access_range(d, 8, Permission::WRITE) {
                return Err(EFAULT);
            }
            let value = fs.seek(fd, split_offset(c, b) as i64, e)?;
            put(emu, d, &value.to_le_bytes())?;
            Ok(0)
        }
        Op::Memfd => fs.create_memfd(text(emu, a)?, b as u32),
        Op::Truncate | Op::Truncate64 => {
            let size = if matches!(op, Op::Truncate64) {
                split_offset(b, c)
            } else {
                b
            };
            if size > i64::MAX as u64 {
                return Err(EINVAL);
            }
            fs.truncate(fd, size)
        }
        Op::Mkdir | Op::MkdirAt => {
            let (dir, path, mode) = if matches!(op, Op::Mkdir) {
                (AT_FDCWD, a, b)
            } else {
                (a as i32, b, c)
            };
            let path = fs.path(dir, &text(emu, path)?)?;
            fs.create(path, S_IFDIR | (mode as u32 & 0o777), true)?;
            Ok(0)
        }
        Op::Unlink | Op::UnlinkAt | Op::Rmdir => {
            let (dir, path, flags) = if matches!(op, Op::UnlinkAt) {
                (a as i32, b, c)
            } else {
                (AT_FDCWD, a, 0)
            };
            if flags & !AT_REMOVEDIR != 0 {
                return Err(EINVAL);
            }
            let path = text(emu, path)?;
            log::trace!(
                "linux guest {op:?} pid={} dirfd={dir} path={path:?}",
                fs.pid
            );
            fs.unlink(&fs.path(dir, &path)?, flags != 0 || matches!(op, Op::Rmdir))
        }
        Op::Readlink | Op::ReadlinkAt => {
            let (dir, path, buf, size) = if matches!(op, Op::Readlink) {
                (AT_FDCWD, a, b, c)
            } else {
                (a as i32, b, c, d)
            };
            let path = fs.path(dir, &text(emu, path)?)?;
            let target = fs.target(&path)?;
            let bytes = target.as_bytes();
            if size == 0 {
                return Err(EINVAL);
            }
            put(emu, buf, &bytes[..bytes.len().min(size as usize)])
        }
        Op::Fstat | Op::Fstat64 => {
            let f = fs.open.get(&fd).ok_or(EBADF)?;
            stat(
                emu,
                arch,
                fs.node(fd)?,
                f.inode,
                b,
                matches!(op, Op::Fstat64),
            )
        }
        Op::Stat | Op::Stat64 | Op::Lstat | Op::Lstat64 | Op::FstatAt => {
            let (dir, path, buf, flags) = if matches!(op, Op::FstatAt) {
                (a as i32, b, c, d)
            } else {
                (AT_FDCWD, a, b, 0)
            };
            let path = text(emu, path)?;
            let inode = if path.is_empty() && flags & AT_EMPTY_PATH != 0 {
                fs.open.get(&fd).ok_or(EBADF)?.inode
            } else {
                let path = fs.path(dir, &path)?;
                if matches!(op, Op::Lstat | Op::Lstat64) || flags & AT_SYMLINK_NOFOLLOW != 0 {
                    fs.lookup_raw(&path)?
                } else {
                    fs.lookup(&path)?
                }
            };
            stat(
                emu,
                arch,
                fs.nodes.get(&inode).ok_or(ENOENT)?,
                inode,
                buf,
                !matches!(op, Op::Stat | Op::Lstat),
            )
        }
        Op::Chmod | Op::Fchmod => {
            let inode = if matches!(op, Op::Fchmod) {
                fs.open.get(&fd).ok_or(EBADF)?.inode
            } else {
                fs.lookup(&fs.path(AT_FDCWD, &text(emu, a)?)?)?
            };
            let n = fs.nodes.get_mut(&inode).ok_or(ENOENT)?;
            if !n.output {
                return Err(EROFS);
            }
            n.mode = (n.mode & !0o7777) | (b as u32 & 0o7777);
            Ok(0)
        }
        Op::Access | Op::AccessAt => {
            let (dir, path, mode) = if matches!(op, Op::Access) {
                (AT_FDCWD, a, b)
            } else {
                (a as i32, b, c)
            };
            let path = text(emu, path)?;
            log::trace!(
                "linux guest {op:?} pid={} dirfd={dir} path={path:?} mode={mode:#o}",
                fs.pid
            );
            let inode = fs.lookup(&fs.path(dir, &path)?)?;
            let n = fs.nodes.get(&inode).ok_or(ENOENT)?;
            if mode & !7 != 0 {
                return Err(EINVAL);
            }
            if (n.mode >> 6) & mode as u32 != mode as u32 {
                return Err(EACCES);
            }
            Ok(0)
        }
        Op::Fcntl => match b as u32 {
            F_GETFD => Ok(if fs.open.close_on_exec(fd)? {
                FD_CLOEXEC
            } else {
                0
            }),
            F_SETFD => fs.open.set_close_on_exec(fd, c & FD_CLOEXEC != 0),
            F_GETFL => {
                Ok((fs.open.get(&fd).ok_or(EBADF)?.flags & (O_ACCMODE | super::O_APPEND)) as u64)
            }
            _ => Err(ENOSYS),
        },
        Op::Exec | Op::ExecAt => {
            let (dir, ptr, argv, envp, flags) = if matches!(op, Op::Exec) {
                (AT_FDCWD, a, b, c, 0)
            } else {
                (a as i32, b, c, d, e)
            };
            if flags & !(AT_EMPTY_PATH | AT_SYMLINK_NOFOLLOW) != 0 {
                return Err(EINVAL);
            }
            let path = text(emu, ptr)?;
            log::trace!(
                "linux guest {op:?} pid={} dirfd={dir} path={path:?} flags={flags:#x}",
                fs.pid
            );
            let inode = if path.is_empty() && flags & AT_EMPTY_PATH != 0 {
                fs.open.get(&fd).ok_or(EBADF)?.inode
            } else {
                let path = fs.path(dir, &path)?;
                if flags & AT_SYMLINK_NOFOLLOW != 0 && fs.target(&path).is_ok() {
                    return Err(ELOOP);
                }
                fs.lookup(&path)?
            };
            let node = fs.nodes.get(&inode).ok_or(ENOENT)?;
            if node.mode & super::S_IFMT != super::S_IFREG || node.mode & 0o111 == 0 {
                return Err(EACCES);
            }
            let argv = exec_args(emu, arch, argv)?;
            let envp = exec_args(emu, arch, envp)?;
            let elf = Elf::parse(&node.bytes).map_err(|_| ENOEXEC)?;
            validate_exec_image(arch, &elf, node.bytes.len())?;
            if arch.is_x86() {
                let filename =
                    if !matches!(op, Op::ExecAt) || dir == AT_FDCWD || path.starts_with('/') {
                        path
                    } else if path.is_empty() {
                        format!("/dev/fd/{dir}")
                    } else {
                        format!("/dev/fd/{dir}/{path}")
                    };
                fs.pending_exec = Some(PendingExec {
                    inode,
                    filename,
                    argv,
                    envp,
                });
                return Ok(0);
            }
            if node.output {
                fs.record_exec(inode);
                fs.exec_handoff = true;
                emu.stop();
            }
            Err(ENOSYS)
        }
        Op::GetDents => getdents64(fs, emu, fd, b, c as u32),
        Op::Statx => Err(ENOSYS),
    }
}

pub(crate) fn dispatch(emu: &mut Emu, arch: Arch, nr: u64) -> bool {
    if emu.linux_files.is_none() {
        return false;
    }
    let Some(op) = operation(arch, nr) else {
        return false;
    };
    let args = LinuxSyscall.args_for(emu, arch);
    let Some(result) = emulate(emu, arch, op, args) else {
        return false;
    };
    if result == 0 && matches!(op, Op::Exec | Op::ExecAt) {
        if let Err(errno) = process::exec_image(emu) {
            if let Some(files) = &mut emu.linux_files {
                files.pending_exec = None;
            }
            LinuxSyscall.set_return(emu, errno);
        }
    } else {
        LinuxSyscall.set_return(emu, result);
    }
    true
}

pub(crate) fn open(emu: &mut Emu, path: u64, flags: u32, mode: u32) -> Option<u64> {
    emulate(
        emu,
        emu.cfg.arch,
        Op::Open,
        [path, flags as u64, mode as u64, 0, 0, 0],
    )
}

pub(crate) fn read(emu: &mut Emu, fd: u32, address: u64, size: u64) -> Option<u64> {
    emulate(
        emu,
        emu.cfg.arch,
        Op::Read,
        [fd as u64, address, size, 0, 0, 0],
    )
}

pub(crate) fn write(emu: &mut Emu, fd: u32, address: u64, size: u64) -> Option<u64> {
    emulate(
        emu,
        emu.cfg.arch,
        Op::Write,
        [fd as u64, address, size, 0, 0, 0],
    )
}

pub(crate) fn close(emu: &mut Emu, fd: u32) -> Option<u64> {
    emulate(emu, emu.cfg.arch, Op::Close, [fd as u64, 0, 0, 0, 0, 0])
}

fn emulate(emu: &mut Emu, arch: Arch, op: Op, args: [u64; 6]) -> Option<u64> {
    let fd = args[0] as u32;
    let files = emu.linux_files.as_mut()?;
    if files.external.contains(&fd) {
        if matches!(op, Op::GetDents) {
            return Some(ENOTDIR);
        }
        if matches!(op, Op::Close) {
            return Some(files.close_external(fd));
        }
        if matches!(
            op,
            Op::Read | Op::Write | Op::Readv | Op::Writev | Op::Fcntl
        ) && io::is_modeled_descriptor(fd)
        {
            return None;
        }
    }
    let mut fs = emu.linux_files.take()?;
    fs.pid = emu.processes.pid;
    fs.sync_mappings(&emu.maps);
    let result = handle(op, &mut fs, emu, arch, args).unwrap_or_else(|errno| errno);
    fs.sync_mappings(&emu.maps);
    emu.linux_files = Some(fs);

    log::trace!("linux guest files {op:?} => {}", result as i64);
    Some(result)
}
