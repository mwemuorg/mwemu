use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::syscall::linux::constants::elf::PT_LOAD;
use crate::syscall::linux::constants::errors::{EBADF, EEXIST, EFAULT, EINVAL, EIO, ENOMEM};
use crate::winapi::helper;

const MAP_TYPE: u64 = 0x0f;
const MAP_SHARED: u64 = 1;
const MAP_PRIVATE: u64 = 2;
const MAP_SHARED_VALIDATE: u64 = 3;
const MAP_FIXED: u64 = 0x10;
const MAP_ANONYMOUS: u64 = 0x20;
const MAP_FIXED_NOREPLACE: u64 = 0x100000;
const MAX_MAPPING: u64 = 64 << 20;
const MAX_RESERVATION: u64 = 4 << 30;
const I386_MMAP_MIN: u64 = 0x1000_0000;
const I386_TASK_SIZE: u64 = 0xc000_0000;
const ELF64_FALLBACK_BRK: u64 = 0x6000_0000_0000;

fn protection(prot: u64) -> Result<Permission, u64> {
    if prot & !(Permission::READ_WRITE_EXECUTE.bits() as u64) != 0 {
        return Err(EINVAL);
    }
    Ok(Permission::from_bits(prot as u8))
}

fn size_in_pages(len: u64) -> Result<u64, u64> {
    len.checked_next_multiple_of(PAGE_SIZE).ok_or(ENOMEM)
}

pub(super) fn mmap(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let mut args = call.args_for(emu, arch);
    if arch == Arch::X86 {
        // mmap2's final argument counts 4 KiB pages; both other ABIs use bytes.
        args[5] *= PAGE_SIZE;
    }
    finish_mmap(emu, arch, args);
}

pub(super) fn old_mmap(emu: &mut Emu) {
    let call = LinuxSyscall;
    let ptr = call.args_for(emu, Arch::X86)[0];
    let mut args = [0; 6];
    for (index, value) in args.iter_mut().enumerate() {
        let Some(arg) = emu.maps.read_dword(ptr + index as u64 * 4) else {
            call.set_return(emu, EFAULT);
            return;
        };
        *value = arg as u64;
    }
    finish_mmap(emu, Arch::X86, args);
}

fn finish_mmap(emu: &mut Emu, arch: Arch, args: [u64; 6]) {
    let call = LinuxSyscall;
    let value = map(emu, arch, args).unwrap_or_else(|errno| errno);
    let [addr, len, prot, flags, fd, offset] = args;
    log::trace!(
        "mmap(addr={addr:#x} len={len:#x} prot={prot:#x} flags={flags:#x} fd={} offset={offset:#x}) = {value:#x}",
        fd as i32
    );
    call.set_return(emu, value);
}

// The legacy x86-64 loader opens backing files through its host library adapter.
fn library_bytes(fd: u64, offset: u64, len: u64) -> Result<Vec<u8>, u64> {
    if !helper::handler_exist(fd) {
        return Err(EBADF);
    }
    let path = helper::handler_get_uri(fd);
    if !path.contains(".so") {
        return Err(EBADF);
    }
    let mut file = File::open(path).map_err(|_| EBADF)?;
    file.seek(SeekFrom::Start(offset)).map_err(|_| EIO)?;
    let mut data = Vec::new();
    file.take(len).read_to_end(&mut data).map_err(|_| EIO)?;
    Ok(data)
}

