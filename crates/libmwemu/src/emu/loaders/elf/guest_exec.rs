use goblin::elf::{
    Elf,
    header::{EM_386, EM_X86_64, ET_DYN, ET_EXEC},
    program_header::{PF_R, PF_W, PF_X, PT_INTERP, PT_LOAD},
};
use rs_header::elf::elf64::Elf64;

use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::syscall::linux::constants::elf::{ELF64_DYN_BASE, LD_BASE};
use crate::syscall::linux::constants::errors::{E2BIG, ENOEXEC, ENOMEM};

pub(crate) const MAX_EXEC_BYTES: u64 = 128 << 20;
const AMD64_TASK_SIZE: u64 = 1 << 47;
const EXEC_STACK_SIZE: u64 = 1 << 20;

pub(crate) fn exec_layout(
    elf: &Elf<'_>,
    file_len: usize,
    kind: u16,
    machine: u16,
    task_size: u64,
) -> Result<(u64, u64, u64), u64> {
    if !matches!(kind, ET_EXEC | ET_DYN)
        || elf.is_64 != (machine != EM_386)
        || !elf.little_endian
        || elf.header.e_machine != machine
        || elf.header.e_type != kind
    {
        return Err(ENOEXEC);
    }
    let mut size = 0u64;
    let mut end = 0;
    let mut alignment = PAGE_SIZE;
    let mut valid_entry = false;
    for p in elf.program_headers.iter().filter(|p| p.p_type == PT_LOAD) {
        let last = p.p_vaddr.checked_add(p.p_memsz).ok_or(ENOEXEC)?;
        if last > task_size
            || p.p_filesz > p.p_memsz
            || p.p_offset
                .checked_add(p.p_filesz)
                .is_none_or(|end| end > file_len as u64)
            || p.p_align > 1
                && (!p.p_align.is_power_of_two() || p.p_offset % p.p_align != p.p_vaddr % p.p_align)
        {
            return Err(ENOEXEC);
        }
        if p.p_memsz == 0 {
            continue;
        }
        if kind == ET_EXEC && p.p_vaddr < PAGE_SIZE {
            return Err(ENOEXEC);
        }
        valid_entry |= p.p_flags & PF_X != 0 && (p.p_vaddr..last).contains(&elf.entry);
        let page_end = last.next_multiple_of(PAGE_SIZE);
        end = end.max(page_end);
        size = size
            .checked_add(page_end - (p.p_vaddr & !(PAGE_SIZE - 1)))
            .ok_or(ENOMEM)?;
        alignment = alignment.max(p.p_align);
    }
    if !valid_entry || size > MAX_EXEC_BYTES {
        return Err(ENOEXEC);
    }
    Ok((size, end, alignment))
}

fn map_segments(
    maps: &mut Maps,
    name: &str,
    elf: &Elf<'_>,
    bytes: &[u8],
    bias: u64,
) -> Result<(), u64> {
    for (index, p) in elf
        .program_headers
        .iter()
        .enumerate()
        .filter(|(_, p)| p.p_type == PT_LOAD && p.p_memsz > 0)
    {
        let page_offset = p.p_vaddr % PAGE_SIZE;
        let start = bias + p.p_vaddr - page_offset;
        let size = (page_offset + p.p_memsz).next_multiple_of(PAGE_SIZE);
        let perm = Permission::from_flags(
            p.p_flags & PF_R != 0,
            p.p_flags & PF_W != 0,
            p.p_flags & PF_X != 0,
        );
        // Linux maps each PT_LOAD over any page shared with the previous segment.
        if maps.overlaps(start, size) {
            maps.unmap_range(start, size);
        }
        let map = maps
            .create_map(&format!("{name}.seg{index}"), start, size, perm)
            .map_err(|_| ENOMEM)?;
        if p.p_filesz > 0 {
            if p.p_offset % PAGE_SIZE != page_offset {
                return Err(ENOEXEC);
            }
            let file_start = (p.p_offset - page_offset) as usize;
            let file_end = if p.p_memsz == p.p_filesz {
                // A file-backed last page keeps the file bytes after the segment.
                (file_start + size as usize).min(bytes.len())
            } else {
                (p.p_offset + p.p_filesz) as usize
            };
            map.force_write_bytes(start, &bytes[file_start..file_end]);
        }
    }
    Ok(())
}

