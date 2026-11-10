//! The macOS layer of mwemu: the macOS syscalls and the libSystem hooks.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), which
//! calls it when the guest makes a syscall or reaches a libSystem hook.
//! `libmwemu` registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`syscall::macos`,
//! `api::macos`), so the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, MacosLayer};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{emu, kernel, maps, threading};
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
}

/// Register the macOS layer with the emulator core. Calling it more than once
/// is harmless.
pub fn register() {
    layers::register_macos(&Layer);
}