pub(crate) fn map(emu: &mut Emu, arch: Arch, args: [u64; 6]) -> Result<u64, u64> {
    let [hint, len, prot, flags, fd, offset] = args;
    let fixed = flags & (MAP_FIXED | MAP_FIXED_NOREPLACE) != 0;
    let perm = protection(prot)?;
    if len == 0
        || (fixed && !hint.is_multiple_of(PAGE_SIZE))
        || !matches!(
            flags & MAP_TYPE,
            MAP_PRIVATE | MAP_SHARED | MAP_SHARED_VALIDATE
        )
        || !offset.is_multiple_of(PAGE_SIZE)
    {
        return Err(EINVAL);
    }
    let size = size_in_pages(len)?;
    let reservation = prot == 0 && flags & MAP_ANONYMOUS != 0;
    if size
        > if reservation {
            MAX_RESERVATION
        } else {
            MAX_MAPPING
        }
    {
        return Err(ENOMEM);
    }
    let limit = if arch == Arch::X86 {
        1u64 << 32
    } else {
        u64::MAX
    };
    let hint_fits = hint != 0
        && hint.is_multiple_of(PAGE_SIZE)
        && hint.checked_add(size).is_some_and(|end| end <= limit);
    let base = if fixed {
        if !hint_fits {
            return Err(ENOMEM);
        }
        if flags & MAP_FIXED_NOREPLACE != 0 && !emu.maps.is_range_free(hint, size) {
            return Err(EEXIST);
        }
        hint
    } else if hint_fits && emu.maps.is_range_free(hint, size) {
        hint
    } else if arch == Arch::X86 {
        emu.maps
            .find_free_range(size, PAGE_SIZE, I386_MMAP_MIN, I386_TASK_SIZE)
            .ok_or(ENOMEM)?
    } else {
        emu.maps.lib64_reserve(size).ok_or(ENOMEM)?
    };

    // Read before altering a fixed mapping, so invalid descriptors preserve the old bytes.
    let mut shared = None;
    let data = if flags & MAP_ANONYMOUS != 0 {
        Vec::new()
    } else if let Some(files) = &mut emu.linux_files {
        files.sync_mappings(&emu.maps);
        if flags & MAP_TYPE != MAP_PRIVATE {
            let end = offset.checked_add(size).ok_or(ENOMEM)?;
            shared = Some(files.mapping_store(&mut emu.maps, fd as u32, end, perm.can_write())?);
            Vec::new()
        } else {
            files.private_mapping_bytes(fd as u32, offset, size as usize)?
        }
    } else if arch == Arch::X86_64 {
        library_bytes(fd, offset, size)?
    } else {
        return Err(EBADF);
    };
    if fixed {
        emu.maps.unmap_range(base, size);
        emu.instruction_cache().invalidate_all();
    }
    let name = emu.maps.unique_map_name(&format!("mmap_{base:x}"));
    if let Some(store) = shared {
        emu.maps
            .create_shared_view(&name, base, size, store, offset, perm)
            .map_err(|_| ENOMEM)?;
    } else {
        let map = emu
            .maps
            .create_map(&name, base, size, perm)
            .map_err(|_| ENOMEM)?;
        if !data.is_empty() {
            map.force_write_bytes(base, &data);
        }
    }
    if fixed && let Some(files) = &mut emu.linux_files {
        files.reap_mappings(&mut emu.maps);
    }
    Ok(base)
}

pub(super) fn munmap(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [addr, len, ..] = call.args_for(emu, arch);
    let result = unmap(emu, addr, len).unwrap_or_else(|errno| errno);
    call.set_return(emu, result);
}

pub(crate) fn unmap(emu: &mut Emu, addr: u64, len: u64) -> Result<u64, u64> {
    if !addr.is_multiple_of(PAGE_SIZE) || len == 0 {
        Err(EINVAL)
    } else {
        size_in_pages(len).and_then(|size| {
            addr.checked_add(size).ok_or(EINVAL)?;
            emu.maps.unmap_range(addr, size);
            if let Some(files) = &mut emu.linux_files {
                files.reap_mappings(&mut emu.maps);
            }
            emu.instruction_cache().invalidate_all();
            Ok(0)
        })
    }
}

pub(super) fn mprotect(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let [addr, len, prot, ..] = call.args_for(emu, arch);
    let result = protect(emu, addr, len, prot).unwrap_or_else(|errno| errno);
    call.set_return(emu, result);
}

