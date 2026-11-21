use crate::api::unix::errno_address;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::threading::process::INITIAL_PID;

const DARWIN_UID: u32 = 501;
const DARWIN_GID: u32 = 20;
const EBADF: u32 = 9;
const ENOMEM: u32 = 12;
const EFAULT: u32 = 14;
const EINVAL: u32 = 22;
const ENOTTY: u32 = 25;
pub(super) const ENOSYS: u32 = 78;
const MAP_SHARED: u32 = 1;
const MAP_PRIVATE: u32 = 2;
const MAP_ANON: u32 = 0x1000;
const PROT_MASK: u32 = 7;
const TIOCGWINSZ: u64 = 0x4008_7468;
const TIOCGPGRP: u64 = 0x4004_7477;

pub(crate) type BsdResult = Result<u64, u32>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Operation {
    Read,
    Write,
    Open,
    Close,
    Ioctl,
    Mmap,
    Munmap,
    Mprotect,
    Issetugid,
    Getuid,
    Getgid,
    Getgroups,
    Getpid,
    Getppid,
}

impl Operation {
    pub(super) fn from_number(number: u64) -> Option<Self> {
        Some(match number {
            3 => Self::Read,
            4 => Self::Write,
            5 => Self::Open,
            6 => Self::Close,
            20 => Self::Getpid,
            24 | 25 => Self::Getuid,
            39 => Self::Getppid,
            43 | 47 => Self::Getgid,
            79 => Self::Getgroups,
            54 => Self::Ioctl,
            73 => Self::Munmap,
            74 => Self::Mprotect,
            197 => Self::Mmap,
            327 => Self::Issetugid,
            _ => return None,
        })
    }

    pub(crate) fn run(self, emu: &mut Emu, args: [u64; 6]) -> BsdResult {
        let [a, b, c, _, _, _] = args;
        log::trace!("Darwin {self:?} {args:x?}");
        match self {
            Self::Write => write(emu, a as i32, b, c),
            Self::Read if a as i32 == 0 => Ok(0),
            Self::Read => Err(EBADF),
            Self::Open if !emu.maps.can_access_range(a, 1, Permission::READ) => Err(EFAULT),
            Self::Open => Err(ENOSYS),
            Self::Close if !(0..=2).contains(&(a as i32)) => Err(EBADF),
            Self::Close => Err(ENOSYS),
            Self::Ioctl => ioctl(emu, a as i32, b, c),
            Self::Mmap => mmap(emu, args),
            Self::Mprotect => change_mapping(emu, a, b, Some(c as u32)),
            Self::Munmap => change_mapping(emu, a, b, None),
            Self::Issetugid => Ok(0),
            Self::Getpid => Ok(emu.processes.pid),
            Self::Getppid => Ok(emu.processes.ppid),
            Self::Getuid => Ok(DARWIN_UID.into()),
            Self::Getgid => Ok(DARWIN_GID.into()),
            Self::Getgroups => match a as i32 {
                n if n < 0 => Err(EINVAL),
                0 => Ok(1),
                _ if b != 0 && emu.maps.write_dword(b, DARWIN_GID) => Ok(1),
                _ => Err(EFAULT),
            },
        }
    }
}

pub(crate) fn libc_result(emu: &mut Emu, result: BsdResult) -> u64 {
    result.unwrap_or_else(|error| {
        if let Some(cell) = errno_address(emu) {
            emu.maps.write_dword(cell, error);
        }
        u64::MAX
    })
}

fn write(emu: &mut Emu, fd: i32, address: u64, count: u64) -> BsdResult {
    if !matches!(fd, 1 | 2) {
        return Err(EBADF);
    }
    if count > i32::MAX as u64 {
        return Err(EINVAL);
    }
    if !emu
        .maps
        .can_access_range(address, count as usize, Permission::READ)
    {
        return Err(EFAULT);
    }
    let end = address + count;
    let mut cursor = address;
    while cursor < end {
        let map = emu.maps.get_mem_by_addr(cursor).ok_or(EFAULT)?;
        let chunk = (map.get_bottom().min(end) - cursor) as usize;
        let bytes = emu.maps.read_bytes_option(cursor, chunk).ok_or(EFAULT)?;
        if fd == 1 {
            emu.emulated_stdout.extend_from_slice(&bytes);
        } else {
            log::info!("stderr: {}", String::from_utf8_lossy(&bytes));
        }
        cursor += chunk as u64;
    }
    Ok(count)
}

pub(crate) fn ioctl_has_output(request: u64) -> bool {
    matches!(request, TIOCGWINSZ | TIOCGPGRP)
}

fn ioctl(emu: &mut Emu, fd: i32, request: u64, out: u64) -> BsdResult {
    if !(0..=2).contains(&fd) {
        return Err(EBADF);
    }
    let written = match request {
        TIOCGWINSZ => emu.maps.write_bytes(out, &[24, 0, 80, 0, 0, 0, 0, 0]),
        TIOCGPGRP => emu.maps.write_dword(out, INITIAL_PID as u32),
        _ => return Err(ENOTTY),
    };
    written.then_some(0).ok_or(EFAULT)
}

fn page_length(size: u64) -> Result<u64, u32> {
    if size == 0 {
        return Err(EINVAL);
    }
    size.checked_next_multiple_of(PAGE_SIZE).ok_or(ENOMEM)
}

fn mmap(emu: &mut Emu, [hint, size, protection, flags, fd, offset]: [u64; 6]) -> BsdResult {
    let length = page_length(size)?;
    let flags = flags as u32;
    if protection as u32 & !PROT_MASK != 0
        || !matches!(flags & (MAP_SHARED | MAP_PRIVATE), MAP_SHARED | MAP_PRIVATE)
    {
        return Err(EINVAL);
    }
    if flags & MAP_ANON == 0 || flags & !(MAP_ANON | MAP_SHARED | MAP_PRIVATE) != 0 {
        return Err(ENOSYS);
    }
    if fd as i32 != -1 || offset != 0 {
        return Err(EINVAL);
    }
    if length > emu.maps.max_alloc_size {
        return Err(ENOMEM);
    }
    let base = if hint != 0
        && hint.is_multiple_of(PAGE_SIZE)
        && hint.checked_add(length).is_some()
        && emu.maps.is_range_free(hint, length)
    {
        hint
    } else {
        emu.maps.alloc(length).ok_or(ENOMEM)?
    };
    emu.maps
        .create_map(
            &format!("mmap_{base:x}"),
            base,
            length,
            Permission::from_bits(protection as u8),
        )
        .map_err(|_| ENOMEM)?;
    Ok(base)
}

fn change_mapping(emu: &mut Emu, address: u64, size: u64, protection: Option<u32>) -> BsdResult {
    if !address.is_multiple_of(PAGE_SIZE) || protection.is_some_and(|bits| bits & !PROT_MASK != 0) {
        return Err(EINVAL);
    }
    let length = page_length(size)?;
    let end = address.checked_add(length).ok_or(EINVAL)?;
    if let Some(bits) = protection {
        if !emu
            .maps
            .can_access_range(address, length as usize, Permission::NONE)
        {
            return Err(ENOMEM);
        }
        let mut cursor = address;
        while cursor < end {
            let map = emu.maps.get_mem_by_addr_mut(cursor).ok_or(ENOMEM)?;
            let stop = end.min(map.get_bottom());
            if !map.set_permission_range(cursor, stop - cursor, Permission::from_bits(bits as u8)) {
                return Err(EINVAL);
            }
            cursor = stop;
        }
        emu.maps.note_code_write(address, length);
    } else {
        emu.maps.unmap_range(address, length);
    }
    Ok(0)
}
