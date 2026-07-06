//! C ABI bindings for `libmwemu`.
//!
//! This is the C counterpart of `pymwemu`: a thin `extern "C"` surface over the
//! `libmwemu::emu::Emu` object. The emulator lives behind an opaque
//! `MwemuEmu*` handle created by `mwemu_init32/64/aarch64` and destroyed by
//! `mwemu_free_emu`.
//!
//! Conventions
//! -----------
//! * Every function takes the handle as its first argument. A NULL handle is a
//!   no-op that records an error retrievable with `mwemu_last_error()`.
//! * Functions that can fail return `int32_t` (`1` = ok, `0` = error) and, when
//!   they produce a value, write it through an out-pointer. Plain getters that
//!   cannot fail return the value directly.
//! * Owned `char*` returned by the library (disassembly, strings) must be freed
//!   with `mwemu_free_string`. Owned byte buffers must be freed with
//!   `mwemu_free_buffer`, and `uint64_t` arrays with `mwemu_free_u64_buffer`.
//! * Input strings are NUL-terminated UTF-8 `const char*`. Input byte blobs are
//!   `(const uint8_t* ptr, size_t len)`.
//! * 128-bit (XMM / 128-bit memory) values are passed as a `lo`/`hi` pair of
//!   `uint64_t` to stay portable across C compilers.

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use libmwemu::emu::Emu as InnerEmu;
use libmwemu::maps::mem64::Permission;

/// Opaque handle to an emulator instance. Treat as `void*`; never dereference.
pub struct MwemuEmu {
    inner: InnerEmu,
}

// ---------------------------------------------------------------------------
// Permission bit constants (mirrors libmwemu::maps::mem64::Permission)
// ---------------------------------------------------------------------------

/// No access.
pub const MWEMU_PERM_NONE: u8 = 0b000;
/// Readable.
pub const MWEMU_PERM_READ: u8 = 0b001;
/// Writable.
pub const MWEMU_PERM_WRITE: u8 = 0b010;
/// Executable.
pub const MWEMU_PERM_EXECUTE: u8 = 0b100;
/// Read + write + execute (the default used when allocating).
pub const MWEMU_PERM_RWX: u8 = 0b111;

// ---------------------------------------------------------------------------
// Error handling: a thread-local "last error" string, C errno-style.
// ---------------------------------------------------------------------------

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_error<S: Into<Vec<u8>>>(msg: S) {
    let c = CString::new(msg).unwrap_or_else(|_| CString::new("error").unwrap());
    LAST_ERROR.with(|e| *e.borrow_mut() = Some(c));
}

fn clear_error() {
    LAST_ERROR.with(|e| *e.borrow_mut() = None);
}

/// Return the last error message recorded on this thread, or NULL if none.
///
/// The pointer is owned by the library and stays valid until the next call on
/// the same thread; copy it if you need to keep it.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_last_error() -> *const c_char {
    LAST_ERROR.with(|e| match &*e.borrow() {
        Some(c) => c.as_ptr(),
        None => std::ptr::null(),
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Deref the handle to `&mut InnerEmu`, returning `$ret` on a NULL pointer.
macro_rules! emu {
    ($ptr:ident, $ret:expr) => {{
        if $ptr.is_null() {
            set_error("null emu handle");
            return $ret;
        }
        clear_error();
        unsafe { &mut (*$ptr).inner }
    }};
}

/// Borrow a `const char*` as `&str`, returning `$ret` on NULL / invalid UTF-8.
macro_rules! cstr {
    ($ptr:ident, $ret:expr) => {{
        if $ptr.is_null() {
            set_error("null string argument");
            return $ret;
        }
        match unsafe { CStr::from_ptr($ptr) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_error("invalid utf-8 in string argument");
                return $ret;
            }
        }
    }};
}

fn ret_string(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c) => c.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Move a byte vector to the caller as `(ptr,len)`; free with `mwemu_free_buffer`.
fn ret_bytes(v: Vec<u8>, out_len: *mut usize) -> *mut u8 {
    let boxed = v.into_boxed_slice();
    let len = boxed.len();
    if !out_len.is_null() {
        unsafe { *out_len = len };
    }
    if len == 0 {
        return std::ptr::null_mut();
    }
    Box::into_raw(boxed) as *mut u8
}

/// Move a u64 vector to the caller as `(ptr,count)`; free with `mwemu_free_u64_buffer`.
fn ret_u64s(v: Vec<u64>, out_count: *mut usize) -> *mut u64 {
    let boxed = v.into_boxed_slice();
    let len = boxed.len();
    if !out_count.is_null() {
        unsafe { *out_count = len };
    }
    if len == 0 {
        return std::ptr::null_mut();
    }
    Box::into_raw(boxed) as *mut u64
}

unsafe fn slice_u8<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() || len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

fn boxed(inner: InnerEmu) -> *mut MwemuEmu {
    Box::into_raw(Box::new(MwemuEmu { inner }))
}

fn fresh(mut inner: InnerEmu) -> *mut MwemuEmu {
    inner.cfg.console_enabled = false;
    inner.cfg.verbose = 0;
    inner.cfg.shellcode = false;
    boxed(inner)
}

/// Create a 32-bit x86 emulator. Free with `mwemu_free_emu`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_init32() -> *mut MwemuEmu {
    clear_error();
    fresh(libmwemu::emu32())
}

/// Create a 64-bit x86-64 emulator. Free with `mwemu_free_emu`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_init64() -> *mut MwemuEmu {
    clear_error();
    fresh(libmwemu::emu64())
}

/// Create an AArch64 (ARM64) emulator. Free with `mwemu_free_emu`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_init_aarch64() -> *mut MwemuEmu {
    clear_error();
    fresh(libmwemu::emu_aarch64())
}

/// Destroy an emulator created by any `mwemu_init*` / load function.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_free_emu(emu: *mut MwemuEmu) {
    if !emu.is_null() {
        unsafe { drop(Box::from_raw(emu)) };
    }
}

/// Library version string. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_version() -> *mut c_char {
    ret_string(env!("CARGO_PKG_VERSION").to_string())
}

// ---------------------------------------------------------------------------
// Deserialization constructors (panic-contained: untrusted blobs).
// ---------------------------------------------------------------------------

fn catch_load<F: FnOnce() -> InnerEmu>(what: &str, f: F) -> *mut MwemuEmu {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(inner) => {
            clear_error();
            boxed(inner)
        }
        Err(_) => {
            set_error(format!("failed to {what}: input is corrupt, truncated, or malformed"));
            std::ptr::null_mut()
        }
    }
}

/// Rebuild an emulator from a serialized state blob (see `mwemu_serialize`).
/// Returns NULL on a corrupt/invalid blob (check `mwemu_last_error`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_deserialize(data: *const u8, len: usize) -> *mut MwemuEmu {
    let bytes = unsafe { slice_u8(data, len) }.to_vec();
    catch_load("deserialize emulator state", || {
        libmwemu::serialization::Serialization::deserialize(&bytes)
    })
}

