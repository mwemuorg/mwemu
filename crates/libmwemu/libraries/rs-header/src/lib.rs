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

// clippy v1 burn-down backlog (see V1-ROADMAP.md P2 #9)
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::doc_lazy_continuation)]

pub mod elf;
pub mod pe;