pub(crate) fn protect(emu: &mut Emu, addr: u64, len: u64, prot: u64) -> Result<u64, u64> {
    if !addr.is_multiple_of(PAGE_SIZE) {
        return Err(EINVAL);
    }
    let perm = protection(prot)?;
    let size = size_in_pages(len)?;
    let end = addr.checked_add(size).ok_or(ENOMEM)?;
    if !emu.maps.can_access_range(
        addr,
        usize::try_from(size).map_err(|_| ENOMEM)?,
        Permission::NONE,
    ) {
        return Err(ENOMEM);
    }
    let mut cursor = addr;
    while cursor < end {
        let map = emu.maps.get_mem_by_addr_mut(cursor).ok_or(ENOMEM)?;
        let next = end.min(map.get_bottom());
        if !map.set_permission_range(cursor, next - cursor, perm) {
            return Err(ENOMEM);
        }
        cursor = next;
    }
    emu.instruction_cache().invalidate_all();
    Ok(0)
}

pub(super) fn msync(emu: &mut Emu, arch: Arch) {
    const MS_ASYNC: u64 = 1;
    const MS_INVALIDATE: u64 = 2;
    const MS_SYNC: u64 = 4;
    let call = LinuxSyscall;
    let [addr, len, flags, ..] = call.args_for(emu, arch);
    let result = if !addr.is_multiple_of(PAGE_SIZE)
        || flags & !(MS_ASYNC | MS_INVALIDATE | MS_SYNC) != 0
        || flags & (MS_ASYNC | MS_SYNC) == MS_ASYNC | MS_SYNC
    {
        EINVAL
    } else if size_in_pages(len)
        .ok()
        .and_then(|size| usize::try_from(size).ok())
        .is_none_or(|size| !emu.maps.can_access_range(addr, size, Permission::NONE))
    {
        ENOMEM
    } else {
        if let Some(files) = &mut emu.linux_files {
            files.sync_mappings(&emu.maps);
        }
        0
    };
    call.set_return(emu, result);
}

pub(super) fn brk32(emu: &mut Emu) {
    let call = LinuxSyscall;
    let requested = call.args_for(emu, Arch::X86)[0];
    let start = emu
        .elf32
        .as_ref()
        .and_then(|elf| {
            elf.elf_phdr
                .iter()
                .filter(|ph| ph.p_type == PT_LOAD)
                .map(|ph| elf.base() + ph.p_vaddr as u64 + ph.p_memsz as u64)
                .max()
        })
        .and_then(|end| size_in_pages(end).ok())
        .unwrap_or(I386_MMAP_MIN);
    let result = move_program_break(emu, requested, start, I386_TASK_SIZE);
    call.set_return(emu, result);
}

pub(super) fn brk64(emu: &mut Emu, arch: Arch) {
    let call = LinuxSyscall;
    let requested = call.args_for(emu, arch)[0];
    let start = emu
        .elf64
        .as_ref()
        .and_then(|elf| {
            elf.elf_phdr
                .iter()
                .filter(|ph| ph.p_type == PT_LOAD && ph.p_memsz > 0)
                .filter_map(|ph| {
                    let base = if ph.p_vaddr < elf.base {
                        elf.base.checked_add(ph.p_vaddr)?
                    } else {
                        ph.p_vaddr
                    };
                    base.checked_add(ph.p_memsz)
                })
                .max()
        })
        .and_then(|end| size_in_pages(end).ok())
        .unwrap_or(ELF64_FALLBACK_BRK);
    let result = move_program_break(emu, requested, start, u64::MAX);
    call.set_return(emu, result);
}

fn move_program_break(emu: &mut Emu, requested: u64, start: u64, limit: u64) -> u64 {
    if emu.heap_addr == 0 {
        emu.heap_addr = start;
    }
    let old = emu.heap_addr;
    if requested >= start && requested < limit {
        let (Ok(first), Ok(last)) = (size_in_pages(old), size_in_pages(requested)) else {
            return old;
        };
        if last == first {
            emu.heap_addr = requested;
        } else if last < first {
            emu.maps.unmap_range(last, first - last);
            emu.heap_addr = requested;
        } else if last - start <= MAX_MAPPING {
            let name = emu.maps.unique_map_name(&format!("brk_{first:x}"));
            if emu
                .maps
                .create_map(&name, first, last - first, Permission::READ_WRITE)
                .is_ok()
            {
                emu.heap_addr = requested;
            }
        }
    }
    log::trace!("brk({requested:#x}) = {:#x}", emu.heap_addr);
    emu.heap_addr
}