/// Rebuild an emulator from a minidump file. Returns NULL on failure.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_from_minidump(filename: *const c_char) -> *mut MwemuEmu {
    let f = cstr!(filename, std::ptr::null_mut()).to_string();
    catch_load("load emulator state from minidump", || {
        libmwemu::serialization::Serialization::load_from_minidump(&f)
    })
}

/// Rebuild an emulator from a state file written by `mwemu_dump_to_file`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_from_file(filename: *const c_char) -> *mut MwemuEmu {
    let f = cstr!(filename, std::ptr::null_mut()).to_string();
    catch_load("load emulator state from file", || {
        libmwemu::serialization::Serialization::load_from_file(&f)
    })
}

/// Rebuild an emulator from a state file written by `mwemu_dump`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load(filename: *const c_char) -> *mut MwemuEmu {
    let f = cstr!(filename, std::ptr::null_mut()).to_string();
    catch_load("load emulator state", || {
        libmwemu::serialization::Serialization::load(&f)
    })
}

// ---------------------------------------------------------------------------
// Mode / introspection
// ---------------------------------------------------------------------------

/// Last emulated mnemonic with operands. Free with `mwemu_free_string`.
/// Returns NULL if no instruction has been decoded yet.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_prev_mnemonic(emu: *mut MwemuEmu) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    match e.last_decoded {
        Some(d) => ret_string(e.format_instruction(&d)),
        None => {
            set_error("no instruction decoded yet");
            std::ptr::null_mut()
        }
    }
}

/// Reset the instruction counter to zero.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_reset_pos(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.pos = 0;
}

/// 1 if the emulator is in 64-bit mode, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_is_64bits(emu: *mut MwemuEmu) -> i32 {
    let e = emu!(emu, 0);
    e.cfg.is_x64() as i32
}

/// 1 if the emulator is in 32-bit mode, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_is_32bits(emu: *mut MwemuEmu) -> i32 {
    let e = emu!(emu, 0);
    (!e.cfg.is_x64()) as i32
}

/// Switch to 64-bit x86-64 mode (needs the matching maps folder).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_64bits(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.arch = libmwemu::arch::Arch::X86_64;
}

/// Switch to 32-bit x86 mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_32bits(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.arch = libmwemu::arch::Arch::X86;
}

/// Switch to AArch64 mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_aarch64(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.arch = libmwemu::arch::Arch::Aarch64;
}

/// Change the LDR-entry base address of a loaded module.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_update_ldr_entry_base(emu: *mut MwemuEmu, libname: *const c_char, base: u64) {
    let e = emu!(emu, ());
    let name = cstr!(libname, ());
    e.update_ldr_entry_base(name, base);
}

/// Resolve an API address to its name. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_api_addr_to_name(emu: *mut MwemuEmu, addr: u64) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    ret_string(e.api_addr_to_name(addr))
}

/// Resolve an API name to its address (0 if not found).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_api_name_to_addr(emu: *mut MwemuEmu, name: *const c_char) -> u64 {
    let e = emu!(emu, 0);
    let n = cstr!(name, 0);
    e.api_name_to_addr(n)
}

// ---------------------------------------------------------------------------
// Output / tracing toggles
// ---------------------------------------------------------------------------

// Simple cfg-flag toggles, written out explicitly (not macro-generated) so
// cbindgen sees every `extern "C"` signature and emits it into mwemu.h.

/// Disable colored output.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_colors(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.nocolors = true;
}
/// Enable colored output.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_colors(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.nocolors = false;
}
/// Trace all memory reads/writes.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_trace_mem(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.trace_mem = true;
}
/// Disable the memory tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_trace_mem(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.trace_mem = false;
}
/// Trace all registers on every step.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_trace_regs(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.trace_regs = true;
}
/// Disable the register tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_trace_regs(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.trace_regs = false;
}
/// Enable the loop counter (slower).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_count_loops(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.loops = true;
}
/// Disable the loop counter.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_count_loops(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.loops = false;
}
/// Allow emulating zero-filled code blocks.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_allow_empty_code_blocks(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.allow_empty_code_blocks = true;
}
/// Enable shellcode mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_shellcode_mode(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.shellcode = true;
}
/// Disable the memory inspector.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_inspect_sequence(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.inspect = false;
}
/// Enable the stack tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_stack_trace(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.stack_trace = true;
}
/// Disable the stack tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_stack_trace(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.stack_trace = false;
}
/// Enable test mode (cross-checks emulation against inline asm).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_test_mode(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.test_mode = true;
}
/// Disable test mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_test_mode(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.test_mode = false;
}
/// Enable the console.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_console(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.console_enabled = true;
}
/// Disable the console.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_console(emu: *mut MwemuEmu) {
    emu!(emu, ()).cfg.console_enabled = false;
}

/// Trace a specific list of registers (array of lowercase names).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_trace_reg(emu: *mut MwemuEmu, names: *const *const c_char, count: usize) {
    let e = emu!(emu, ());
    let mut v = Vec::with_capacity(count);
    if !names.is_null() {
        for i in 0..count {
            let p = unsafe { *names.add(i) };
            if p.is_null() {
                continue;
            }
            if let Ok(s) = unsafe { CStr::from_ptr(p) }.to_str() {
                v.push(s.to_string());
            }
        }
    }
    e.cfg.trace_reg = true;
    e.cfg.reg_names = v;
}

/// Disable the multi-register tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_trace_reg(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.trace_reg = false;
    e.cfg.reg_names.clear();
}

/// Set verbosity (0..=3).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_verbose(emu: *mut MwemuEmu, verbose: u32) {
    let e = emu!(emu, ());
    e.cfg.verbose = verbose;
}

/// Trace a NUL/length-delimited string at `addr` on every step.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_trace_string(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.cfg.trace_string = true;
    e.cfg.string_addr = addr;
}

/// Disable the string tracer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_trace_string(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.trace_string = false;
    e.cfg.string_addr = 0;
}

/// Inspect a memory area described like `dword ptr [esp + 0x8]`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_inspect_sequence(emu: *mut MwemuEmu, seq: *const c_char) {
    let e = emu!(emu, ());
    let s = cstr!(seq, ());
    e.cfg.inspect = true;
    e.cfg.inspect_seq = s.to_string();
}

/// Set the program entry point.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_entry_point(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.cfg.entry_point = addr;
}

/// Rebase the code base address.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_base_address(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.cfg.code_base_addr = addr;
}

/// Set the stack base address.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_stack_base(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.cfg.stack_addr = addr;
}

/// Keep emulating past unimplemented instructions/APIs (banzai mode).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_banzai_mode(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.skip_unimplemented = true;
    e.maps.set_banzai(true);
}

/// Disable banzai mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_banzai_mode(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.skip_unimplemented = false;
}

