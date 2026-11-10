//! The x86 and x86-64 engine of mwemu: what each instruction does.
//!
//! [`register`] plugs it into the emulator core (`mwemu_core::layers`), whose
//! run loop decodes an instruction and hands it to
//! [`engine::emulate_instruction`]. `libmwemu` registers it for you.
//!
//! The modules keep the paths they had inside `mwemu-core` (`engine`,
//! `engine::instructions`, `engine::logic`), so the files moved here
//! unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
// NOTE: `neg_cmp_op_on_partial_ord` is kept deliberately — in the float
// compare handlers (cmpss/cmpsd/cmpps/cmppd/avx) `!(a < b)` is NOT the same
// as `a >= b` for NaN operands, which is the intended x86 semantics.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[macro_use]
extern crate mwemu_core;

use mwemu_core::emu::Emu;
use mwemu_core::layers::{self, X86Engine};

// The code names the emulator core as `crate::…`, as it did when it was part
// of mwemu-core. Keep those paths valid.
pub(crate) use mwemu_core::{arch, color, debug, emu, exception, fpu, regs64, utils, windows};

pub mod engine;

struct Engine;

impl X86Engine for Engine {
    fn emulate(
        &self,
        emu: &mut Emu,
        ins: &iced_x86::Instruction,
        instruction_sz: usize,
        rep_step: bool,
    ) -> bool {
        engine::emulate_instruction(emu, ins, instruction_sz, rep_step)
    }
}

/// Register the x86 engine with the emulator core. Calling it more than once
/// is harmless.
pub fn register() {
    layers::register_x86(&Engine);
}
