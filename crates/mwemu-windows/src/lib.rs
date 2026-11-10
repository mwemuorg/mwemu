//! The Windows layer of mwemu: the NT syscalls, the helpers the API handlers
//! share (`api::windows::common`) and the emulated registry, environment,
//! profile and window state.
//!
//! [`register`] plugs the syscalls into the emulator core
//! (`mwemu_core::layers`). `libmwemu` registers it for you. The API handlers
//! themselves are `mwemu-winapi32` and `mwemu-winapi64`; the PE loader and
//! the PEB/TEB setup are still in the core, and the structures and constants
//! are in `mwemu-wintypes`.
//!
//! The modules keep the paths they had inside `mwemu-core`, so the files
//! moved here unchanged. Each module also re-exports what the core has under
//! the same path, so `windows::` and `api::windows::` name everything.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
// The crypto helpers round up by hand; mwemu-core allowed that crate-wide.
#![allow(clippy::manual_div_ceil)]

#[macro_use]
extern crate mwemu_core;

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, WindowsLayer};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{config, emu, exception, kernel, maps};
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
    pub mod profile_store;
    pub mod registry_store;
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
}

/// Register the Windows layer with the emulator core. Calling it more than
/// once is harmless.
pub fn register() {
    layers::register_windows(&Layer);
}