/// Register an API name + parameter count for banzai mode.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_banzai_add(emu: *mut MwemuEmu, apiname: *const c_char, nparams: i32) {
    let e = emu!(emu, ());
    let n = cstr!(apiname, ());
    e.banzai_add(n, nparams);
}

/// Enable Ctrl-C handling (spawns the console).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_ctrlc(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.enable_ctrlc();
}

/// Disable Ctrl-C handling.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disable_ctrlc(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.disable_ctrlc();
}

/// Spawn the interactive console after `position` instructions.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_spawn_console_at_pos(emu: *mut MwemuEmu, position: u64) {
    let e = emu!(emu, ());
    e.cfg.console_enabled = true;
    e.spawn_console_at(position);
}

/// Spawn the interactive console when reaching `addr`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_spawn_console_at_addr(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.cfg.console2 = true;
    e.cfg.console_addr = addr;
    e.cfg.console_enabled = true;
}

/// Spawn the interactive console now.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_spawn_console(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.cfg.console_enabled = true;
    libmwemu::console::Console::spawn_console(e);
}

// ---------------------------------------------------------------------------
// Loading / maps
// ---------------------------------------------------------------------------

/// Set the maps folder (realistic memory layout from the mwemu maps repo).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_maps(emu: *mut MwemuEmu, folder: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(folder, ());
    e.cfg.maps_folder = f.to_string();
}

/// Fetch genuine Windows system DLLs from the symbol server and use them as the
/// maps folder. `version` is a friendly name ("win11") or a build ("26100.7920").
/// Returns 1 on success, 0 on failure (network/cache; see `mwemu_last_error`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_maps_from_winver(emu: *mut MwemuEmu, version: *const c_char) -> i32 {
    let e = emu!(emu, 0);
    let v = cstr!(version, 0);
    match e.set_maps_from_winver(v) {
        Ok(()) => 1,
        Err(err) => {
            set_error(format!("winver maps fetch failed: {}", err));
            0
        }
    }
}

/// Initialize the Windows environment (PEB/TEB/LDR/DLLs) for 32-bit.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_init_win32(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.init_win32(false, false);
}

/// Initialize the Linux environment (optionally with dynamic linking).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_init_linux64(emu: *mut MwemuEmu, dynamic: i32) {
    let e = emu!(emu, ());
    e.init_linux64(dynamic != 0);
}

/// Load a binary (PE/ELF/Mach-O or shellcode) from a file path.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_binary(emu: *mut MwemuEmu, filename: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(filename, ());
    e.load_code(f);
}

/// Load shellcode from a raw byte buffer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_code_bytes(emu: *mut MwemuEmu, data: *const u8, len: usize) {
    let e = emu!(emu, ());
    let bytes = unsafe { slice_u8(data, len) };
    e.load_code_bytes(bytes);
}

/// Allocate a named map of `size` bytes (RWX by default). Returns the base
/// address through `out_addr`; 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_alloc(
    emu: *mut MwemuEmu,
    name: *const c_char,
    size: u64,
    perm_bits: u8,
    out_addr: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    let n = cstr!(name, 0);
    let addr = e.alloc(n, size, Permission::from_bits(perm_bits));
    if !out_addr.is_null() {
        unsafe { *out_addr = addr };
    }
    1
}

/// Allocate a named map at a fixed address. Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_alloc_at(
    emu: *mut MwemuEmu,
    name: *const c_char,
    addr: u64,
    size: u64,
    perm_bits: u8,
) -> i32 {
    let e = emu!(emu, 0);
    let n = cstr!(name, 0);
    match e.maps.create_map(n, addr, size, Permission::from_bits(perm_bits)) {
        Ok(_) => 1,
        Err(err) => {
            set_error(format!("alloc_at failed: {}", err));
            0
        }
    }
}

/// Load an additional blob file at `base_addr` as a named map.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_load_map(
    emu: *mut MwemuEmu,
    name: *const c_char,
    filename: *const c_char,
    base_addr: u64,
) -> i32 {
    let e = emu!(emu, 0);
    let n = cstr!(name, 0);
    let f = cstr!(filename, 0);
    match e.maps.create_map(n, base_addr, 1, Permission::READ_WRITE_EXECUTE) {
        Ok(map) => {
            map.load(f);
            1
        }
        Err(err) => {
            set_error(format!("load_map failed: {}", err));
            0
        }
    }
}

/// Link a library by path, returning its base address (0 on failure path).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_link_library(emu: *mut MwemuEmu, filepath: *const c_char) -> u64 {
    let e = emu!(emu, 0);
    let f = cstr!(filepath, 0);
    e.link_library(f)
}

/// Free a named memory map.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_free(emu: *mut MwemuEmu, name: *const c_char) {
    let e = emu!(emu, ());
    let n = cstr!(name, ());
    e.maps.free(n);
}

// ---------------------------------------------------------------------------
// Registers
// ---------------------------------------------------------------------------

/// Read a register by name (e.g. "rax", "eip", "x0"). 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_reg(emu: *mut MwemuEmu, reg: *const c_char, out: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    let r = cstr!(reg, 0);
    let val = if e.cfg.arch.is_aarch64() {
        match e.regs_aarch64().get_by_name(r) {
            Some(v) => v,
            None => {
                set_error("invalid aarch64 register name");
                return 0;
            }
        }
    } else if e.regs().is_reg(r) {
        e.regs().get_by_name(r)
    } else {
        set_error("invalid register name");
        return 0;
    };
    if !out.is_null() {
        unsafe { *out = val };
    }
    1
}

/// Set a register by name; writes the previous value to `out_prev` if non-NULL.
/// Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_reg(
    emu: *mut MwemuEmu,
    reg: *const c_char,
    value: u64,
    out_prev: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    let r = cstr!(reg, 0);
    if e.cfg.arch.is_aarch64() {
        match e.regs_aarch64().get_by_name(r) {
            Some(prev) => {
                e.regs_aarch64_mut().set_by_name(r, value);
                if !out_prev.is_null() {
                    unsafe { *out_prev = prev };
                }
                1
            }
            None => {
                set_error("invalid aarch64 register name");
                0
            }
        }
    } else if e.regs().is_reg(r) {
        let prev = e.regs().get_by_name(r);
        e.regs_mut().set_by_name(r, value);
        if !out_prev.is_null() {
            unsafe { *out_prev = prev };
        }
        1
    } else {
        set_error("invalid register name");
        0
    }
}

/// Read an XMM register by name into a lo/hi `uint64_t` pair (x86 only).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_xmm(
    emu: *mut MwemuEmu,
    reg: *const c_char,
    out_lo: *mut u64,
    out_hi: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    if e.cfg.arch.is_aarch64() {
        set_error("xmm registers not available on aarch64");
        return 0;
    }
    let r = cstr!(reg, 0);
    if e.regs().is_xmm_by_name(r) {
        let v = e.regs().get_xmm_by_name(r);
        if !out_lo.is_null() {
            unsafe { *out_lo = v as u64 };
        }
        if !out_hi.is_null() {
            unsafe { *out_hi = (v >> 64) as u64 };
        }
        1
    } else {
        set_error("invalid register name");
        0
    }
}

