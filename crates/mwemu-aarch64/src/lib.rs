//! The AArch64 engine of mwemu: what each instruction does.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), whose
//! run loop decodes an instruction and hands it to
//! [`engine::aarch64::emulate_instruction`]. `libmwemu` registers it for you.
//!
//! The module keeps the path it had inside `mwemu-core` (`engine::aarch64`),
//! so the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, Aarch64Engine};
use yaxpeax_arm::armv8::a64::Instruction;

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{emu, threading, windows};

pub mod engine {
    pub mod aarch64;
}

struct Engine;

impl Aarch64Engine for Engine {
    fn emulate(&self, emu: &mut Emu, ins: &Instruction) -> bool {
        engine::aarch64::emulate_instruction(emu, ins)
    }
}

/// Register the AArch64 engine with the emulator core. Calling it more than
/// once is harmless.
pub fn register() {
    layers::register_aarch64(&Engine);
}
