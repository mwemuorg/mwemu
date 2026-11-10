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

/// The Windows layer (`mwemu-windows`): the NT syscalls and the fix-ups they
/// need on a real ntdll.
pub trait WindowsLayer: Sync {
    /// x86-64 `syscall` and `int`.
    fn syscall64(&self, emu: &mut Emu);
    /// x86 `int`.
    fn syscall32(&self, emu: &mut Emu);
    /// x86 `sysenter`: NT API number `syscall`, arguments at `argv`.
    fn ntapi32(&self, syscall: u64, argv: u64, emu: &mut Emu);
    /// Map the syscall numbers of the loaded ntdll to the ones the syscall
    /// gateway matches on.
    fn build_syscall_translation_table(&self, emu: &mut Emu);
    /// Patch the load-reason checks of the loaded ntdll before
    /// `LdrInitializeThunk` runs.
    fn patch_ntdll_load_reason_checks(&self, emu: &mut Emu);
    /// Called before each instruction while the real ntdll runs (`--ssdt`),
    /// to redirect its heap-list walks.
    fn ntdll_heap_list_walk_fixup(&self, emu: &mut Emu, ins: &iced_x86::Instruction, rip: u64);
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
static WINDOWS: OnceLock<&'static dyn WindowsLayer> = OnceLock::new();
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

/// Register the Windows layer for this process. The first call wins; later
/// calls are ignored.
pub fn register_windows(layer: &'static dyn WindowsLayer) {
    let _ = WINDOWS.set(layer);
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

/// The registered Windows layer.
pub fn windows() -> &'static dyn WindowsLayer {
    WINDOWS.get().copied().unwrap_or(&Missing("Windows"))
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

impl WindowsLayer for Missing {
    fn syscall64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn syscall32(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn ntapi32(&self, _syscall: u64, _argv: u64, emu: &mut Emu) {
        self.stop(emu);
    }
    fn build_syscall_translation_table(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn patch_ntdll_load_reason_checks(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn ntdll_heap_list_walk_fixup(&self, emu: &mut Emu, _ins: &iced_x86::Instruction, _rip: u64) {
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
