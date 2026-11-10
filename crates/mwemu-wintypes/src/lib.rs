//! Windows data types of mwemu: the structures a guest sees in memory (PEB,
//! TEB, LDR entries, contexts, ...), the Windows constants and the index of
//! module exports.
//!
//! This crate knows nothing about the emulator: the structures read and write
//! themselves through `mwemu_maps::maps::Maps`. `mwemu-core` re-exports the
//! three modules at `mwemu_core::windows`, next to the Windows code that
//! needs `Emu`.
//!
//! The modules keep the paths they had inside `mwemu-core`
//! (`windows::structures`, `windows::constants`, `windows::export_index`), so
//! the files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub mod windows {
    pub mod constants;
    pub mod export_index;
    pub mod structures;
}

// The code names guest memory as it did when it was part of mwemu-core. Keep
// that path valid inside this crate.
pub(crate) use mwemu_maps::maps;
