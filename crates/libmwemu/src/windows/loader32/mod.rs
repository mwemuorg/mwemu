//! 32-bit Windows module loader: `LoadLibrary`-style DLL mapping and export
//! resolution (name <-> address, forwarders, API sets).
//!
//! This is loader state the emulator core needs (IAT binding, API dispatch,
//! the debugger), so it lives here and not with the kernel32 handlers.

mod library;
mod resolver;

pub use library::*;
pub use resolver::*;
