//! `rs-header` — generic executable-header parsers and loaders.
//!
//! Two independent front-ends share one philosophy (borrow the file bytes,
//! store only parsed metadata, bind into guest memory through a backend trait
//! so there is no emulator dependency):
//!
//! - [`pe`] — PE32/PE64 parser + loader (generic over [`pe::PeLoader`]).
//! - [`elf`] — ELF32/ELF64 parser + loader (generic over [`elf::ElfLoader`]).
//!
//! See `design/ARCHITECTURE.md` and `design/PE_EXTRACTION.md`.

pub mod pe;
pub mod elf;
