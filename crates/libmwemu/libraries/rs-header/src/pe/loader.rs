//! The `PeLoader` trait: the only coupling between this crate and whatever
//! environment a PE is loaded into (an emulator, a sandbox, a real process
//! image builder, …).
//!
//! `mwemu-pe` knows *what* to patch (the IAT, base relocations) and *which*
//! imports to resolve; the consumer provides *how* to touch guest memory and
//! *how* to resolve an export. All loader entry points are generic over
//! `L: PeLoader`, so calls are monomorphized and inlined — no dynamic dispatch,
//! no runtime cost versus hand-written code.

/// Environment a PE is mapped and bound into. Implement this for your memory /
/// API-resolution backend and pass `&mut self` to the loader entry points.
pub trait PeLoader {
    // --- guest memory the loader patches ---

    /// Is `addr` backed by mapped guest memory?
    fn is_mapped(&self, addr: u64) -> bool;

    /// Write a section's bytes into guest memory at `addr` (the mapping step).
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool;

    /// Patch a 32-bit slot (IAT entry / 32-bit relocation).
    fn write_dword(&mut self, addr: u64, val: u32) -> bool;

    /// Patch a 64-bit slot (IAT entry / ADDR64 relocation).
    fn write_qword(&mut self, addr: u64, val: u64) -> bool;

    // --- import resolution ---

    /// Load a dependency by name, returning its image base (0 if it could not
    /// be loaded). The implementor owns DLL discovery/mapping.
    fn load_library(&mut self, libname: &str) -> u64;

    /// Resolve an exported function by name across loaded modules.
    fn resolve_api_name(&mut self, name: &str) -> u64;

    /// Resolve an exported function by name within a specific module.
    fn resolve_api_name_in_module(&mut self, module: &str, name: &str) -> u64;

    /// Best-effort search for a function by name; returns `(addr, dll, name)`.
    fn search_api_name(&mut self, name: &str) -> (u64, String, String);
}
