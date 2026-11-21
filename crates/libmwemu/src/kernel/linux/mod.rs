//! The Linux kernel API surface available to an emulated `.ko`.
//!
//! Dispatch is split by subsystem so each file has one job and the list of what
//! is implemented stays readable. The order below is by call frequency in real
//! driver code, so the common path takes the fewest comparisons.
//!
//! Coverage is intentionally partial: everything a driver needs to *allocate,
//! free, copy and log* is implemented, because that is what memory-safety
//! analysis depends on. Dispatch declarations also supply [`SURFACE`]. Calls
//! without a handler return zero and record the symbol in
//! `KernelEnv::unimplemented`.

pub mod io;
pub mod misc;
pub mod mm;
pub mod module;
pub mod net;
pub mod printk;
pub mod string;
pub mod sync;

use crate::emu::Emu;

/// Route one kernel API call. Returns false when nothing implements it.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    mm::dispatch(symbol, emu)
        || string::dispatch(symbol, emu)
        || sync::dispatch(symbol, emu)
        || printk::dispatch(symbol, emu)
        || module::dispatch(symbol, emu)
        || net::dispatch(symbol, emu)
        || io::dispatch(symbol, emu)
        || misc::dispatch(symbol, emu)
}

/// Kernel symbols that name *variables*, not functions.
///
/// The distinction matters at link time: a data import must resolve to
/// readable storage, while a function import resolves to an interceptable stub
/// in the synthetic kernel text. Guessing wrong for data is loud (the driver
/// would execute its contents), so the list is explicit and anything not on it
/// is treated as a function.
///
/// The size is what the module may legitimately touch through the symbol.
pub fn data_symbol_size(name: &str) -> Option<u64> {
    let size = match name {
        "jiffies" | "jiffies_64" => 8,
        // kmalloc_caches[NR_KMALLOC_TYPES][KMALLOC_SHIFT_HIGH + 1]
        "kmalloc_caches" => 0x1000,
        "current_task" | "cpu_number" | "__preempt_count" | "__per_cpu_offset" => 0x100,
        "__stack_chk_guard" | "__ref_stack_chk_guard" => 8,
        "system_wq"
        | "system_highpri_wq"
        | "system_long_wq"
        | "system_unbound_wq"
        | "system_freezable_wq"
        | "system_power_efficient_wq" => 8,
        "boot_cpu_data" | "init_task" | "init_net" | "init_user_ns" | "init_mm" => 0x400,
        "page_offset_base" | "vmemmap_base" | "physical_mask" | "phys_base" => 8,
        "param_ops_int" | "param_ops_uint" | "param_ops_long" | "param_ops_ulong"
        | "param_ops_charp" | "param_ops_bool" | "param_ops_string" | "param_ops_short" => 0x40,
        "pv_ops" | "static_key_initialized" | "__tracepoint_module_get" => 0x200,
        "empty_zero_page" | "mem_section" | "max_pfn" | "totalram_pages" => 0x100,
        _ => return None,
    };
    Some(size)
}

/// Exact callable names declared by the dispatchers, grouped by subsystem.
/// Prefix-based partial models are also recognized by `is_implemented`.
pub const SURFACE: &[(&str, &[&str])] = &[
    ("alloc", mm::SURFACE),
    ("string", string::SURFACE),
    ("sync", sync::SURFACE),
    ("logging", printk::SURFACE),
    ("registration", module::SURFACE),
    ("net", net::SURFACE),
    ("io", io::SURFACE),
    ("misc", misc::SURFACE),
];

/// True when dispatch has a handler or a deliberate partial model for this name.
pub fn is_implemented(name: &str) -> bool {
    mm::has_handler(name)
        || string::has_handler(name)
        || sync::has_handler(name)
        || printk::has_handler(name)
        || module::has_handler(name)
        || net::has_handler(name)
        || io::has_handler(name)
        || misc::has_handler(name)
}