/// Set an XMM register from a lo/hi `uint64_t` pair (x86 only).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_xmm(emu: *mut MwemuEmu, reg: *const c_char, lo: u64, hi: u64) -> i32 {
    let e = emu!(emu, 0);
    if e.cfg.arch.is_aarch64() {
        set_error("xmm registers not available on aarch64");
        return 0;
    }
    let r = cstr!(reg, 0);
    if e.regs().is_xmm_by_name(r) {
        let value = ((hi as u128) << 64) | (lo as u128);
        e.regs_mut().set_xmm_by_name(r, value);
        1
    } else {
        set_error("invalid register name");
        0
    }
}

/// Get the program counter (any arch).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_pc(emu: *mut MwemuEmu) -> u64 {
    let e = emu!(emu, 0);
    e.pc()
}

/// Set the program counter (any arch).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_pc(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.set_pc(addr);
}

/// Get the stack pointer (any arch).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_sp(emu: *mut MwemuEmu) -> u64 {
    let e = emu!(emu, 0);
    e.sp()
}

/// Set the stack pointer (any arch).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_sp(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.set_sp(addr);
}

/// Set RIP; if it points to an API it will be emulated. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_rip(emu: *mut MwemuEmu, addr: u64) -> i32 {
    let e = emu!(emu, 0);
    e.set_rip(addr, false) as i32
}

/// Set EIP; if it points to an API it will be emulated. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_eip(emu: *mut MwemuEmu, addr: u64) -> i32 {
    let e = emu!(emu, 0);
    e.set_eip(addr, false) as i32
}

/// Number of instructions emulated so far.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_position(emu: *mut MwemuEmu) -> u64 {
    let e = emu!(emu, 0);
    e.pos
}

// ---------------------------------------------------------------------------
// Stack
// ---------------------------------------------------------------------------

/// Push a 32-bit value. Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_stack_push32(emu: *mut MwemuEmu, value: u32) -> i32 {
    let e = emu!(emu, 0);
    if e.stack_push32(value) {
        1
    } else {
        set_error("push error");
        0
    }
}

/// Push a 64-bit value. Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_stack_push64(emu: *mut MwemuEmu, value: u64) -> i32 {
    let e = emu!(emu, 0);
    if e.stack_push64(value) {
        1
    } else {
        set_error("push error");
        0
    }
}

/// Pop a 32-bit value into `out`. Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_stack_pop32(emu: *mut MwemuEmu, out: *mut u32) -> i32 {
    let e = emu!(emu, 0);
    match e.stack_pop32(false) {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => {
            set_error("pop error");
            0
        }
    }
}

/// Pop a 64-bit value into `out`. Returns 1 on success, 0 on error.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_stack_pop64(emu: *mut MwemuEmu, out: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.stack_pop64(false) {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => {
            set_error("pop error");
            0
        }
    }
}

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

/// Stop emulation.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_stop(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.stop();
}

/// Emulate a single step. Returns 1 if it continued, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_step(emu: *mut MwemuEmu) -> i32 {
    let e = emu!(emu, 0);
    e.step() as i32
}

/// Run until `end_addr` (when `has_end` != 0) or forever. Writes the final PC
/// to `out_pc`. Returns 1 on success, 0 on error (see `mwemu_last_error`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_run(emu: *mut MwemuEmu, has_end: i32, end_addr: u64, out_pc: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    let end = if has_end != 0 { Some(end_addr) } else { None };
    match e.run(end) {
        Ok(pc) => {
            if !out_pc.is_null() {
                unsafe { *out_pc = pc };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Run until reaching instruction-count `position`. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_run_to(emu: *mut MwemuEmu, position: u64, out_pc: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.run_to(position) {
        Ok(pc) => {
            if !out_pc.is_null() {
                unsafe { *out_pc = pc };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Run until the first return. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_run_until_return(emu: *mut MwemuEmu, out_pc: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.run_until_ret() {
        Ok(pc) => {
            if !out_pc.is_null() {
                unsafe { *out_pc = pc };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Run until the next winapi call. Writes the API address to `out_addr` and its
/// name to `out_name` (free with `mwemu_free_string`). Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_run_until_apicall(
    emu: *mut MwemuEmu,
    out_addr: *mut u64,
    out_name: *mut *mut c_char,
) -> i32 {
    let e = emu!(emu, 0);
    e.skip_apicall = true;
    e.is_break_on_api = true;
    let _ = e.run(None);
    match e.its_apicall {
        Some(addr) => {
            e.skip_apicall = false;
            let name = e.api_addr_to_name(addr);
            let new_pc = e.pc() + e.last_instruction_size as u64;
            e.set_pc(new_pc);
            if !out_addr.is_null() {
                unsafe { *out_addr = addr };
            }
            if !out_name.is_null() {
                unsafe { *out_name = ret_string(name) };
            }
            1
        }
        None => {
            set_error("no apicall reached");
            0
        }
    }
}

/// Handle a winapi call at `addr`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_handle_winapi(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.handle_winapi(addr);
}

/// Call a 32-bit function, pushing `params` (reverse order internally).
/// Writes the return value to `out_ret`. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_call32(
    emu: *mut MwemuEmu,
    address: u64,
    params: *const u32,
    nparams: usize,
    out_ret: *mut u32,
) -> i32 {
    let e = emu!(emu, 0);
    let args: Vec<u32> = if params.is_null() || nparams == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(params, nparams) }.to_vec()
    };
    match e.call32(address, &args) {
        Ok(v) => {
            if !out_ret.is_null() {
                unsafe { *out_ret = v };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Call a 64-bit (Windows ABI) function. Writes the return value to `out_ret`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_call64(
    emu: *mut MwemuEmu,
    address: u64,
    params: *const u64,
    nparams: usize,
    out_ret: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    let args: Vec<u64> = if params.is_null() || nparams == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(params, nparams) }.to_vec()
    };
    match e.call64(address, &args) {
        Ok(v) => {
            if !out_ret.is_null() {
                unsafe { *out_ret = v };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Call a 64-bit function using the Linux SysV ABI.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_linux_call64(
    emu: *mut MwemuEmu,
    address: u64,
    params: *const u64,
    nparams: usize,
    out_ret: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    let args: Vec<u64> = if params.is_null() || nparams == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(params, nparams) }.to_vec()
    };
    match e.linux_call64(address, &args) {
        Ok(v) => {
            if !out_ret.is_null() {
                unsafe { *out_ret = v };
            }
            1
        }
        Err(err) => {
            set_error(err.message);
            0
        }
    }
}

/// Disassemble `amount` instructions at `addr`. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_disassemble(emu: *mut MwemuEmu, addr: u64, amount: u32) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    ret_string(e.disassemble(addr, amount))
}

// ---------------------------------------------------------------------------
// Memory: scalar read/write
// ---------------------------------------------------------------------------

/// Write a little-endian qword. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_qword(emu: *mut MwemuEmu, addr: u64, value: u64) -> i32 {
    let e = emu!(emu, 0);
    if e.maps.write_qword(addr, value) { 1 } else { set_error("write on non allocated address"); 0 }
}
/// Write a little-endian dword. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_dword(emu: *mut MwemuEmu, addr: u64, value: u32) -> i32 {
    let e = emu!(emu, 0);
    if e.maps.write_dword(addr, value) { 1 } else { set_error("write on non allocated address"); 0 }
}
/// Write a little-endian word. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_word(emu: *mut MwemuEmu, addr: u64, value: u16) -> i32 {
    let e = emu!(emu, 0);
    if e.maps.write_word(addr, value) { 1 } else { set_error("write on non allocated address"); 0 }
}
/// Write a byte. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_byte(emu: *mut MwemuEmu, addr: u64, value: u8) -> i32 {
    let e = emu!(emu, 0);
    if e.maps.write_byte(addr, value) { 1 } else { set_error("write on non allocated address"); 0 }
}

/// Read a little-endian qword into `out`. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_qword(emu: *mut MwemuEmu, addr: u64, out: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_qword(addr) {
        Some(v) => { if !out.is_null() { unsafe { *out = v }; } 1 }
        None => { set_error("read on non allocated address"); 0 }
    }
}
/// Read a little-endian dword into `out`. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_dword(emu: *mut MwemuEmu, addr: u64, out: *mut u32) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_dword(addr) {
        Some(v) => { if !out.is_null() { unsafe { *out = v }; } 1 }
        None => { set_error("read on non allocated address"); 0 }
    }
}
/// Read a little-endian word into `out`. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_word(emu: *mut MwemuEmu, addr: u64, out: *mut u16) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_word(addr) {
        Some(v) => { if !out.is_null() { unsafe { *out = v }; } 1 }
        None => { set_error("read on non allocated address"); 0 }
    }
}
/// Read a byte into `out`. Returns 1 on success, 0 if unmapped.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_byte(emu: *mut MwemuEmu, addr: u64, out: *mut u8) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_byte(addr) {
        Some(v) => { if !out.is_null() { unsafe { *out = v }; } 1 }
        None => { set_error("read on non allocated address"); 0 }
    }
}

