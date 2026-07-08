//! Self-contained robustness tests: feed hostile / malformed input and assert
//! the emulator degrades gracefully (returns None/false/empty, logs) instead of
//! panicking. No sample bundle, no Windows DLLs, no network — these run in CI
//! and lock in the panic!/exit → graceful-degradation work so it can't regress.

mod elf_parse;
mod flags_diff;
mod instructions;
mod loader;
mod memory;
mod misc_diff;
mod pe_parse;
mod rotates_diff;
mod shifts_diff;
