//! The Linux layer of mwemu: the Linux syscalls and the libc hooks.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), which
//! calls it when the guest makes a syscall or reaches a libc hook. `libmwemu`
//! registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`syscall::linux`,
//! `api::linux`), so the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, LinuxLayer};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{debug, emu, kernel, maps, windows};
pub mod api {
    pub(crate) use mwemu_core::api::{abi, printf, strconv, windows};

    pub mod linux;
}
pub mod syscall {
    pub mod linux;
}
pub(crate) use api::windows as winapi;

struct Layer;

impl LinuxLayer for Layer {
    fn syscall64(&self, emu: &mut Emu) {
        syscall::linux::syscall64::gateway(emu);
    }
    fn syscall32(&self, emu: &mut Emu) {
        syscall::linux::syscall32::gateway(emu);
    }
    fn syscall_aarch64(&self, emu: &mut Emu) {
        syscall::linux::syscall_aarch64::gateway(emu);
    }
    fn api_call(&self, addr: u64, section: &str, symbol: &str, emu: &mut Emu) {
        api::linux::gateway(addr, section, symbol, emu);
    }
}

/// Register the Linux layer with the emulator core. Calling it more than once
/// is harmless.
pub fn register() {
    layers::register_linux(&Layer);
}
