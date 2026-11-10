//! The Windows layer of mwemu: the PE loader, the PEB, TEB and loader data,
//! the set-up of an emulator as a Windows process, the exception dispatch
//! (SEH, VEH, UEF), the NT syscalls, the helpers the API handlers share
//! (`api::windows::common`) and the emulated registry, environment, profile
//! and window state.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`).
//! `libmwemu` registers it for you. The API handlers themselves are
//! `mwemu-winapi32` and `mwemu-winapi64`; the structures and constants are in
//! `mwemu-wintypes`.
//!
//! The modules keep the paths they had inside `mwemu-core`, so the files
//! moved here unchanged. Each module also re-exports what the core has under
//! the same path, so `windows::`, `exception::`, `emu::` and
//! `api::windows::` name everything.
//!
//! The PE loader and the set-up were methods of `Emu`; here they are the
//! crate-private traits `PeLoading`, `PeStubs` and `WindowsInit`,
//! implemented for `Emu`. `Emu::init_win32()`, `load_pe32()`, `load_pe64()`,
//! `map_dll_pe64()`, `create_stub_pe32()` and `create_stub_pe64()` in the
//! core call them through the layer.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
// The crypto helpers round up by hand; mwemu-core allowed that crate-wide.
#![allow(clippy::manual_div_ceil)]

#[macro_use]
extern crate mwemu_core;

use mwemu_core::emu::Emu;
use mwemu_core::emu::loaders::{MappedPe64, PeLoadInfo};
use mwemu_core::err::MwemuError;
use mwemu_core::exception::types::{ExceptionRecord, HandlerKind};
use mwemu_core::layers::{self, WindowsLayer};

use emu::initialization::WindowsInit;
use emu::loaders::pe::PeLoading;
use emu::loaders::pe_stub::PeStubs;

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{arch, config, context, debug, err, kernel, maps, threading};
pub mod emu {
    pub use mwemu_core::emu::*;

    pub mod initialization;
    pub mod loaders {
        pub use mwemu_core::emu::loaders::*;

        pub mod pe;
        pub mod pe_stub;
        pub mod pe_stub_exports;
    }
}
pub mod exception {
    pub use mwemu_core::exception::*;

    pub mod handlers;
    pub mod raise;
    pub mod seh32;
}
pub mod api {
    pub mod windows {
        pub use mwemu_core::api::windows::*;

        pub mod common;
    }
}
pub mod syscall {
    pub mod windows;
}
pub mod windows {
    pub use mwemu_core::windows::*;

    pub mod environment_store;
    pub mod kuser_shared;
    pub mod loader32;
    pub mod loader64;
    pub mod peb;
    pub mod profile_store;
    pub mod registry_store;
    pub mod stolen_prologue;
    pub mod window_table;
}
pub use api::windows as winapi;
pub use syscall::windows::ntapi;

struct Layer;