/// Read a 128-bit little-endian value into a lo/hi pair. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_128bits_le(
    emu: *mut MwemuEmu,
    addr: u64,
    out_lo: *mut u64,
    out_hi: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_128bits_le(addr) {
        Some(v) => {
            if !out_lo.is_null() {
                unsafe { *out_lo = v as u64 };
            }
            if !out_hi.is_null() {
                unsafe { *out_hi = (v >> 64) as u64 };
            }
            1
        }
        None => {
            set_error("read on non allocated address");
            0
        }
    }
}

/// Read a 128-bit big-endian value into a lo/hi pair. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_128bits_be(
    emu: *mut MwemuEmu,
    addr: u64,
    out_lo: *mut u64,
    out_hi: *mut u64,
) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.read_128bits_be(addr) {
        Some(v) => {
            if !out_lo.is_null() {
                unsafe { *out_lo = v as u64 };
            }
            if !out_hi.is_null() {
                unsafe { *out_hi = (v >> 64) as u64 };
            }
            1
        }
        None => {
            set_error("read on non allocated address");
            0
        }
    }
}

/// Fill `amount` bytes at `addr` with `byte`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_memset(emu: *mut MwemuEmu, addr: u64, byte: u8, amount: usize) {
    let e = emu!(emu, ());
    e.maps.memset(addr, byte, amount);
}

// ---------------------------------------------------------------------------
// Memory: buffers & strings
// ---------------------------------------------------------------------------

/// Write a byte buffer to memory.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_buffer(emu: *mut MwemuEmu, addr: u64, data: *const u8, len: usize) {
    let e = emu!(emu, ());
    let bytes = unsafe { slice_u8(data, len) };
    e.maps.write_buffer(addr, bytes);
}

/// Read `len` bytes from memory. Returns an owned buffer (free with
/// `mwemu_free_buffer`) and writes its length to `out_len`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_bytes(
    emu: *mut MwemuEmu,
    addr: u64,
    len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    let e = emu!(emu, std::ptr::null_mut());
    let v = e.maps.read_bytes(addr, len).to_vec();
    ret_bytes(v, out_len)
}

/// Read `len` bytes into a caller-provided buffer. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_mem(emu: *mut MwemuEmu, addr: u64, out: *mut u8, len: usize) -> i32 {
    let e = emu!(emu, 0);
    if out.is_null() {
        set_error("null output buffer");
        return 0;
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(out, len) };
    for (i, b) in dst.iter_mut().enumerate() {
        match e.maps.read_byte(addr + i as u64) {
            Some(v) => *b = v,
            None => {
                set_error("read on non allocated address");
                return 0;
            }
        }
    }
    1
}

/// Write a NUL-terminated ASCII string to memory.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_string(emu: *mut MwemuEmu, addr: u64, s: *const c_char) {
    let e = emu!(emu, ());
    let st = cstr!(s, ());
    e.maps.write_string(addr, st);
}

/// Write a UTF-16 (wide) string to memory.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_wide_string(emu: *mut MwemuEmu, addr: u64, s: *const c_char) {
    let e = emu!(emu, ());
    let st = cstr!(s, ());
    e.maps.write_wide_string(addr, st);
}

/// Read an ASCII string from memory. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_string(emu: *mut MwemuEmu, addr: u64) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    ret_string(e.maps.read_string(addr))
}

/// Read a wide (UTF-16) string from memory. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_wide_string(emu: *mut MwemuEmu, addr: u64) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    ret_string(e.maps.read_wide_string(addr))
}

/// Spaced-hex string of `sz` bytes at `addr`. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_read_string_of_bytes(emu: *mut MwemuEmu, addr: u64, sz: usize) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    ret_string(e.maps.read_string_of_bytes(addr, sz))
}

/// Size in bytes of a wide string at the given pointer.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_sizeof_wide(emu: *mut MwemuEmu, ptr: u64) -> usize {
    let e = emu!(emu, 0);
    e.maps.sizeof_wide(ptr)
}

/// Write spaced-hex bytes ("90 90 c3") to memory. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_write_spaced_bytes(emu: *mut MwemuEmu, addr: u64, spaced: *const c_char) -> i32 {
    let e = emu!(emu, 0);
    let s = cstr!(spaced, 0);
    if e.maps.write_spaced_bytes(addr, s) {
        1
    } else {
        set_error("couldn't write bytes at that address");
        0
    }
}

// ---------------------------------------------------------------------------
// Memory: maps, search & diagnostics
// ---------------------------------------------------------------------------

