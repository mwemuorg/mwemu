//! Whether memory accesses are traced on this thread.
//!
//! `Mem64` has no access to `Emu` (it only gets `&self`), yet its per-access
//! trace lines (`log_mem_read` / `log_mem_write` features) must still honour
//! `cfg.trace_mem`. Rather than keeping a raw `*const Emu` around — which
//! aliased the run loop's `&mut Emu` — the run loop publishes the one bool the
//! memory layer needs. Plain data, no pointer, no `unsafe`.

use std::cell::Cell;

thread_local! {
    static MEM_TRACE: Cell<bool> = const { Cell::new(false) };
}

/// Publish `cfg.trace_mem` for this thread's memory layer. Called by the run
/// loop entry points, so toggling `cfg.trace_mem` between runs takes effect.
pub fn set_mem_trace(enabled: bool) {
    MEM_TRACE.with(|flag| flag.set(enabled));
}

/// Whether `Mem64` should emit its per-access trace lines on this thread.
#[inline]
pub fn mem_trace_enabled() -> bool {
    MEM_TRACE.with(|flag| flag.get())
}
