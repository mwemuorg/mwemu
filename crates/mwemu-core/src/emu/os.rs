//! Loading a binary and setting an emulator up for its operating system.
//!
//! The code is in the OS crates (`mwemu-linux`, `mwemu-macos`), which sit above this
//! one. These methods keep it reachable as `emu.load_elf64(..)`: each one
//! calls its layer through [`crate::layers`].

use crate::emu::Emu;
use crate::layers;

impl Emu {
    /// Load the ELF64 executable `filename` and its libraries. `load_code()`
    /// calls this when the sample is an ELF64.
    pub fn load_elf64(&mut self, filename: &str) {
        layers::linux().load_elf64(self, filename);
    }

    /// Initialize the x86-64 Linux simulation. `load_elf64()` calls this.
    pub fn init_linux64(&mut self, dyn_link: bool) {
        layers::linux().init64(self, dyn_link);
    }

    /// Initialize the AArch64 Linux simulation. `load_elf64()` calls this.
    pub fn init_linux64_aarch64(&mut self) {
        layers::linux().init_aarch64(self);
    }

    /// Load the 64-bit Mach-O executable `filename` and its dylibs.
    /// `load_code()` calls this when the sample is a Mach-O.
    pub fn load_macho64(&mut self, filename: &str) {
        layers::macos().load_macho64(self, filename);
    }

    /// Initialize the x86-64 macOS simulation. `load_macho64()` calls this.
    pub fn init_macos64(&mut self) {
        layers::macos().init64(self);
    }

    /// Initialize the AArch64 macOS simulation. `load_macho64()` calls this.
    pub fn init_macos_aarch64(&mut self) {
        layers::macos().init_aarch64(self);
    }
}
