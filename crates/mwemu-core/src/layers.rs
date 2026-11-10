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
use crate::emu::loaders::{MappedPe64, PeLoadInfo};
use crate::err::MwemuError;
use crate::exception::types::{ExceptionRecord, HandlerKind};
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
    /// Load the ELF64 executable `filename` and its libraries
    /// (`Emu::load_elf64`).
    fn load_elf64(&self, emu: &mut Emu, filename: &str);
    /// Set `emu` up as an x86-64 Linux process (`Emu::init_linux64`).
    fn init64(&self, emu: &mut Emu, dyn_link: bool);
    /// Set `emu` up as an AArch64 Linux process
    /// (`Emu::init_linux64_aarch64`).
    fn init_aarch64(&self, emu: &mut Emu);
}

/// The macOS layer (`mwemu-macos`): syscalls and the libSystem hooks.
pub trait MacosLayer: Sync {
    /// x86-64 `syscall`.
    fn syscall_x86_64(&self, emu: &mut Emu);
    /// AArch64 `svc`.
    fn syscall_aarch64(&self, emu: &mut Emu);
    /// The guest called the libSystem hook at `addr`, in the map named `section`.
    fn api_call(&self, addr: u64, section: &str, symbol: &str, emu: &mut Emu);
    /// Load the 64-bit Mach-O executable `filename` and its dylibs
    /// (`Emu::load_macho64`).
    fn load_macho64(&self, emu: &mut Emu, filename: &str);
    /// Set `emu` up as an x86-64 macOS process (`Emu::init_macos64`).
    fn init64(&self, emu: &mut Emu);
    /// Set `emu` up as an AArch64 macOS process
    /// (`Emu::init_macos_aarch64`).
    fn init_aarch64(&self, emu: &mut Emu);
}

