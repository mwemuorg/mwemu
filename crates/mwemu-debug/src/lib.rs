//! The debugger layer of mwemu: the interactive console, the script runner,
//! the gdb server and snapshots (native and minidump).
//!
//! [`register`] plugs the console and the snapshot writer into the emulator
//! core (`mwemu_core::layers`), which opens the console on a breakpoint and
//! writes a snapshot on exit. `libmwemu` registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`debug`,
//! `serialization`), so the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

#[macro_use]
extern crate mwemu_core;

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, DebugLayer};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{
    api, arch, config, eflags, emu, flags, fpu, hooks, kernel, maps, regs_aarch64, regs64,
    threading, utils, windows,
};
pub mod debug {
    pub(crate) use mwemu_core::debug::{breakpoint, console};

    pub mod commands;
    pub mod gdb;
    pub mod script;
    pub mod tracing;
}
pub mod serialization;

struct Layer;

impl DebugLayer for Layer {
    fn spawn_console(&self, emu: &mut Emu) {
        debug::commands::Commands::spawn_console(emu);
    }
    fn dump(&self, emu: &Emu, filename: &str) {
        serialization::Serialization::dump(emu, filename);
    }
}

/// Register the debugger layer with the emulator core. Calling it more than
/// once is harmless.
pub fn register() {
    layers::register_debugger(&Layer);
}

/// `libmwemu::emu32()` for this crate's own unit tests.
#[cfg(test)]
fn emu32() -> Emu {
    let mut emu = Emu::new(mwemu_core::arch::Arch::X86);
    emu.disable_ctrlc();
    emu
}
