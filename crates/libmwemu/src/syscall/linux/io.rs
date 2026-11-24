use crate::arch::Arch;
use std::borrow::Cow;
use std::io::Write;

use crate::api::abi::unix::LinuxSyscall;
use crate::api::windows::helper;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{EAGAIN, EBADF, EFAULT, EINVAL, ENOSYS, EPIPE};
use crate::syscall::linux::constants::io::*;
use crate::syscall::linux::files::external_descriptor;

// Bound host work and buffering per call; callers can retry a short write.
const WRITE_CHUNK_BYTES: usize = 64 * 1024;
const IOVEC_FIELDS: usize = 2;
const PIPE_CAPACITY: usize = 64 * 1024;
const EVENT_BYTES: usize = size_of::<u64>();
const EVENT_MAX: u64 = u64::MAX - 1;
const PIPE_READ_PREFIX: &str = "pipe:r:";
const PIPE_WRITE_PREFIX: &str = "pipe:w:";
const EVENT_PREFIX: &str = "eventfd:";

#[derive(Clone, Copy)]
enum Descriptor {
    PipeRead { flags: u32 },
    PipeWrite { reader: u64, flags: u32 },
    Event { flags: u32 },
}

impl Descriptor {
    fn flags(self) -> u32 {
        match self {
            Self::PipeRead { flags } | Self::PipeWrite { flags, .. } | Self::Event { flags } => {
                flags
            }
        }
    }

    fn access(self) -> u64 {
        match self {
            Self::PipeRead { .. } => O_RDONLY,
            Self::PipeWrite { .. } => O_WRONLY,
            Self::Event { .. } => O_RDWR,
        }
    }
}

pub(super) fn is_modeled_descriptor(fd: u32) -> bool {
    descriptor(fd as u64).is_some()
}

fn descriptor(fd: u64) -> Option<Descriptor> {
    let uri = helper::handler_get_uri(fd);
    if let Some(flags) = uri.strip_prefix(PIPE_READ_PREFIX) {
        return flags
            .parse()
            .ok()
            .map(|flags| Descriptor::PipeRead { flags });
    }
    if let Some(rest) = uri.strip_prefix(PIPE_WRITE_PREFIX) {
        let (reader, flags) = rest.split_once(':')?;
        return Some(Descriptor::PipeWrite {
            reader: reader.parse().ok()?,
            flags: flags.parse().ok()?,
        });
    }
    uri.strip_prefix(EVENT_PREFIX)?
        .parse()
        .ok()
        .map(|flags| Descriptor::Event { flags })
}

fn would_block(flags: u32) -> u64 {
    // Blocking scheduler integration is not modeled; never fabricate EOF or success.
    if flags & O_NONBLOCK != 0 {
        EAGAIN
    } else {
        ENOSYS
    }
}

fn event_count(fd: u64) -> Result<u64, u64> {
    let bytes: [u8; EVENT_BYTES] = helper::handler_get_bytes(fd)
        .try_into()
        .map_err(|_| ENOSYS)?;
    Ok(u64::from_le_bytes(bytes))
}

pub(super) fn eventfd(emu: &mut Emu, arch: Arch, has_flags: bool) {
    let call = LinuxSyscall;
    let [initial, flags, ..] = call.args_for(emu, arch);
    let flags = if has_flags { flags as u32 } else { 0 };
    let result = if flags & !(EFD_SEMAPHORE | EFD_NONBLOCK | EFD_CLOEXEC) != 0 {
        EINVAL
    } else {
        let fd = external_descriptor(emu, Some(&format!("{EVENT_PREFIX}{flags}")));
        if fd <= u32::MAX as u64 {
            helper::handler_put_bytes(fd, &u64::from(initial as u32).to_le_bytes());
        }
        fd
    };
    log::trace!(
        "syscall eventfd(initial:{initial} flags:{flags:#x}) ={}",
        result as i64
    );
    call.set_return(emu, result);
}

pub(super) fn pipe(emu: &mut Emu, arch: Arch, has_flags: bool) {
    let call = LinuxSyscall;
    let [output, flags, ..] = call.args_for(emu, arch);
    let flags = if has_flags { flags as u32 } else { 0 };
    let result = if flags & !(O_NONBLOCK | O_CLOEXEC) != 0 {
        EINVAL
    } else if !writable(emu, arch, output, 2 * size_of::<u32>()) {
        EFAULT
    } else {
        let reader = external_descriptor(emu, Some(&format!("{PIPE_READ_PREFIX}{flags}")));
        if (reader as i64) < 0 {
            call.set_return(emu, reader);
            return;
        }
        let writer =
            external_descriptor(emu, Some(&format!("{PIPE_WRITE_PREFIX}{reader}:{flags}")));
        if (writer as i64) < 0 {
            close_external(emu, reader);
            call.set_return(emu, writer);
            return;
        }
        let mut bytes = [0; 2 * size_of::<u32>()];
        bytes[..size_of::<u32>()].copy_from_slice(&(reader as u32).to_le_bytes());
        bytes[size_of::<u32>()..].copy_from_slice(&(writer as u32).to_le_bytes());
        if emu.maps.write_bytes(output, &bytes) {
            0
        } else {
            close_external(emu, reader);
            close_external(emu, writer);
            EFAULT
        }
    };
    log::trace!(
        "syscall pipe(output:{output:#x} flags:{flags:#x}) ={}",
        result as i64
    );
    call.set_return(emu, result);
}

