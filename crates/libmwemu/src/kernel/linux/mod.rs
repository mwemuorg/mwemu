//! The Linux kernel API surface available to an emulated `.ko`.
//!
//! Dispatch is split by subsystem so each file has one job and the list of what
//! is implemented stays readable. The order below is by call frequency in real
//! driver code, so the common path takes the fewest comparisons.
//!
//! Coverage is intentionally partial: everything a driver needs to *allocate,
//! free, copy and log* is implemented, because that is what memory-safety
//! analysis depends on. The rest of the kernel is declared in [`SURFACE`] and
//! answered with a benign zero, which is enough to keep a driver running
//! through its own logic. An unimplemented symbol that is actually called is
//! recorded in `KernelEnv::unimplemented` rather than aborting the run.

pub mod misc;
pub mod mm;
pub mod module;
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

/// Every symbol this surface knows about, grouped the way the modules are.
/// Used for `--kernel-surface`-style reporting: it answers "what can mwemu
/// emulate for a driver today?" without running anything.
pub const SURFACE: &[(&str, &[&str])] = &[
    (
        "alloc",
        &[
            "__kmalloc",
            "__kmalloc_noprof",
            "__kmalloc_node",
            "__kmalloc_node_noprof",
            "__kmalloc_cache_noprof",
            "__kmalloc_cache_node_noprof",
            "__kmalloc_large_noprof",
            "kmalloc_trace",
            "kcalloc",
            "kmalloc_array",
            "kfree",
            "kfree_sensitive",
            "krealloc",
            "krealloc_noprof",
            "kmemdup",
            "kstrdup",
            "kstrndup",
            "kmem_cache_create",
            "kmem_cache_create_usercopy",
            "__kmem_cache_create_args",
            "kmem_cache_destroy",
            "kmem_cache_alloc",
            "kmem_cache_alloc_noprof",
            "kmem_cache_zalloc",
            "kmem_cache_free",
            "vmalloc",
            "vzalloc",
            "__vmalloc",
            "vfree",
            "kvmalloc",
            "kvmalloc_node_noprof",
            "kvzalloc",
            "kvfree",
            "__get_free_pages",
            "get_zeroed_page",
            "free_pages",
            "__free_pages",
            "alloc_pages",
            "devm_kmalloc",
            "devm_kzalloc",
            "devm_kfree",
        ],
    ),
    (
        "usercopy",
        &[
            "copy_from_user",
            "_copy_from_user",
            "copy_to_user",
            "_copy_to_user",
            "clear_user",
            "strncpy_from_user",
            "memdup_user",
            "vmemdup_user",
            "__check_object_size",
            "validate_usercopy_range",
        ],
    ),
    (
        "string",
        &[
            "memcpy",
            "memmove",
            "memset",
            "memcmp",
            "strlen",
            "strnlen",
            "strcmp",
            "strncmp",
            "strcpy",
            "strncpy",
            "strscpy",
            "strlcpy",
            "strcat",
            "strchr",
            "strrchr",
            "strstr",
            "snprintf",
            "scnprintf",
            "sprintf",
            "simple_strtoul",
            "kstrtoint",
            "kstrtoul",
            "kstrtouint",
            "kstrtou32",
            "sized_strscpy",
        ],
    ),
    (
        "lifetime",
        &[
            "refcount_inc",
            "refcount_dec",
            "refcount_dec_and_test",
            "refcount_add",
            "refcount_sub_and_test",
            "refcount_inc_not_zero",
            "refcount_warn_saturate",
            "kref_get",
            "kref_put",
        ],
    ),
    (
        "locking",
        &[
            "mutex_lock",
            "mutex_unlock",
            "mutex_trylock",
            "__mutex_init",
            "_raw_spin_lock",
            "_raw_spin_unlock",
            "_raw_spin_lock_irqsave",
            "_raw_spin_unlock_irqrestore",
            "down_read",
            "up_read",
            "down_write",
            "up_write",
            "synchronize_rcu",
            "rcu_read_lock",
            "rcu_read_unlock",
            "mutex_init_generic",
            "__list_add_valid_or_report",
            "__list_del_entry_valid_or_report",
        ],
    ),
    (
        "deferred",
        &[
            "queue_work_on",
            "schedule_work",
            "queue_delayed_work_on",
            "flush_work",
            "cancel_work_sync",
            "cancel_delayed_work_sync",
            "mod_timer",
            "add_timer",
            "del_timer_sync",
            "timer_delete_sync",
            "call_rcu",
            "kvfree_call_rcu",
            "kthread_create_on_node",
            "wake_up_process",
            "kthread_stop",
        ],
    ),
    (
        "logging",
        &[
            "printk",
            "_printk",
            "_dev_info",
            "_dev_warn",
            "_dev_err",
            "__warn_printk",
            "panic",
            "dump_stack",
            "__stack_chk_fail",
            "__fortify_panic",
        ],
    ),
    (
        "registration",
        &[
            "__register_chrdev",
            "__unregister_chrdev",
            "alloc_chrdev_region",
            "register_chrdev_region",
            "unregister_chrdev_region",
            "cdev_init",
            "cdev_add",
            "cdev_del",
            "misc_register",
            "misc_deregister",
            "class_create",
            "class_destroy",
            "device_create",
            "device_destroy",
            "proc_create",
            "remove_proc_entry",
            "debugfs_create_file",
            "debugfs_create_dir",
            "debugfs_remove",
            "try_module_get",
            "module_put",
        ],
    ),
    (
        "time",
        &[
            "msleep",
            "usleep_range",
            "ssleep",
            "ktime_get",
            "ktime_get_real_ts64",
            "get_random_bytes",
            "get_random_u32",
            "capable",
        ],
    ),
];

/// True when the symbol has a real implementation (not just a benign stub).
pub fn is_implemented(name: &str) -> bool {
    SURFACE.iter().any(|(_, names)| names.contains(&name))
}
