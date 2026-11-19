//! ELF32 (Linux x86) loading.

use rs_header::elf::elf32::Elf32 as Elf32Image;

use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;

use super::format::{FormatLoader, LoadInput, apply_entry_point_override};

/// Loader for a 32-bit Linux ELF executable.
pub(crate) struct Elf32;

impl FormatLoader for Elf32 {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        emu.os = OperatingSystem::Linux;
        emu.cfg.arch = Arch::X86;

        log::trace!("elf32 detected.");
        let mut elf32 = match Elf32Image::parse(input.bytes) {
            Ok(e) => e,
            Err(err) => {
                log::error!("elf32 parse failed: {err}");
                return Err(MwemuError::new(&format!("elf32 parse failed: {err}")));
            }
        };
        elf32.load(&mut emu.maps);

        let stack_sz = 0x30000;
        let stack = emu.alloc("stack", stack_sz, Permission::READ_WRITE);
        emu.regs_mut().rsp = stack + (stack_sz / 2);

        // Set RIP from the ELF32 entry point. Mirrors the ELF64 path in
        // `load_elf64` (which uses `rebase_vaddr(e_entry)` then `set_pc`).
        // Without this `load_code` returns with `rip == 0` and downstream
        // tests like `hello_linux_x86` fail at `assert!(entry != 0, …)`.
        // `Elf32::load` already rebased each `PT_LOAD` segment with the same
        // `base` (ELF32_DYN_BASE for dynamic/PIE binaries, 0 for ET_EXEC),
        // so apply that base to `e_entry` here for parity.
        let entry_raw = elf32.elf_hdr.e_entry as u64;
        let base = elf32.base();
        emu.regs_mut().rip = if entry_raw != 0 && entry_raw < base {
            entry_raw + base
        } else {
            entry_raw
        };

        emu.elf32 = Some(elf32);

        apply_entry_point_override(emu);
        Ok(())
    }
}