fn close_external(emu: &mut Emu, fd: u64) {
    if let Some(files) = &mut emu.linux_files {
        files.close_external(fd as u32);
    } else {
        helper::handler_close(fd);
    }
}

pub(super) fn close(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [fd, ..] = call.args_for(emu, arch);
    let result = close_fd(fd as u32);
    log::trace!("syscall close(fd:{fd}) ={}", result as i64);
    call.set_return(emu, result);
}

pub(crate) fn close_fd(fd: u32) -> u64 {
    if fd <= STDERR_FILENO {
        // The standard streams are fixed for the lifetime of this guest model.
        return ENOSYS;
    }
    let handle = u64::from(fd);
    if helper::handler_close(handle) {
        0
    } else if helper::socket_exist(handle) {
        helper::socket_close(handle);
        0
    } else {
        EBADF
    }
}

pub(super) fn read_descriptor(
    emu: &mut Emu,
    arch: Arch,
    fd: u64,
    address: u64,
    count: u64,
) -> Option<u64> {
    let kind = descriptor(fd)?;
    let result = (|| {
        match kind {
            Descriptor::PipeWrite { .. } => Err(EBADF),
            Descriptor::Event { flags } => {
                if count < EVENT_BYTES as u64 {
                    return Err(EINVAL);
                }
                let current = event_count(fd)?;
                if current == 0 {
                    return Err(would_block(flags));
                }
                let value = if flags & EFD_SEMAPHORE != 0 {
                    1
                } else {
                    current
                };
                // Linux consumes the counter before copying the result to user memory.
                helper::handler_put_bytes(fd, &(current - value).to_le_bytes());
                if !writable(emu, arch, address, EVENT_BYTES)
                    || !emu.maps.write_bytes(address, &value.to_le_bytes())
                {
                    return Err(EFAULT);
                }
                Ok(EVENT_BYTES as u64)
            }
            Descriptor::PipeRead { flags } => {
                if count == 0 {
                    return Ok(0);
                }
                let bytes = helper::handler_get_bytes(fd);
                if bytes.is_empty() {
                    let writer = format!("{PIPE_WRITE_PREFIX}{fd}:{flags}");
                    return if helper::handler_find_by_uri(&writer).is_some() {
                        Err(would_block(flags))
                    } else {
                        Ok(0)
                    };
                }
                let size = count.min(bytes.len() as u64) as usize;
                if !writable(emu, arch, address, size)
                    || !emu.maps.write_bytes(address, &bytes[..size])
                {
                    return Err(EFAULT);
                }
                helper::handler_put_bytes(fd, &bytes[size..]);
                Ok(size as u64)
            }
        }
    })()
    .unwrap_or_else(|error| error);
    Some(result)
}

fn descriptor_error(fd: u32) -> u64 {
    if helper::handler_exist(fd as u64) || helper::socket_exist(fd as u64) {
        ENOSYS
    } else {
        EBADF
    }
}

/// The default model has an empty stdin and no generic source of file/socket data.
pub(super) fn unbacked_read(fd: u32) -> u64 {
    match fd {
        STDIN_FILENO => 0,
        STDOUT_FILENO | STDERR_FILENO => EBADF,
        _ => descriptor_error(fd),
    }
}

pub(super) fn read(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [fd, buffer, count, ..] = call.args_for(emu, arch);
    let result = read_fd(emu, arch, fd as u32, buffer, count);
    log::trace!(
        "syscall read(fd:{fd} buf:{buffer:#x} count:{count}) ={}",
        result as i64
    );
    call.set_return(emu, result);
}

pub(crate) fn read_fd(emu: &mut Emu, arch: Arch, fd: u32, buffer: u64, count: u64) -> u64 {
    read_descriptor(emu, arch, u64::from(fd), buffer, count).unwrap_or_else(|| unbacked_read(fd))
}

