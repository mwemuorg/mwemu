//! Entry points into the layers built on top of the emulator core.
//!
//! The CPU engines (what each instruction does), the OS layers (their
//! syscalls and API hooks), the kernel API surfaces and the debugger need
//! `Emu`, so they are crates above this one and the core cannot name them. Whoever links a layer in
//! registers it here once, and the core reaches it through the layer's trait.
//! `libmwemu` registers every layer in `emu32()`, `emu64()`, `emu_aarch64()`
//! and `register_api_handlers()`.
//!
//! A layer that was never registered logs what is missing and stops the
//! emulator: carrying on would run the guest with whatever the missing
//! instruction, syscall or API call left in the registers.

use std::sync::OnceLock;

use crate::emu::Emu;
use crate::kernel::KernelOs;

/// The x86 and x86-64 engine (`mwemu-x86`).
pub trait X86Engine: Sync {
    /// Execute one decoded instruction of `instruction_sz` bytes. `rep_step`
    /// says the run loop handles `rep` prefixes one iteration at a time.
    /// Returns false if the instruction could not be emulated.
    fn emulate(
        &self,
        emu: &mut Emu,
        ins: &iced_x86::Instruction,
        instruction_sz: usize,
        rep_step: bool,
    ) -> bool;
}

/// The AArch64 engine (`mwemu-aarch64`).
pub trait Aarch64Engine: Sync {
    /// Execute one decoded instruction. Returns false if it could not be
    /// emulated.
    fn emulate(&self, emu: &mut Emu, ins: &yaxpeax_arm::armv8::a64::Instruction) -> bool;
}

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

/// The kernel API surfaces (`mwemu-kernel`): the kernel functions an emulated
/// driver can call.
pub trait KernelLayer: Sync {
    /// If the driver import `name` is a variable of the `os` kernel, the size
    /// of its storage; `None` if it is a function.
    fn data_symbol_size(&self, os: KernelOs, name: &str) -> Option<u64>;
    /// The driver called the `os` kernel function `symbol`. Returns false if
    /// it is not implemented.
    fn api_call(&self, os: KernelOs, symbol: &str, emu: &mut Emu) -> bool;
}

/// The debugger layer (`mwemu-debug`): the interactive console and snapshots.
pub trait DebugLayer: Sync {
    /// Open the interactive console on the current state of `emu`.
    fn spawn_console(&self, emu: &mut Emu);
    /// Write a snapshot of `emu` to `filename`.
    fn dump(&self, emu: &Emu, filename: &str);
}

static X86: OnceLock<&'static dyn X86Engine> = OnceLock::new();
static AARCH64: OnceLock<&'static dyn Aarch64Engine> = OnceLock::new();
static LINUX: OnceLock<&'static dyn LinuxLayer> = OnceLock::new();
static MACOS: OnceLock<&'static dyn MacosLayer> = OnceLock::new();
static WINDOWS: OnceLock<&'static dyn WindowsLayer> = OnceLock::new();
static KERNEL: OnceLock<&'static dyn KernelLayer> = OnceLock::new();
static DEBUGGER: OnceLock<&'static dyn DebugLayer> = OnceLock::new();

/// Register the x86 engine for this process. The first call wins; later
/// calls are ignored.
pub fn register_x86(engine: &'static dyn X86Engine) {
    let _ = X86.set(engine);
}

/// Register the AArch64 engine for this process. The first call wins; later
/// calls are ignored.
pub fn register_aarch64(engine: &'static dyn Aarch64Engine) {
    let _ = AARCH64.set(engine);
}

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

/// Register the kernel API surfaces for this process. The first call wins;
/// later calls are ignored.
pub fn register_kernel(layer: &'static dyn KernelLayer) {
    let _ = KERNEL.set(layer);
}

/// Register the debugger layer for this process. The first call wins; later
/// calls are ignored.
pub fn register_debugger(layer: &'static dyn DebugLayer) {
    let _ = DEBUGGER.set(layer);
}

/// The registered x86 engine. The run loop calls this for every
/// instruction, so it is inlined into it.
#[inline]
pub fn x86() -> &'static dyn X86Engine {
    X86.get().copied().unwrap_or(&Missing("x86 engine"))
}

/// The registered AArch64 engine. The run loop calls this for every
/// instruction, so it is inlined into it.
#[inline]
pub fn aarch64() -> &'static dyn Aarch64Engine {
    AARCH64.get().copied().unwrap_or(&Missing("AArch64 engine"))
}

/// The registered Linux layer.
pub fn linux() -> &'static dyn LinuxLayer {
    LINUX.get().copied().unwrap_or(&Missing("Linux layer"))
}

/// The registered macOS layer.
pub fn macos() -> &'static dyn MacosLayer {
    MACOS.get().copied().unwrap_or(&Missing("macOS layer"))
}

/// The registered Windows layer.
pub fn windows() -> &'static dyn WindowsLayer {
    WINDOWS.get().copied().unwrap_or(&Missing("Windows layer"))
}

/// The registered kernel API surfaces.
pub fn kernel() -> &'static dyn KernelLayer {
    KERNEL
        .get()
        .copied()
        .unwrap_or(&Missing("kernel API layer"))
}

/// The registered debugger layer.
pub fn debugger() -> &'static dyn DebugLayer {
    DEBUGGER
        .get()
        .copied()
        .unwrap_or(&Missing("debugger layer"))
}

/// Stands in for a layer nobody registered.
struct Missing(&'static str);

impl Missing {
    fn stop(&self, emu: &mut Emu) {
        log::error!(
            "the {} is not registered: create the emulator with \
             libmwemu::emu32()/emu64()/emu_aarch64(), or call \
             libmwemu::register_api_handlers() first",
            self.0
        );
        emu.stop();
    }
}

impl X86Engine for Missing {
    fn emulate(
        &self,
        emu: &mut Emu,
        _ins: &iced_x86::Instruction,
        _instruction_sz: usize,
        _rep_step: bool,
    ) -> bool {
        self.stop(emu);
        false
    }
}

impl Aarch64Engine for Missing {
    fn emulate(&self, emu: &mut Emu, _ins: &yaxpeax_arm::armv8::a64::Instruction) -> bool {
        self.stop(emu);
        false
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

impl KernelLayer for Missing {
    fn data_symbol_size(&self, _os: KernelOs, name: &str) -> Option<u64> {
        // There is no emulator to stop here: the first call the driver makes
        // into the kernel does that.
        log::error!(
            "cannot classify the kernel import {}: the kernel API layer is not registered",
            name
        );
        None
    }
    fn api_call(&self, _os: KernelOs, _symbol: &str, emu: &mut Emu) -> bool {
        self.stop(emu);
        // Not "unimplemented": the caller would log that and carry on.
        true
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

#[cfg(test)]
mod tests {
    /// Nothing registers an engine in this crate's test binary (the engines
    /// are crates above this one), so this is the unregistered case.
    #[test]
    fn an_emulator_without_an_engine_stops_at_its_first_instruction() {
        let mut emu = crate::emu64();
        // mov rax, 5 ; hlt
        let code = [0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, 0xf4];
        emu.load_code_bytes(&code).unwrap();
        let entry = emu.pc();

        let _ = emu.run(None);

        assert_eq!(emu.regs().rax, 0, "the instruction must not have run");
        assert_eq!(emu.pc(), entry, "the emulator must stop where it was");
    }
}
