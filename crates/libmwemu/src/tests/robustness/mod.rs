//! Self-contained robustness tests: feed hostile / malformed input and assert
//! the emulator degrades gracefully (returns None/false/empty, logs) instead of
//! panicking. No sample bundle, no Windows DLLs, no network — these run in CI
//! and lock in the panic!/exit → graceful-degradation work so it can't regress.

mod memory;
mod pe_parse;
mod elf_parse;
mod loader;
mod instructions;
mod flags_diff;
mod shifts_diff;
mod rotates_diff;
mod misc_diff;