fn program_headers(elf: &Elf<'_>, bias: u64) -> Option<u64> {
    let table_size = elf.header.e_phentsize as u64 * elf.header.e_phnum as u64;
    elf.program_headers.iter().find_map(|p| {
        let offset = elf.header.e_phoff.checked_sub(p.p_offset)?;
        (p.p_type == PT_LOAD && offset.checked_add(table_size)? <= p.p_filesz)
            .then(|| bias + p.p_vaddr + offset)
    })
}

impl Emu {
    /// Build a fresh Linux process image from files resolved by the guest VFS.
    pub(crate) fn load_guest_elf64(
        &mut self,
        path: &str,
        bytes: &[u8],
        elf: &Elf<'_>,
        interpreter: Option<&[u8]>,
    ) -> Result<(), u64> {
        let has_interp = elf.program_headers.iter().any(|p| p.p_type == PT_INTERP);
        if self.cfg.arch != Arch::X86_64 || interpreter.is_some() != has_interp {
            return Err(ENOEXEC);
        }
        let kind = elf.header.e_type;
        let (size, span, align) = exec_layout(elf, bytes.len(), kind, EM_X86_64, AMD64_TASK_SIZE)?;
        let bias = if kind == ET_DYN {
            self.maps
                .find_free_range(span, align, ELF64_DYN_BASE, AMD64_TASK_SIZE)
                .ok_or(ENOMEM)?
        } else {
            0
        };
        let mut main = Elf64::parse(bytes).map_err(|_| ENOEXEC)?;
        main.base = bias;
        map_segments(&mut self.maps, "elf64", elf, bytes, bias)?;
        let entry = bias + elf.entry;
        let phdr = program_headers(elf, bias).unwrap_or(0);
        let (pc, ld_base) = if let Some(raw) = interpreter {
            let ld = Elf::parse(raw).map_err(|_| ENOEXEC)?;
            let (ld_size, ld_span, ld_align) =
                exec_layout(&ld, raw.len(), ET_DYN, EM_X86_64, AMD64_TASK_SIZE)?;
            if ld_size > MAX_EXEC_BYTES - size
                || phdr == 0
                || ld.program_headers.iter().any(|p| p.p_type == PT_INTERP)
            {
                return Err(ENOEXEC);
            }
            let base = self
                .maps
                .find_free_range(ld_span, ld_align, LD_BASE, AMD64_TASK_SIZE)
                .ok_or(ENOMEM)?;
            map_segments(&mut self.maps, "ld-linux", &ld, raw, base)?;
            (base + ld.entry, base)
        } else {
            (entry, 0)
        };
        let stack = self
            .maps
            .find_free_range_top_down(
                EXEC_STACK_SIZE,
                PAGE_SIZE,
                PAGE_SIZE,
                AMD64_TASK_SIZE - PAGE_SIZE,
            )
            .ok_or(ENOMEM)?;
        self.maps
            .create_map("stack", stack, EXEC_STACK_SIZE, Permission::READ_WRITE)
            .map_err(|_| ENOMEM)?;
        if self
            .maps
            .get_mem_by_addr(pc)
            .is_none_or(|map| !map.permission_at(pc).can_execute())
        {
            return Err(ENOEXEC);
        }
        self.os = OperatingSystem::Linux;
        self.set_sp(stack + EXEC_STACK_SIZE);
        self.write_linux_exec_stack(
            entry,
            phdr,
            elf.header.e_phentsize,
            elf.header.e_phnum,
            ld_base,
            Some(path),
        )
        .ok_or(E2BIG)?;
        self.set_pc(pc);
        self.ld_bootstrap = interpreter.is_some();
        self.elf64 = Some(main.into());
        Ok(())
    }
}