/// The Windows layer (`mwemu-windows`): the NT syscalls and the fix-ups they
/// need on a real ntdll, the PE loader, the PEB and loader data, the set-up
/// of an emulator as a Windows process and the exception dispatch.
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

    /// The name of the API at `addr`, or an empty string. `is64` selects the
    /// 64-bit loader; the other name and library look-ups below take it too.
    fn guess_api_name(&self, emu: &mut Emu, addr: u64, is64: bool) -> String;
    /// The name of the export at `addr`.
    fn resolve_api_addr_to_name(&self, emu: &mut Emu, addr: u64, is64: bool) -> String;
    /// Find an export by name in the loaded modules: its address, library and name.
    fn search_api_name(&self, emu: &mut Emu, name: &str, is64: bool) -> (u64, String, String);
    /// The address of the export `name` in any loaded module, or 0.
    fn resolve_api_name(&self, emu: &mut Emu, name: &str, is64: bool) -> u64;
    /// The address of the export `name` of `module`, or 0.
    fn resolve_api_name_in_module(
        &self,
        emu: &mut Emu,
        module: &str,
        name: &str,
        is64: bool,
    ) -> u64;
    /// Map the DLL `libname` and link it into the loader lists. Returns its base, or 0.
    fn load_library(&self, emu: &mut Emu, libname: &str, is64: bool) -> u64;
    /// Set the base address of the loader entry of `libname`.
    fn update_ldr_entry_base(&self, emu: &mut Emu, libname: &str, base: u64, is64: bool);
    /// Add a module to the 64-bit loader lists.
    fn dynamic_link_module64(&self, emu: &mut Emu, base: u64, pe_off: u32, libname: &str);
    /// Give the main image its real name in the 64-bit loader lists.
    fn fix_exe_module_name(&self, emu: &mut Emu);
    /// Initialise the loader globals of the mapped ntdll (`--ssdt`).
    fn ensure_ntdll_loader_globals(&self, emu: &mut Emu);
    /// Fill `PEB.SystemDependent07` before `LdrInitializeThunk` runs (`--ssdt`).
    fn ensure_peb_system_dependent_07(&self, emu: &mut Emu);
    /// The guest address of the exception record of the handler that is
    /// running, if one is.
    fn active_exception_record(&self, emu: &Emu) -> Option<u64>;
    /// Save the context and build the guest frame for calling a SEH, VEH or
    /// UEF handler with `record`. The caller sets the instruction pointer.
    fn enter_exception_record(
        &self,
        emu: &mut Emu,
        record: &ExceptionRecord,
        kind: HandlerKind,
        veh_index: usize,
    );
    /// A `ret` reached `return_address`: finish the exception handler if that
    /// is where it returns to. Returns whether it did.
    fn exception_handle_return(&self, emu: &mut Emu, return_address: u64) -> bool;
    /// If `addr` is inside an export whose first instructions the guest copied
    /// and skipped, the address to continue at.
    fn stolen_prologue_unwind(&self, emu: &mut Emu, addr: u64) -> Option<u64>;
    /// Set `emu` up as a Windows process (`Emu::init_win32`).
    fn init(
        &self,
        emu: &mut Emu,
        clear_registers: bool,
        clear_flags: bool,
    ) -> Result<(), MwemuError>;
    /// Load and bind a PE32 image (`Emu::load_pe32`).
    fn load_pe32(
        &self,
        emu: &mut Emu,
        filename: &str,
        main_image: bool,
        force_base: u32,
    ) -> Result<PeLoadInfo, MwemuError>;
    /// Load and bind a PE64 image (`Emu::load_pe64`).
    fn load_pe64(
        &self,
        emu: &mut Emu,
        filename: &str,
        main_image: bool,
        force_base: u64,
    ) -> Result<PeLoadInfo, MwemuError>;
    /// Map and relocate a PE64 DLL without binding its imports
    /// (`Emu::map_dll_pe64`).
    fn map_dll_pe64(&self, emu: &mut Emu, filename: &str) -> Result<MappedPe64, MwemuError>;
    /// Build and map a stub PE32 DLL (`Emu::create_stub_pe32`).
    fn create_stub_pe32(&self, emu: &mut Emu, dll_name: &str, exports: &[&str]) -> u32;
    /// Build and map a stub PE64 DLL (`Emu::create_stub_pe64`).
    fn create_stub_pe64(&self, emu: &mut Emu, dll_name: &str, exports: &[&str]) -> u64;
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

    /// `stop()`, for the entry points that report failure as an error.
    fn error(&self, emu: &mut Emu) -> MwemuError {
        self.stop(emu);
        MwemuError::new(&format!("the {} is not registered", self.0))
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
    fn load_elf64(&self, emu: &mut Emu, _filename: &str) {
        self.stop(emu);
    }
    fn init64(&self, emu: &mut Emu, _dyn_link: bool) {
        self.stop(emu);
    }
    fn init_aarch64(&self, emu: &mut Emu) {
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
    fn load_macho64(&self, emu: &mut Emu, _filename: &str) {
        self.stop(emu);
    }
    fn init64(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn init_aarch64(&self, emu: &mut Emu) {
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
    fn guess_api_name(&self, emu: &mut Emu, _addr: u64, _is64: bool) -> String {
        self.stop(emu);
        String::new()
    }
    fn resolve_api_addr_to_name(&self, emu: &mut Emu, _addr: u64, _is64: bool) -> String {
        self.stop(emu);
        String::new()
    }
    fn search_api_name(&self, emu: &mut Emu, _name: &str, _is64: bool) -> (u64, String, String) {
        self.stop(emu);
        (0, String::new(), String::new())
    }
    fn resolve_api_name(&self, emu: &mut Emu, _name: &str, _is64: bool) -> u64 {
        self.stop(emu);
        0
    }
    fn resolve_api_name_in_module(
        &self,
        emu: &mut Emu,
        _module: &str,
        _name: &str,
        _is64: bool,
    ) -> u64 {
        self.stop(emu);
        0
    }
    fn load_library(&self, emu: &mut Emu, _libname: &str, _is64: bool) -> u64 {
        self.stop(emu);
        0
    }
    fn update_ldr_entry_base(&self, emu: &mut Emu, _libname: &str, _base: u64, _is64: bool) {
        self.stop(emu);
    }
    fn dynamic_link_module64(&self, emu: &mut Emu, _base: u64, _pe_off: u32, _libname: &str) {
        self.stop(emu);
    }
    fn fix_exe_module_name(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn ensure_ntdll_loader_globals(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn ensure_peb_system_dependent_07(&self, emu: &mut Emu) {
        self.stop(emu);
    }
    fn active_exception_record(&self, _emu: &Emu) -> Option<u64> {
        None
    }
    fn enter_exception_record(
        &self,
        emu: &mut Emu,
        _record: &ExceptionRecord,
        _kind: HandlerKind,
        _veh_index: usize,
    ) {
        self.stop(emu);
    }
    fn exception_handle_return(&self, emu: &mut Emu, _return_address: u64) -> bool {
        self.stop(emu);
        false
    }
    fn stolen_prologue_unwind(&self, emu: &mut Emu, _addr: u64) -> Option<u64> {
        self.stop(emu);
        None
    }
    fn init(
        &self,
        emu: &mut Emu,
        _clear_registers: bool,
        _clear_flags: bool,
    ) -> Result<(), MwemuError> {
        Err(self.error(emu))
    }
    fn load_pe32(
        &self,
        emu: &mut Emu,
        _filename: &str,
        _main_image: bool,
        _force_base: u32,
    ) -> Result<PeLoadInfo, MwemuError> {
        Err(self.error(emu))
    }
    fn load_pe64(
        &self,
        emu: &mut Emu,
        _filename: &str,
        _main_image: bool,
        _force_base: u64,
    ) -> Result<PeLoadInfo, MwemuError> {
        Err(self.error(emu))
    }
    fn map_dll_pe64(&self, emu: &mut Emu, _filename: &str) -> Result<MappedPe64, MwemuError> {
        Err(self.error(emu))
    }
    fn create_stub_pe32(&self, emu: &mut Emu, _dll_name: &str, _exports: &[&str]) -> u32 {
        self.stop(emu);
        0
    }
    fn create_stub_pe64(&self, emu: &mut Emu, _dll_name: &str, _exports: &[&str]) -> u64 {
        self.stop(emu);
        0
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
