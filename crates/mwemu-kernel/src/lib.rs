//! The kernel API surfaces of mwemu: the Linux, Windows and macOS kernel
//! functions an emulated driver can call.
//!
//! The machinery that runs a driver (the linker, the stubs, the allocator
//! with its guard) is part of the emulator core, in `mwemu_core::kernel`.
//! [`register`] plugs these tables into it (`mwemu_core::layers`): the core
//! calls them when a driver reaches a kernel function. `libmwemu` registers
//! them for you.
//!
//! The three tables are one crate because the Windows and macOS ones reuse
//! the Linux `printk` formatter and string functions.
//!
//! The modules keep the paths they had inside `mwemu-core`
//! (`kernel::linux`, `kernel::windows`, `kernel::macos`), so the files moved
//! here unchanged. [`kernel`] also re-exports what the core has under the
//! same path, so it names everything.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::kernel::KernelOs;
use mwemu_core::layers::{self, KernelLayer};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{emu, maps};
pub mod kernel {
    pub use mwemu_core::kernel::*;

    pub mod linux;
    pub mod macos;
    pub mod windows;
}

struct Layer;

impl KernelLayer for Layer {
    fn data_symbol_size(&self, os: KernelOs, name: &str) -> Option<u64> {
        match os {
            KernelOs::Linux => kernel::linux::data_symbol_size(name),
            // The Windows and macOS surfaces import data through the image's
            // import table rather than by name, so nothing is classified here
            // yet; a function stub is the safe default.
            KernelOs::Windows | KernelOs::MacOS => None,
        }
    }
    fn api_call(&self, os: KernelOs, symbol: &str, emu: &mut Emu) -> bool {
        match os {
            KernelOs::Linux => kernel::linux::gateway(symbol, emu),
            KernelOs::Windows => kernel::windows::gateway(symbol, emu),
            KernelOs::MacOS => kernel::macos::gateway(symbol, emu),
        }
    }
}

/// Register the kernel API surfaces with the emulator core. Calling it more
/// than once is harmless.
pub fn register() {
    layers::register_kernel(&Layer);
}
