//! The Linux layer of mwemu: the ELF loader, the set-up of an emulator as a
//! Linux process, the Linux syscalls and the libc hooks.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), which
//! calls it to load an ELF and when the guest makes a syscall or reaches a
//! libc hook. `libmwemu` registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`syscall::linux`,
//! `api::linux`, `emu::loaders::elf`), so the files moved here unchanged.
//! The loader and the set-up were methods of `Emu`; here they are the
//! crate-private traits `ElfLoading` and `LinuxInit`, implemented for `Emu`,
//! and `Emu::load_elf64()`, `init_linux64()` and `init_linux64_aarch64()`
//! in the core call them through the layer.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, LinuxLayer};

use emu::initialization::LinuxInit;
use emu::loaders::elf::ElfLoading;

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{debug, kernel, maps, threading, windows};
pub mod emu {
    pub use mwemu_core::emu::*;

    pub mod initialization;
    pub mod loaders {
        pub use mwemu_core::emu::loaders::*;

        pub mod elf;
        pub mod elf_constants;
    }
}
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
    fn load_elf64(&self, emu: &mut Emu, filename: &str) {
        ElfLoading::load_elf64(emu, filename);
    }
    fn init64(&self, emu: &mut Emu, dyn_link: bool) {
        LinuxInit::init_linux64(emu, dyn_link);
    }
    fn init_aarch64(&self, emu: &mut Emu) {
        LinuxInit::init_linux64_aarch64(emu);
    }
}

/// Register the Linux layer with the emulator core. Calling it more than once
/// is harmless.
pub fn register() {
    layers::register_linux(&Layer);
}
