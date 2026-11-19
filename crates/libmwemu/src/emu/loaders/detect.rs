//! Format detection: one job — name the format of an input file.
//!
//! The detectors live together so the loader dispatch and the CLI's
//! pre-flight checks ([`crate::emu::Emu::is_non_windows_file`]) can never
//! disagree about what a file is.

use rs_header::elf::elf32::Elf32;
use rs_header::elf::elf64::Elf64;
use rs_header::pe::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, pe_machine_type,
};

use crate::loaders::macho::macho64::Macho64;

use super::sys;

/// Every input format the loader knows how to map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryFormat {
    /// Linux ELF32 (x86).
    Elf32,
    /// Linux ELF64 (x86_64).
    Elf64X64,
    /// Linux ELF64 (AArch64).
    Elf64Aarch64,
    /// Linux kernel module (ET_REL).
    Elf64Relocatable,
    /// macOS Mach-O (AArch64).
    MachoAarch64,
    /// macOS Mach-O (x86_64).
    MachoX64,
    /// macOS kernel extension (MH_KEXT_BUNDLE).
    MachoKext,
    /// Windows PE32 (x86).
    Pe32,
    /// Windows PE32+ (x86_64).
    Pe64X64,
    /// Windows PE32+ (ARM64).
    Pe64Arm64,
    /// Windows kernel driver (native-subsystem PE32+).
    PeSysDriver,
    /// Raw machine code with no executable header.
    Shellcode,
}

impl BinaryFormat {
    /// Name the format of `bytes`, which were read from `path`.
    ///
    /// `force_shellcode` mirrors `--is_shellcode`: it skips every header
    /// detector and takes the raw-code path.
    pub(crate) fn detect(bytes: &[u8], path: &str, force_shellcode: bool) -> BinaryFormat {
        if force_shellcode {
            return BinaryFormat::Shellcode;
        }

        // A relocatable object has no program headers and no entry point, so
        // it is classified before the executable ELF branches.
        if Elf64::is_elf64_relocatable(bytes) {
            return BinaryFormat::Elf64Relocatable;
        }
        if Elf32::is_elf32(bytes) {
            return BinaryFormat::Elf32;
        }
        if Elf64::is_elf64_aarch64(bytes) {
            return BinaryFormat::Elf64Aarch64;
        }
        // A kext is an arm64 Mach-O without an entry point: it is linked into
        // the emulated kernel rather than run as a process.
        if Macho64::is_macho64_aarch64(path) {
            if Macho64::is_macho64_kext(path) {
                return BinaryFormat::MachoKext;
            }
            return BinaryFormat::MachoAarch64;
        }
        if Macho64::is_macho64_x64(path) {
            return BinaryFormat::MachoX64;
        }
        if Elf64::is_elf64_x64(bytes) {
            return BinaryFormat::Elf64X64;
        }

        // the case for pe file
        match pe_machine_type(bytes) {
            Some(IMAGE_FILE_MACHINE_I386) => BinaryFormat::Pe32,
            Some(IMAGE_FILE_MACHINE_ARM64) => BinaryFormat::Pe64Arm64,
            // A native-subsystem driver imports ntoskrnl: it is linked into
            // the emulated kernel, not run as a user-mode process.
            Some(IMAGE_FILE_MACHINE_AMD64) if sys::is_kernel_driver(bytes) => {
                BinaryFormat::PeSysDriver
            }
            Some(IMAGE_FILE_MACHINE_AMD64) => BinaryFormat::Pe64X64,
            _ => BinaryFormat::Shellcode,
        }
    }

    /// Is this a guest of another OS (Linux or macOS), so it needs none of
    /// the Windows system DLLs?
    pub(crate) fn is_non_windows(self) -> bool {
        matches!(
            self,
            BinaryFormat::Elf32
                | BinaryFormat::Elf64X64
                | BinaryFormat::Elf64Aarch64
                | BinaryFormat::Elf64Relocatable
                | BinaryFormat::MachoAarch64
                | BinaryFormat::MachoX64
                | BinaryFormat::MachoKext
        )
    }
}