/// Print all memory maps whose name matches `kw`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_print_maps_by_keyword(emu: *mut MwemuEmu, kw: *const c_char) {
    let e = emu!(emu, ());
    let k = cstr!(kw, ());
    e.maps.print_maps_keyword(k);
}

/// Print all memory maps.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_print_maps(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.maps.print_maps();
}

/// Base address of the map containing `addr`. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_addr_base(emu: *mut MwemuEmu, addr: u64, out: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.get_addr_base(addr) {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => {
            set_error("address is not allocated");
            0
        }
    }
}

/// 1 if `addr` is mapped, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_is_mapped(emu: *mut MwemuEmu, addr: u64) -> i32 {
    let e = emu!(emu, 0);
    e.maps.is_mapped(addr) as i32
}

/// Name of the map containing `addr`. Free with `mwemu_free_string`; NULL if
/// the address is not allocated.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_addr_name(emu: *mut MwemuEmu, addr: u64) -> *mut c_char {
    let e = emu!(emu, std::ptr::null_mut());
    match e.maps.get_addr_name(addr) {
        Some(v) => ret_string(v.to_string()),
        None => {
            set_error("address not in an allocated block");
            std::ptr::null_mut()
        }
    }
}

/// Hex-dump the bytes at `addr`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_dump_memory(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.maps.dump(addr);
}

/// Hex-dump `amount` bytes at `addr`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_dump_n(emu: *mut MwemuEmu, addr: u64, amount: u64) {
    let e = emu!(emu, ());
    e.maps.dump_n(addr, amount);
}

/// Total allocated bytes.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_allocated_size(emu: *mut MwemuEmu) -> usize {
    let e = emu!(emu, 0);
    e.maps.size()
}

/// 1 if a block of `sz` at `addr` overlaps another map.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_memory_overlaps(emu: *mut MwemuEmu, addr: u64, sz: u64) -> i32 {
    let e = emu!(emu, 0);
    e.maps.overlaps(addr, sz) as i32
}

/// Print all allocations made during emulation.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_show_allocs(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.maps.show_allocs();
}

/// Find a free block of `sz` bytes (does not allocate). Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_memory_alloc(emu: *mut MwemuEmu, sz: u64, out_addr: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.maps.alloc(sz) {
        Some(a) => {
            if !out_addr.is_null() {
                unsafe { *out_addr = a };
            }
            1
        }
        None => {
            set_error("no free space of that size");
            0
        }
    }
}

/// Save every allocation to files under `path`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_save_all_allocs(emu: *mut MwemuEmu, path: *const c_char) {
    let e = emu!(emu, ());
    let p = cstr!(path, ());
    e.maps.save_all_allocs(p.to_string());
}

/// Save `size` bytes at `addr` to `filename`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_save(emu: *mut MwemuEmu, addr: u64, size: u64, filename: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(filename, ());
    e.maps.save(addr, size, f.to_string());
}

/// Run a memory consistency test. Returns 1/0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_mem_test(emu: *mut MwemuEmu) -> i32 {
    let e = emu!(emu, 0);
    e.maps.mem_test() as i32
}

/// Search an ASCII substring in a named map. Returns an owned `uint64_t` array
/// (free with `mwemu_free_u64_buffer`); `out_count` gets the match count.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_string(
    emu: *mut MwemuEmu,
    kw: *const c_char,
    map_name: *const c_char,
    out_count: *mut usize,
) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    let k = cstr!(kw, std::ptr::null_mut());
    let m = cstr!(map_name, std::ptr::null_mut());
    let v = e.maps.search_string(k, m).unwrap_or_default();
    ret_u64s(v, out_count)
}

/// Search spaced-hex bytes in a named map. Returns an owned `uint64_t` array.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_spaced_bytes(
    emu: *mut MwemuEmu,
    sbs: *const c_char,
    map_name: *const c_char,
    out_count: *mut usize,
) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    let s = cstr!(sbs, std::ptr::null_mut());
    let m = cstr!(map_name, std::ptr::null_mut());
    ret_u64s(e.maps.search_spaced_bytes(s, m), out_count)
}

/// Search spaced-hex bytes across all maps. Returns an owned `uint64_t` array.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_spaced_bytes_in_all(
    emu: *mut MwemuEmu,
    sbs: *const c_char,
    out_count: *mut usize,
) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    let s = cstr!(sbs, std::ptr::null_mut());
    ret_u64s(e.maps.search_spaced_bytes_in_all(s), out_count)
}

/// Search spaced-hex bytes forward from `saddr`. Returns the address or 0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_spaced_bytes_from(emu: *mut MwemuEmu, saddr: u64, sbs: *const c_char) -> u64 {
    let e = emu!(emu, 0);
    let s = cstr!(sbs, 0);
    e.maps.search_spaced_bytes_from(s, saddr)
}

/// Search spaced-hex bytes backward from `saddr`. Returns the address or 0.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_spaced_bytes_from_bw(emu: *mut MwemuEmu, saddr: u64, sbs: *const c_char) -> u64 {
    let e = emu!(emu, 0);
    let s = cstr!(sbs, 0);
    e.maps.search_spaced_bytes_from_bw(s, saddr)
}

/// Search a byte buffer in a named map. Returns an owned `uint64_t` array.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_search_bytes(
    emu: *mut MwemuEmu,
    data: *const u8,
    len: usize,
    map_name: *const c_char,
    out_count: *mut usize,
) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    let m = cstr!(map_name, std::ptr::null_mut());
    let bytes = unsafe { slice_u8(data, len) }.to_vec();
    ret_u64s(e.maps.search_bytes(bytes, m), out_count)
}

// ---------------------------------------------------------------------------
// Breakpoints
// ---------------------------------------------------------------------------

/// Print all breakpoints.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_show(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.bp.show();
}

/// Clear all breakpoints.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_clear_all(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.bp.clear_bp();
}

/// Set an address breakpoint.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_set_addr(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.bp.add_bp(addr);
}

/// Set an instruction-count breakpoint.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_set_inst(emu: *mut MwemuEmu, ins: u64) {
    let e = emu!(emu, ());
    e.bp.add_bp_instruction(ins);
}

/// Set a memory-read breakpoint.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_set_mem_read(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.bp.add_bp_mem_read(addr);
}

/// Set a memory-write breakpoint.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_set_mem_write(emu: *mut MwemuEmu, addr: u64) {
    let e = emu!(emu, ());
    e.bp.add_bp_mem_write(addr);
}

/// Get all address breakpoints. Owned `uint64_t` array; free with
/// `mwemu_free_u64_buffer`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_get_addr(emu: *mut MwemuEmu, out_count: *mut usize) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    ret_u64s(e.bp.addr.clone(), out_count)
}

/// Get all instruction-count breakpoints.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_get_inst(emu: *mut MwemuEmu, out_count: *mut usize) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    ret_u64s(e.bp.instruction.clone(), out_count)
}

