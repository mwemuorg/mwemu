//! The macOS layer of mwemu: the Mach-O loader, the set-up of an emulator as
//! a macOS process, the macOS syscalls and the libSystem hooks.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), which
//! calls it to load a Mach-O and when the guest makes a syscall or reaches a
//! libSystem hook. `libmwemu` registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`syscall::macos`,
//! `api::macos`, `emu::loaders::macho`), so the files moved here unchanged.
//! The loader and the set-up were methods of `Emu`; here they are the
//! crate-private traits `MachoLoading` and `MacosInit`, implemented for
//! `Emu`, and `Emu::load_macho64()`, `init_macos64()` and
//! `init_macos_aarch64()` in the core call them through the layer. The
//! Mach-O parser (`emu::loaders::macho64`) is still in the core.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, MacosLayer};

use emu::initialization::MacosInit;
use emu::loaders::macho::MachoLoading;

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{arch, kernel, maps, threading};
pub mod emu {
    pub use mwemu_core::emu::*;

    pub mod initialization;
    pub mod loaders {
        pub use mwemu_core::emu::loaders::*;

        pub mod macho;
    }
}
pub mod api {
    pub(crate) use mwemu_core::api::{abi, printf, strconv};

    pub mod macos;
}
pub mod syscall {
    pub mod macos;
}

struct Layer;

impl MacosLayer for Layer {
    fn syscall_x86_64(&self, emu: &mut Emu) {
        syscall::macos::syscall_x86_64::gateway(emu);
    }
    fn syscall_aarch64(&self, emu: &mut Emu) {
        syscall::macos::syscall_aarch64::gateway(emu);
    }
    fn api_call(&self, addr: u64, section: &str, symbol: &str, emu: &mut Emu) {
        api::macos::gateway(addr, section, symbol, emu);
    }
    fn load_macho64(&self, emu: &mut Emu, filename: &str) {
        MachoLoading::load_macho64(emu, filename);
    }
    fn init64(&self, emu: &mut Emu) {
        MacosInit::init_macos64(emu);
    }
    fn init_aarch64(&self, emu: &mut Emu) {
        MacosInit::init_macos_aarch64(emu);
    }
}

/// Register the macOS layer with the emulator core. Calling it more than once
/// is harmless.
pub fn register() {
    layers::register_macos(&Layer);
}