fn writable_descriptor(fd: u32) -> Result<(), u64> {
    match fd {
        STDOUT_FILENO | STDERR_FILENO => Ok(()),
        STDIN_FILENO => Err(EBADF),
        _ => match descriptor(fd as u64) {
            Some(Descriptor::PipeWrite { .. } | Descriptor::Event { .. }) => Ok(()),
            Some(Descriptor::PipeRead { .. }) => Err(EBADF),
            None => Err(descriptor_error(fd)),
        },
    }
}

fn accessible(emu: &Emu, arch: Arch, address: u64, size: usize, permission: Permission) -> bool {
    size == 0
        || (address
            .checked_add(size as u64 - 1)
            .is_some_and(|last| arch.is_64bits() || last <= u32::MAX as u64)
            && emu.maps.can_access_range(address, size, permission))
}

fn writable(emu: &Emu, arch: Arch, address: u64, size: usize) -> bool {
    accessible(emu, arch, address, size, Permission::WRITE)
}

pub(super) fn emit(emu: &mut Emu, fd: u32, bytes: &[u8]) {
    if cfg!(test) && fd == STDOUT_FILENO {
        // Capture the sink in tests without writing to the host terminal.
        emu.emulated_stdout.extend_from_slice(bytes);
    }
    if !cfg!(test) {
        if fd == STDOUT_FILENO {
            let _ = std::io::stdout().write_all(bytes);
            let _ = std::io::stdout().flush();
        } else {
            let _ = std::io::stderr().write_all(bytes);
            let _ = std::io::stderr().flush();
        }
    }
}

pub(super) fn read_bytes(
    emu: &Emu,
    arch: Arch,
    address: u64,
    size: usize,
) -> Result<Cow<'_, [u8]>, u64> {
    if size == 0 {
        return Ok(Cow::Borrowed(&[]));
    }
    if !accessible(emu, arch, address, size, Permission::READ) {
        return Err(EFAULT);
    }
    if let Some(map) = emu.maps.get_mem_by_addr(address)
        && map.inside(address + size as u64 - 1)
    {
        return emu.maps.read_bytes_option(address, size).ok_or(EFAULT);
    }
    // Maps' borrowed fast path requires a single map. Adjacent readable maps are valid too.
    (0..size)
        .map(|offset| emu.maps.read_byte(address + offset as u64).ok_or(EFAULT))
        .collect::<Result<Vec<_>, _>>()
        .map(Cow::Owned)
}

fn gather(emu: &Emu, arch: Arch, buffers: &[(u64, u64)]) -> Result<Vec<u8>, u64> {
    let mut bytes = Vec::new();
    for &(address, count) in buffers {
        let size = count.min((WRITE_CHUNK_BYTES - bytes.len()) as u64) as usize;
        if size == 0 {
            continue;
        }
        let data = match read_bytes(emu, arch, address, size) {
            Ok(data) => data,
            Err(error) => {
                return if bytes.is_empty() {
                    Err(error)
                } else {
                    Ok(bytes)
                };
            }
        };
        bytes.extend_from_slice(&data);
        if bytes.len() == WRITE_CHUNK_BYTES {
            break;
        }
    }
    Ok(bytes)
}

fn write_event(
    emu: &Emu,
    arch: Arch,
    fd: u64,
    flags: u32,
    address: u64,
    count: u64,
) -> Result<u64, u64> {
    if count != EVENT_BYTES as u64 {
        return Err(EINVAL);
    }
    let bytes = read_bytes(emu, arch, address, EVENT_BYTES)?;
    let mut word = [0; EVENT_BYTES];
    word.copy_from_slice(&bytes);
    let value = u64::from_le_bytes(word);
    if value == u64::MAX {
        return Err(EINVAL);
    }
    let next = event_count(fd)?
        .checked_add(value)
        .filter(|value| *value <= EVENT_MAX)
        .ok_or_else(|| would_block(flags))?;
    helper::handler_put_bytes(fd, &next.to_le_bytes());
    Ok(EVENT_BYTES as u64)
}

fn write_events(
    emu: &Emu,
    arch: Arch,
    fd: u64,
    flags: u32,
    buffers: &[(u64, u64)],
) -> Result<u64, u64> {
    // eventfd has a scalar write callback: writev invokes it once per vector.
    let mut written = 0;
    for &(address, count) in buffers {
        match write_event(emu, arch, fd, flags, address, count) {
            Ok(size) => written += size,
            Err(error) => {
                return if written == 0 {
                    Err(error)
                } else {
                    Ok(written)
                };
            }
        }
    }
    Ok(written)
}

