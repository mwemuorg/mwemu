//! Loading a binary and setting an emulator up for its operating system.
//!
//! The code is in the OS crates (`mwemu-linux`, `mwemu-macos`, `mwemu-windows`),
//! which sit above this
//! one. These methods keep it reachable as `emu.load_elf64(..)`: each one
//! calls its layer through [`crate::layers`].

use crate::emu::Emu;
use crate::emu::loaders::{MappedPe64, PeLoadInfo};
use crate::err::MwemuError;
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

    /// Initialize the Windows simulation: the stack, the PEB and TEB, the
    /// loader data and the system DLLs. `load_code()` calls this when the
    /// sample is a PE.
    pub fn init_win32(
        &mut self,
        clear_registers: bool,
        clear_flags: bool,
    ) -> Result<(), MwemuError> {
        layers::windows().init(self, clear_registers, clear_flags)
    }

    /// Load and bind a PE32 image. Only the main image owns process metadata,
    /// the initial instruction pointer and TLS state.
    pub fn load_pe32(
        &mut self,
        filename: &str,
        main_image: bool,
        force_base: u32,
    ) -> Result<PeLoadInfo, MwemuError> {
        layers::windows().load_pe32(self, filename, main_image, force_base)
    }

    /// Load and bind a PE64 image. Dependency loading preserves main-image state.
    pub fn load_pe64(
        &mut self,
        filename: &str,
        main_image: bool,
        force_base: u64,
    ) -> Result<PeLoadInfo, MwemuError> {
        layers::windows().load_pe64(self, filename, main_image, force_base)
    }

    /// Map and relocate a PE64 DLL without binding its imports.
    pub fn map_dll_pe64(&mut self, filename: &str) -> Result<MappedPe64, MwemuError> {
        layers::windows().map_dll_pe64(self, filename)
    }

    /// Build and map a stub PE32 DLL exporting `exports`. Returns its base.
    pub fn create_stub_pe32(&mut self, dll_name: &str, exports: &[&str]) -> u32 {
        layers::windows().create_stub_pe32(self, dll_name, exports)
    }

    /// Build and map a stub PE64 DLL exporting `exports`. Returns its base.
    pub fn create_stub_pe64(&mut self, dll_name: &str, exports: &[&str]) -> u64 {
        layers::windows().create_stub_pe64(self, dll_name, exports)
    }
}
