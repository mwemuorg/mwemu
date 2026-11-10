//! Guest memory of mwemu: the memory maps ([`maps::Maps`], [`maps::mem64::Mem64`]),
//! their permissions and the heap allocator.
//!
//! This crate knows nothing about the emulator: `mwemu-core` owns a `Maps` in
//! `Emu` and re-exports this crate as `mwemu_core::maps`. `libmwemu`
//! re-exports it as `libmwemu::maps`.
//!
//! The module keeps the path it had inside `mwemu-core` (`maps`), so the
//! files moved here unchanged.

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod elf_loader;
pub mod hint;
pub mod maps;
pub mod mem_trace;
/// The Windows memory constants (`MEM_COMMIT`, `PAGE_READWRITE`, ...) that
/// `Maps` uses for its allocation and protection calls. `mwemu-core` has
/// them at `windows::constants::memory` with the other Windows constants.
pub mod windows {
    pub mod constants {
        pub mod memory;
    }
}

// The code names these as it did when it was part of mwemu-core. Keep those
// paths valid inside this crate.
pub(crate) mod utils {
    pub(crate) mod helpers {
        pub(crate) use crate::hint::{likely, unlikely};
    }
}
pub(crate) mod emu {
    pub(crate) mod emu_context {
        pub(crate) use crate::mem_trace::mem_trace_enabled;
    }
}
