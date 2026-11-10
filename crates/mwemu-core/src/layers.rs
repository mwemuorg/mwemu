//! Entry points into the layers built on top of the emulator core.
//!
//! The OS layers (their syscalls and API hooks) and the debugger need `Emu`,
//! so they are crates above this one and the core cannot name them. Whoever
//! links a layer in registers it here once, and the core reaches it through
//! the layer's trait.
//! `libmwemu` registers every layer in `emu32()`, `emu64()`, `emu_aarch64()`
//! and `register_api_handlers()`.
//!
//! An OS layer that was never registered logs what is missing and stops the
//! emulator: carrying on would run the guest with whatever the missing
//! syscall or API call left in the registers.

use std::sync::OnceLock;

use crate::emu::Emu;

/// The Linux layer (`mwemu-linux`): syscalls and the libc hooks.
pub trait LinuxLayer: Sync {
    /// x86-64 `syscall` and `int 0x80`.
    fn syscall64(&self, emu: &mut Emu);
    /// x86 `int 0x80`.
    fn syscall32(&self, emu: &mut Emu);
    /// AArch64 `svc`.
    fn syscall_aarch64(&self, emu: &mut Emu);
    /// The guest called the libc hook at `addr`, in the map named `section`.
    fn api_call(&self, addr: u64, section: &str, symbol: &str, emu: &mut Emu);
}

/// The macOS layer (`mwemu-macos`): syscalls and the libSystem hooks.
pub trait MacosLayer: Sync {
    /// x86-64 `syscall`.
    fn syscall_x86_64(&self, emu: &mut Emu);
    /// AArch64 `svc`.
    fn syscall_aarch64(&self, emu: &mut Emu);
    /// The guest called the libSystem hook at `addr`, in the map named `section`.
    fn api_call(&self, addr: u64, section: &str, symbol: &str, emu: &mut Emu);
}

/// The debugger layer (`mwemu-debug`): the interactive console and snapshots.
pub trait DebugLayer: Sync {
    /// Open the interactive console on the current state of `emu`.
    fn spawn_console(&self, emu: &mut Emu);
    /// Write a snapshot of `emu` to `filename`.
    fn dump(&self, emu: &Emu, filename: &str);
}

static LINUX: OnceLock<&'static dyn LinuxLayer> = OnceLock::new();
static MACOS: OnceLock<&'static dyn MacosLayer> = OnceLock::new();
static DEBUGGER: OnceLock<&'static dyn DebugLayer> = OnceLock::new();

/// Register the Linux layer for this process. The first call wins; later
/// calls are ignored.
pub fn register_linux(layer: &'static dyn LinuxLayer) {
    let _ = LINUX.set(layer);
}

/// Register the macOS layer for this process. The first call wins; later
/// calls are ignored.
pub fn register_macos(layer: &'static dyn MacosLayer) {
    let _ = MACOS.set(layer);
}

/// Register the debugger layer for this process. The first call wins; later
/// calls are ignored.
pub fn register_debugger(layer: &'static dyn DebugLayer) {
    let _ = DEBUGGER.set(layer);
}

/// The registered Linux layer.
pub fn linux() -> &'static dyn LinuxLayer {
    LINUX.get().copied().unwrap_or(&Missing("Linux"))
}

/// The registered macOS layer.
pub fn macos() -> &'static dyn MacosLayer {
    MACOS.get().copied().unwrap_or(&Missing("macOS"))
}

/// The registered debugger layer.
pub fn debugger() -> &'static dyn DebugLayer {
    DEBUGGER.get().copied().unwrap_or(&Missing("debugger"))
}

/// Stands in for a layer nobody registered.
struct Missing(&'static str);

impl Missing {
    fn stop(&self, emu: &mut Emu) {
        log::error!(
            "the {} layer is not registered: create the emulator with \
             libmwemu::emu32()/emu64()/emu_aarch64(), or call \
             libmwemu::register_api_handlers() first",
            self.0
        );
        emu.stop();
    }
}

impl LinuxLayer for Missing {
    fn syscall64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn syscall32(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn syscall_aarch64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn api_call(&self, _addr: u64, _section: &str, _symbol: &str, emu: &mut Emu) {
        self.stop(emu);
    }
}

impl MacosLayer for Missing {
    fn syscall_x86_64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn syscall_aarch64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn api_call(&self, _addr: u64, _section: &str, _symbol: &str, emu: &mut Emu) {
        self.stop(emu);
    }
}

impl DebugLayer for Missing {
    fn spawn_console(&self, emu: &mut Emu) {
        // The console is where the user decides what happens next, so do not
        // run on without it.
        self.stop(emu);
    }
    fn dump(&self, _emu: &Emu, filename: &str) {
        log::error!(
            "cannot write the snapshot {}: the debugger layer is not registered",
            filename
        );
    }
}
