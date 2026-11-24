//! ELF32 (Linux x86) loading.

use crate::syscall::linux::constants::elf::ELF32_DYN_BASE;
use crate::syscall::linux::constants::errors::{E2BIG, ENOEXEC, ENOMEM};
use goblin::elf::{
    Elf,
    header::{EM_386, ET_DYN, ET_EXEC},
    program_header::{PT_INTERP, PT_LOAD},
    reloc::R_386_RELATIVE,
};
use rs_header::elf::elf32::Elf32 as Elf32Image;

use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;

use super::elf::{MAX_EXEC_BYTES, exec_layout};
use super::format::{FormatLoader, LoadInput, apply_entry_point_override, self_relocating_pie};

/// Loader for a 32-bit Linux ELF executable.
pub(crate) struct Elf32;

impl FormatLoader for Elf32 {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        let image = Self::map_main(emu, input.bytes)?;
        let sp = Self::allocate_stack(emu, INITIAL_STACK_SIZE)?;
        emu.set_sp(sp);
        let entry = Self::entry(&image);
        emu.set_pc(entry);
        emu.write_linux_stack_layout(
            entry,
            Self::phdr(&image),
            image.elf_hdr.e_phentsize,
            image.elf_hdr.e_phnum,
        );
        emu.elf32 = Some(image.into());
        apply_entry_point_override(emu);
        Ok(())
    }
}

impl Elf32 {
    fn map_main(emu: &mut Emu, bytes: &[u8]) -> Result<Elf32Image, MwemuError> {
        emu.os = OperatingSystem::Linux;
        emu.cfg.arch = Arch::X86;

        log::trace!("elf32 detected.");
        let mut elf32 = match Elf32Image::parse_linux(bytes) {
            Ok(e) => e,
            Err(err) => {
                log::error!("elf32 parse failed: {err}");
                return Err(MwemuError::new(&format!("elf32 parse failed: {err}")));
            }
        };
        elf32.load(&mut emu.maps);

        // REL stores its addend in the guest slot. A local relative relocation
        // needs no symbol provider, including for a library loaded on its own.
        if elf32.base() != 0 && elf32.is_dynamic() {
            let parsed = Elf::parse(bytes)
                .map_err(|err| MwemuError::new(&format!("ELF32 dynamic table: {err}")))?;
            if !self_relocating_pie(&parsed) {
                for rel in &parsed.dynrels {
                    if rel.r_type != R_386_RELATIVE {
                        continue;
                    }
                    let slot = elf32.base() + rel.r_offset;
                    let value = emu
                        .maps
                        .read_dword(slot)
                        .ok_or_else(|| MwemuError::new("unmapped ELF32 relative relocation"))?;
                    if let Some(map) = emu.maps.get_mem_by_addr_mut(slot) {
                        map.force_write_dword(slot, value.wrapping_add(elf32.base() as u32));
                    }
                }
            }
        }
        Ok(elf32)
    }

    fn allocate_stack(emu: &mut Emu, size: u64) -> Result<u64, MwemuError> {
        let stack = emu.alloc("stack", size, Permission::READ_WRITE);
        (stack != 0)
            .then_some(stack + size / 2)
            .ok_or_else(|| MwemuError::new("cannot allocate ELF32 stack"))
    }

    fn entry(image: &Elf32Image) -> u64 {
        if image.elf_hdr.e_entry == 0 {
            0
        } else {
            image.base() + image.elf_hdr.e_entry as u64
        }
    }

    fn phdr(image: &Elf32Image) -> u64 {
        let phoff = image.elf_hdr.e_phoff;
        let size = image.elf_hdr.e_phnum as u64 * image.elf_hdr.e_phentsize as u64;
        image
            .elf_phdr
            .iter()
            .find_map(|ph| {
                let offset = phoff.checked_sub(ph.p_offset)?;
                (ph.p_type == PT_LOAD && offset as u64 + size <= ph.p_filesz as u64)
                    .then(|| image.base() + ph.p_vaddr as u64 + offset as u64)
            })
            .unwrap_or(0)
    }
}

const INITIAL_STACK_SIZE: u64 = 0x30000;
const EXEC_STACK_SIZE: u64 = 1 << 20;
const I386_TASK_SIZE: u64 = 0xc000_0000;
const INTERPRETER_MMAP_START: u64 = 0x4000_0000;

impl Emu {
    /// Prepare the new guest address space. All images come from the guest VFS.
    pub(crate) fn load_guest_elf32(
        &mut self,
        path: &str,
        bytes: &[u8],
        elf: &Elf<'_>,
        interpreter: Option<&[u8]>,
    ) -> Result<(), u64> {
        let kind = elf.header.e_type;
        let (main_size, span, align) = exec_layout(elf, bytes.len(), kind, EM_386, I386_TASK_SIZE)?;
        let main_bias = match kind {
            ET_EXEC => 0,
            ET_DYN => self
                .maps
                .find_free_range(span, align, ELF32_DYN_BASE, I386_TASK_SIZE)
                .ok_or(ENOMEM)?,
            _ => return Err(ENOEXEC),
        };
        // The guest interpreter or static PIE startup code applies its own relocations.
        let mut main = Elf32Image::parse_linux(bytes).map_err(|_| ENOEXEC)?;
        main.load_at(&mut self.maps, main_bias as u32, "elf32_seg");
        self.os = OperatingSystem::Linux;
        self.cfg.arch = Arch::X86;
        let entry = Elf32::entry(&main);
        let phdr = Elf32::phdr(&main);
        let mut pc = entry;
        let mut base = 0;
        if let Some(raw) = interpreter {
            let parsed = Elf::parse(raw).map_err(|_| ENOEXEC)?;
            let (size, span, align) =
                exec_layout(&parsed, raw.len(), ET_DYN, EM_386, I386_TASK_SIZE)?;
            if size > MAX_EXEC_BYTES - main_size
                || phdr == 0
                || parsed.program_headers.iter().any(|p| p.p_type == PT_INTERP)
            {
                return Err(ENOEXEC);
            }
            base = self
                .maps
                .find_free_range(span, align, INTERPRETER_MMAP_START, I386_TASK_SIZE)
                .ok_or(ENOMEM)?;
            let mut image = Elf32Image::parse_linux(raw).map_err(|_| ENOEXEC)?;
            image.load_at(
                &mut self.maps,
                u32::try_from(base).map_err(|_| ENOMEM)?,
                "ld32_seg",
            );
            pc = base + parsed.entry;
        }
        if self
            .maps
            .get_mem_by_addr(pc)
            .is_none_or(|m| !m.permission_at(pc).can_execute())
        {
            return Err(ENOEXEC);
        }
        // execve bounds argv and envp independently. Leave enough room for both together.
        let sp = Elf32::allocate_stack(self, EXEC_STACK_SIZE).map_err(|_| ENOMEM)?;
        self.set_sp(sp);
        self.write_linux_exec_stack(
            entry,
            phdr,
            main.elf_hdr.e_phentsize,
            main.elf_hdr.e_phnum,
            base,
            Some(path),
        )
        .ok_or(E2BIG)?;
        self.set_pc(pc);
        self.elf32 = Some(main.into());
        Ok(())
    }
}