impl WindowsLayer for Layer {
    fn syscall64(&self, emu: &mut Emu) {
        syscall::windows::syscall64::gateway(emu);
    }
    fn syscall32(&self, emu: &mut Emu) {
        syscall::windows::syscall32::gateway(emu);
    }
    fn ntapi32(&self, syscall: u64, argv: u64, emu: &mut Emu) {
        ntapi::ntapi32::gateway(syscall, argv, emu);
    }
    fn build_syscall_translation_table(&self, emu: &mut Emu) {
        syscall::windows::syscall64::build_syscall_translation_table(emu);
    }
    fn patch_ntdll_load_reason_checks(&self, emu: &mut Emu) {
        syscall::windows::syscall64::memory::patch_ntdll_load_reason_checks(emu);
    }
    fn ntdll_heap_list_walk_fixup(&self, emu: &mut Emu, ins: &iced_x86::Instruction, rip: u64) {
        syscall::windows::syscall64::memory::ntdll_heap_list_walk_fixup(emu, ins, rip);
    }
    fn guess_api_name(&self, emu: &mut Emu, addr: u64, is64: bool) -> String {
        if is64 {
            windows::loader64::guess_api_name(emu, addr)
        } else {
            windows::loader32::guess_api_name(emu, addr as u32)
        }
    }
    fn resolve_api_addr_to_name(&self, emu: &mut Emu, addr: u64, is64: bool) -> String {
        if is64 {
            windows::loader64::resolve_api_addr_to_name(emu, addr)
        } else {
            windows::loader32::resolve_api_addr_to_name(emu, addr)
        }
    }
    fn search_api_name(&self, emu: &mut Emu, name: &str, is64: bool) -> (u64, String, String) {
        if is64 {
            windows::loader64::search_api_name(emu, name)
        } else {
            windows::loader32::search_api_name(emu, name)
        }
    }
    fn resolve_api_name(&self, emu: &mut Emu, name: &str, is64: bool) -> u64 {
        if is64 {
            windows::loader64::resolve_api_name(emu, name)
        } else {
            windows::loader32::resolve_api_name(emu, name)
        }
    }
    fn resolve_api_name_in_module(
        &self,
        emu: &mut Emu,
        module: &str,
        name: &str,
        is64: bool,
    ) -> u64 {
        if is64 {
            windows::loader64::resolve_api_name_in_module(emu, module, name)
        } else {
            windows::loader32::resolve_api_name_in_module(emu, module, name)
        }
    }
    fn load_library(&self, emu: &mut Emu, libname: &str, is64: bool) -> u64 {
        if is64 {
            windows::loader64::load_library(emu, libname)
        } else {
            windows::loader32::load_library(emu, libname)
        }
    }
    fn update_ldr_entry_base(&self, emu: &mut Emu, libname: &str, base: u64, is64: bool) {
        if is64 {
            windows::peb::peb64::update_ldr_entry_base(libname, base, emu)
        } else {
            windows::peb::peb32::update_ldr_entry_base(libname, base, emu)
        }
    }
    fn dynamic_link_module64(&self, emu: &mut Emu, base: u64, pe_off: u32, libname: &str) {
        windows::peb::peb64::dynamic_link_module(base, pe_off, libname, emu)
    }
    fn fix_exe_module_name(&self, emu: &mut Emu) {
        windows::peb::peb64::fix_exe_module_name(emu)
    }
    fn ensure_ntdll_loader_globals(&self, emu: &mut Emu) {
        windows::peb::peb64::ensure_ntdll_loader_globals(emu)
    }
    fn ensure_peb_system_dependent_07(&self, emu: &mut Emu) {
        windows::peb::peb64::ensure_peb_system_dependent_07(emu)
    }
    fn active_exception_record(&self, emu: &Emu) -> Option<u64> {
        exception::handlers::active_record(emu)
    }
    fn enter_exception_record(
        &self,
        emu: &mut Emu,
        record: &ExceptionRecord,
        kind: HandlerKind,
        veh_index: usize,
    ) {
        exception::handlers::enter_record(emu, record, kind, veh_index)
    }
    fn exception_handle_return(&self, emu: &mut Emu, return_address: u64) -> bool {
        exception::seh32::handle_return(emu, return_address)
            || exception::handlers::handle_return(emu, return_address)
    }
    fn stolen_prologue_unwind(&self, emu: &mut Emu, addr: u64) -> Option<u64> {
        windows::stolen_prologue::unwind(emu, addr)
    }
    fn init(
        &self,
        emu: &mut Emu,
        clear_registers: bool,
        clear_flags: bool,
    ) -> Result<(), MwemuError> {
        WindowsInit::init_win32(emu, clear_registers, clear_flags)
    }
    fn load_pe32(
        &self,
        emu: &mut Emu,
        filename: &str,
        main_image: bool,
        force_base: u32,
    ) -> Result<PeLoadInfo, MwemuError> {
        PeLoading::load_pe32(emu, filename, main_image, force_base)
    }
    fn load_pe64(
        &self,
        emu: &mut Emu,
        filename: &str,
        main_image: bool,
        force_base: u64,
    ) -> Result<PeLoadInfo, MwemuError> {
        PeLoading::load_pe64(emu, filename, main_image, force_base)
    }
    fn map_dll_pe64(&self, emu: &mut Emu, filename: &str) -> Result<MappedPe64, MwemuError> {
        PeLoading::map_dll_pe64(emu, filename)
    }
    fn create_stub_pe32(&self, emu: &mut Emu, dll_name: &str, exports: &[&str]) -> u32 {
        PeStubs::create_stub_pe32(emu, dll_name, exports)
    }
    fn create_stub_pe64(&self, emu: &mut Emu, dll_name: &str, exports: &[&str]) -> u64 {
        PeStubs::create_stub_pe64(emu, dll_name, exports)
    }
}

/// Register the Windows layer with the emulator core. Calling it more than
/// once is harmless.
pub fn register() {
    layers::register_windows(&Layer);
}

/// `libmwemu::emu64()` for this crate's own unit tests.
#[cfg(test)]
fn emu64() -> Emu {
    let mut emu = Emu::new(mwemu_core::arch::Arch::X86_64);
    emu.disable_ctrlc();
    emu
}