pub(crate) fn write_buffers(
    emu: &mut Emu,
    arch: Arch,
    fd: u32,
    buffers: &[(u64, u64)],
    output: fn(&mut Emu, u32, &[u8]),
) -> Result<u64, u64> {
    writable_descriptor(fd)?;
    if let Some(Descriptor::Event { flags }) = descriptor(fd as u64) {
        return write_events(emu, arch, fd as u64, flags, buffers);
    }
    let bytes = gather(emu, arch, buffers)?;
    match descriptor(fd as u64) {
        Some(Descriptor::PipeWrite { reader, flags }) => {
            if bytes.is_empty() {
                return Ok(0);
            }
            if !matches!(descriptor(reader), Some(Descriptor::PipeRead { .. })) {
                return Err(EPIPE);
            }
            let mut queue = helper::handler_get_bytes(reader);
            let capacity = PIPE_CAPACITY.saturating_sub(queue.len());
            if capacity == 0 || (bytes.len() <= PIPE_BUF && bytes.len() > capacity) {
                return Err(would_block(flags));
            }
            let size = bytes.len().min(capacity);
            queue.extend_from_slice(&bytes[..size]);
            helper::handler_put_bytes(reader, &queue);
            Ok(size as u64)
        }
        _ => {
            let count = bytes.len() as u64;
            output(emu, fd, &bytes);
            Ok(count)
        }
    }
}

pub(super) fn write(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [fd, buffer, count, ..] = call.args_for(emu, arch);
    let result =
        write_buffers(emu, arch, fd as u32, &[(buffer, count)], emit).unwrap_or_else(|error| error);
    log::trace!(
        "syscall write(fd:{fd} buf:{buffer:#x} count:{count}) ={}",
        result as i64
    );
    call.set_return(emu, result);
}

pub(super) fn read_iovecs(
    emu: &Emu,
    arch: Arch,
    address: u64,
    count: u64,
) -> Result<Vec<(u64, u64)>, u64> {
    if count > UIO_MAXIOV as u64 {
        return Err(EINVAL);
    }
    if count == 0 {
        return Ok(Vec::new());
    }
    let word_size = if arch.is_64bits() {
        size_of::<u64>()
    } else {
        size_of::<u32>()
    };
    let entry_size = IOVEC_FIELDS * word_size;
    let size = count as usize * entry_size;
    let data = read_bytes(emu, arch, address, size)?;
    let read_word = |bytes: &[u8]| {
        let mut word = [0; size_of::<u64>()];
        word[..bytes.len()].copy_from_slice(bytes);
        u64::from_le_bytes(word)
    };
    let mut buffers = Vec::with_capacity(count as usize);
    let mut total = 0u64;
    let max_size = if arch.is_64bits() {
        i64::MAX as u64
    } else {
        i32::MAX as u64
    };
    for entry in data.chunks_exact(entry_size) {
        let buffer = read_word(&entry[..word_size]);
        let len = read_word(&entry[word_size..]);
        total = total
            .checked_add(len)
            .filter(|total| *total <= max_size)
            .ok_or(EINVAL)?;
        buffers.push((buffer, len));
    }
    Ok(buffers)
}

pub(super) fn writev(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [fd, address, count, ..] = call.args_for(emu, arch);
    let result = writable_descriptor(fd as u32)
        .and_then(|()| read_iovecs(emu, arch, address, count))
        .and_then(|buffers| {
            if buffers.iter().all(|(_, count)| *count == 0) {
                Ok(0)
            } else {
                write_buffers(emu, arch, fd as u32, &buffers, emit)
            }
        })
        .unwrap_or_else(|error| error);
    log::trace!(
        "syscall writev(fd:{fd} iov:{address:#x} count:{count}) ={}",
        result as i64
    );
    call.set_return(emu, result);
}

pub(super) fn fcntl(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [fd, command, argument, ..] = call.args_for(emu, arch);
    let fd = fd as u32;
    let flags = match fd {
        STDIN_FILENO => Some((0, O_RDONLY)),
        STDOUT_FILENO | STDERR_FILENO => Some((0, O_WRONLY)),
        _ => descriptor(fd as u64).map(|kind| {
            let descriptor_flags = if kind.flags() & O_CLOEXEC != 0 {
                FD_CLOEXEC
            } else {
                0
            };
            let status_flags = kind.access() | (kind.flags() & O_NONBLOCK) as u64;
            (descriptor_flags, status_flags)
        }),
    };
    let result = match (command as u32, flags) {
        (F_GETFD, Some((current, _))) => current,
        (F_GETFL, Some((_, status))) => status,
        // Go clears close-on-exec on inherited stdin before exec. An unchanged flag needs no mutation.
        (F_SETFD, Some((current, _))) if argument & FD_CLOEXEC == current => 0,
        (_, Some(_)) => ENOSYS,
        (_, None) => descriptor_error(fd),
    };
    log::trace!("syscall fcntl(fd:{fd} cmd:{command}) ={}", result as i64);
    call.set_return(emu, result);
}