/// Get all memory-read breakpoints.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_get_mem_read(emu: *mut MwemuEmu, out_count: *mut usize) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    ret_u64s(e.bp.mem_read_addr.clone(), out_count)
}

/// Get all memory-write breakpoints.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_bp_get_mem_write(emu: *mut MwemuEmu, out_count: *mut usize) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    ret_u64s(e.bp.mem_write_addr.clone(), out_count)
}

// ---------------------------------------------------------------------------
// Call stack & threading
// ---------------------------------------------------------------------------

/// Get the call stack as a flat `[from0, to0, from1, to1, ...]` `uint64_t`
/// array. `out_count` gets the number of frames (pairs). Free the array with
/// `mwemu_free_u64_buffer` passing `out_count * 2` as the element count.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_call_stack(emu: *mut MwemuEmu, out_count: *mut usize) -> *mut u64 {
    let e = emu!(emu, std::ptr::null_mut());
    let cs = e.call_stack();
    let mut flat = Vec::with_capacity(cs.len() * 2);
    for (from, to) in cs.iter() {
        flat.push(*from);
        flat.push(*to);
    }
    if !out_count.is_null() {
        unsafe { *out_count = cs.len() };
    }
    let boxed = flat.into_boxed_slice();
    if boxed.is_empty() {
        return std::ptr::null_mut();
    }
    Box::into_raw(boxed) as *mut u64
}

/// Enable or disable threading emulation.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_enable_threading(emu: *mut MwemuEmu, enable: i32) {
    let e = emu!(emu, ());
    e.enable_threading(enable != 0);
}

// ---------------------------------------------------------------------------
// Serialization
// ---------------------------------------------------------------------------

/// Serialize the whole emulator state. Owned buffer (free with
/// `mwemu_free_buffer`); `out_len` gets the length.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_serialize(emu: *mut MwemuEmu, out_len: *mut usize) -> *mut u8 {
    let e = emu!(emu, std::ptr::null_mut());
    let v = libmwemu::serialization::Serialization::serialize(e);
    ret_bytes(v, out_len)
}

/// Serialize the state to a file (load with `mwemu_load_from_file`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_dump_to_file(emu: *mut MwemuEmu, filename: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(filename, ());
    libmwemu::serialization::Serialization::dump_to_file(e, f);
}

/// Serialize the state to a minidump file (load with `mwemu_load_from_minidump`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_dump_to_minidump(emu: *mut MwemuEmu, filename: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(filename, ());
    let _ = libmwemu::serialization::Serialization::dump_to_minidump(e, f);
}

/// Serialize the state to a file (load with `mwemu_load`).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_dump(emu: *mut MwemuEmu, filename: *const c_char) {
    let e = emu!(emu, ());
    let f = cstr!(filename, ());
    libmwemu::serialization::Serialization::dump(e, f);
}

// ---------------------------------------------------------------------------
// Config getters / setters
// ---------------------------------------------------------------------------

/// Set max instructions; `has_value` 0 clears the limit.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_max_instructions(emu: *mut MwemuEmu, has_value: i32, value: u64) {
    let e = emu!(emu, ());
    e.cfg.max_instructions = if has_value != 0 { Some(value) } else { None };
}

/// Get max instructions into `out`. Returns 1 if set, 0 if unlimited.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_max_instructions(emu: *mut MwemuEmu, out: *mut u64) -> i32 {
    let e = emu!(emu, 0);
    match e.cfg.max_instructions {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => 0,
    }
}

/// Set the timeout in seconds; `has_value` 0 clears it.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_timeout_secs(emu: *mut MwemuEmu, has_value: i32, secs: f64) {
    let e = emu!(emu, ());
    e.cfg.timeout_secs = if has_value != 0 { Some(secs) } else { None };
}

/// Get the timeout into `out`. Returns 1 if set, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_timeout_secs(emu: *mut MwemuEmu, out: *mut f64) -> i32 {
    let e = emu!(emu, 0);
    match e.cfg.timeout_secs {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => 0,
    }
}

/// Set max faults; `has_value` 0 clears it.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_max_faults(emu: *mut MwemuEmu, has_value: i32, faults: u32) {
    let e = emu!(emu, ());
    e.cfg.max_faults = if has_value != 0 { Some(faults) } else { None };
}

/// Get max faults into `out`. Returns 1 if set, 0 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_max_faults(emu: *mut MwemuEmu, out: *mut u32) -> i32 {
    let e = emu!(emu, 0);
    match e.cfg.max_faults {
        Some(v) => {
            if !out.is_null() {
                unsafe { *out = v };
            }
            1
        }
        None => 0,
    }
}

/// Get the exit position (instruction count at which to stop).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_exit_position(emu: *mut MwemuEmu) -> u64 {
    emu!(emu, 0).cfg.exit_position
}
/// Set the exit position.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_exit_position(emu: *mut MwemuEmu, value: u64) {
    emu!(emu, ()).cfg.exit_position = value;
}
/// Get the trace start position.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_trace_start(emu: *mut MwemuEmu) -> u64 {
    emu!(emu, 0).cfg.trace_start
}
/// Set the trace start position.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_trace_start(emu: *mut MwemuEmu, value: u64) {
    emu!(emu, ()).cfg.trace_start = value;
}
/// Get the minimum heap allocation size.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_heap_alloc_min_size(emu: *mut MwemuEmu) -> u64 {
    emu!(emu, 0).cfg.heap_alloc_min_size
}
/// Set the minimum heap allocation size.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_heap_alloc_min_size(emu: *mut MwemuEmu, value: u64) {
    emu!(emu, ()).cfg.heap_alloc_min_size = value;
}

/// Get dump-on-exit (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_dump_on_exit(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.dump_on_exit as i32
}
/// Set dump-on-exit.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_dump_on_exit(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.dump_on_exit = value != 0;
}
/// Get trace-calls (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_trace_calls(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.trace_calls as i32
}
/// Set trace-calls.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_trace_calls(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.trace_calls = value != 0;
}
/// Get winapi emulation (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_emulate_winapi(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.emulate_winapi as i32
}
/// Set winapi emulation.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_emulate_winapi(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.emulate_winapi = value != 0;
}
/// Get short-circuit-sleep (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_short_circuit_sleep(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.short_circuit_sleep as i32
}
/// Set short-circuit-sleep.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_short_circuit_sleep(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.short_circuit_sleep = value != 0;
}
/// Get soft heap-free (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_heap_free_soft(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.heap_free_soft as i32
}
/// Set soft heap-free.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_heap_free_soft(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.heap_free_soft = value != 0;
}
/// Get SSDT LdrInitializeThunk usage (1/0).
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_ssdt_use_ldr_initialize_thunk(emu: *mut MwemuEmu) -> i32 {
    emu!(emu, 0).cfg.ssdt_use_ldr_initialize_thunk as i32
}
/// Set SSDT LdrInitializeThunk usage.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_ssdt_use_ldr_initialize_thunk(emu: *mut MwemuEmu, value: i32) {
    emu!(emu, ()).cfg.ssdt_use_ldr_initialize_thunk = value != 0;
}

/// Set trace flags on/off.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_trace_flags(emu: *mut MwemuEmu, value: i32) {
    let e = emu!(emu, ());
    e.cfg.trace_flags = value != 0;
}

/// Open the trace file configured via `mwemu_set_trace_filename`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_open_trace_file(emu: *mut MwemuEmu) {
    let e = emu!(emu, ());
    e.open_trace_file();
}

// String cfg getters (free result with `mwemu_free_string`) and setters.

/// Get the module name. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_module_name(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.module_name.clone())
}
/// Set the module name.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_module_name(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.module_name = cstr!(value, ()).to_string();
}
/// Get the exe name. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_exe_name(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.exe_name.clone())
}
/// Set the exe name.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_exe_name(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.exe_name = cstr!(value, ()).to_string();
}
/// Get the user name. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_user_name(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.user_name.clone())
}
/// Set the user name.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_user_name(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.user_name = cstr!(value, ()).to_string();
}
/// Get the temp path. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_temp_path(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.temp_path.clone())
}
/// Set the temp path.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_temp_path(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.temp_path = cstr!(value, ()).to_string();
}
/// Get the current working directory path. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_cwd_path(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.cwd_path.clone())
}
/// Set the current working directory path.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_cwd_path(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.cwd_path = cstr!(value, ()).to_string();
}
/// Get the Windows directory. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_windows_directory(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.windows_directory.clone())
}
/// Set the Windows directory.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_windows_directory(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.windows_directory = cstr!(value, ()).to_string();
}
/// Get the System directory. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_system_directory(emu: *mut MwemuEmu) -> *mut c_char {
    ret_string(emu!(emu, std::ptr::null_mut()).cfg.system_directory.clone())
}
/// Set the System directory.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_system_directory(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    e.cfg.system_directory = cstr!(value, ()).to_string();
}

/// Set the host name.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_host_name(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    let s = cstr!(value, ());
    e.cfg.host_name = s.to_string();
}

// Optional-string cfg accessors: getter returns NULL when unset; setter clears
// the field when passed a NULL pointer.

fn opt_string_get(field: &Option<String>) -> *mut c_char {
    match field {
        Some(s) => ret_string(s.clone()),
        None => std::ptr::null_mut(),
    }
}

fn opt_string_parse(value: *const c_char) -> Option<Option<String>> {
    if value.is_null() {
        Some(None)
    } else {
        match unsafe { CStr::from_ptr(value) }.to_str() {
            Ok(s) => Some(Some(s.to_string())),
            Err(_) => {
                set_error("invalid utf-8 in string argument");
                None
            }
        }
    }
}

/// Get the dump filename, or NULL if unset. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_dump_filename(emu: *mut MwemuEmu) -> *mut c_char {
    opt_string_get(&emu!(emu, std::ptr::null_mut()).cfg.dump_filename)
}
/// Set the dump filename; a NULL pointer clears it.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_dump_filename(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    if let Some(v) = opt_string_parse(value) {
        e.cfg.dump_filename = v;
    }
}
/// Get the trace filename, or NULL if unset. Free with `mwemu_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_get_trace_filename(emu: *mut MwemuEmu) -> *mut c_char {
    opt_string_get(&emu!(emu, std::ptr::null_mut()).cfg.trace_filename)
}
/// Set the trace filename; a NULL pointer clears it.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_set_trace_filename(emu: *mut MwemuEmu, value: *const c_char) {
    let e = emu!(emu, ());
    if let Some(v) = opt_string_parse(value) {
        e.cfg.trace_filename = v;
    }
}

// ---------------------------------------------------------------------------
// Buffer/string deallocators
// ---------------------------------------------------------------------------

/// Free a `char*` returned by this library.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

/// Free a byte buffer returned by this library.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_free_buffer(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len != 0 {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)));
        }
    }
}

/// Free a `uint64_t` array returned by this library.
#[unsafe(no_mangle)]
pub extern "C" fn mwemu_free_u64_buffer(ptr: *mut u64, count: usize) {
    if !ptr.is_null() && count != 0 {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(ptr, count)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cs(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn shellcode_step_reads_register() {
        // mov rax, 1 ; mov rbx, 2 ; add rax, rbx  => rax should be 3
        let code: [u8; 14] = [
            0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax,1
            0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // mov rbx,2
        ];
        let emu = mwemu_init64();
        assert!(!emu.is_null());
        mwemu_load_code_bytes(emu, code.as_ptr(), code.len());
        // step the two movs
        assert_eq!(mwemu_step(emu), 1);
        assert_eq!(mwemu_step(emu), 1);

        let reg = cs("rax");
        let mut rax: u64 = 0;
        assert_eq!(mwemu_get_reg(emu, reg.as_ptr(), &mut rax), 1);
        assert_eq!(rax, 1);

        let regb = cs("rbx");
        let mut rbx: u64 = 0;
        assert_eq!(mwemu_get_reg(emu, regb.as_ptr(), &mut rbx), 1);
        assert_eq!(rbx, 2);

        mwemu_free_emu(emu);
    }

    #[test]
    fn set_reg_returns_previous() {
        let emu = mwemu_init64();
        let reg = cs("rcx");
        let mut prev: u64 = 123;
        assert_eq!(mwemu_set_reg(emu, reg.as_ptr(), 0xdead, &mut prev), 1);
        let mut now: u64 = 0;
        assert_eq!(mwemu_get_reg(emu, reg.as_ptr(), &mut now), 1);
        assert_eq!(now, 0xdead);
        mwemu_free_emu(emu);
    }

    #[test]
    fn invalid_register_sets_error() {
        let emu = mwemu_init64();
        let reg = cs("not_a_reg");
        let mut out: u64 = 0;
        assert_eq!(mwemu_get_reg(emu, reg.as_ptr(), &mut out), 0);
        assert!(!mwemu_last_error().is_null());
        mwemu_free_emu(emu);
    }

    #[test]
    fn null_handle_is_safe() {
        let mut out: u64 = 0;
        let reg = cs("rax");
        assert_eq!(mwemu_get_reg(std::ptr::null_mut(), reg.as_ptr(), &mut out), 0);
        assert!(!mwemu_last_error().is_null());
    }

    #[test]
    fn alloc_write_read_roundtrip() {
        let emu = mwemu_init64();
        let name = cs("scratch");
        let mut base: u64 = 0;
        assert_eq!(mwemu_alloc(emu, name.as_ptr(), 0x1000, MWEMU_PERM_RWX, &mut base), 1);
        assert!(base != 0);
        assert_eq!(mwemu_write_qword(emu, base, 0x1122334455667788), 1);
        let mut v: u64 = 0;
        assert_eq!(mwemu_read_qword(emu, base, &mut v), 1);
        assert_eq!(v, 0x1122334455667788);
        mwemu_free_emu(emu);
    }
}
