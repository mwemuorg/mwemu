# Kernel-mode emulation (drivers)

Emulating a driver is not emulating a program with different imports. A driver
has no entry point, no libc, no loader and no process: it is an object file that
an operating system links into its own address space and then calls back into.
mwemu's kernel mode supplies the three things that are missing, and nothing
else:

1. **A linker.** `.ko` images are ET_REL relocatable objects — no program
   headers, no `.dynamic`, sections that have not been placed. Placement and
   relocation happen at load time (`rs_header::elf::relocatable`, driven by
   `Emu::load_kernel_module`).
2. **A kernel to call.** Every imported symbol is resolved to an address in a
   synthetic "kernel text" region; a call landing there is intercepted and
   routed to a Rust implementation, the same mechanism the winapi layer uses.
3. **An allocator with a memory.** Driver bugs are lifetime bugs, so the slab is
   modelled explicitly: chunks are tracked with their provenance, freed chunks
   go to quarantine instead of being recycled, and every access is checked
   against the ledger.

Point 3 is the reason this exists. A real slab hands freed memory straight back
out, which is exactly what makes a use-after-free hard to see; keeping the chunk
mapped and poisoned turns it into a report.

## Layout

```
crates/libmwemu/src/kernel/
    mod.rs        KernelOs, KernelEnv, stub allocation, call interception
    layout.rs     address-space plan per OS
    heap.rs       allocation ledger (pure bookkeeping)
    guard.rs      memory-safety verdicts and findings
    linux/        mm, string, sync, printk, module, misc
    windows/      ntoskrnl surface (pool allocators implemented)
    macos/        XNU / IOKit surface (allocators implemented)
crates/libmwemu/src/emu/loaders/ko.rs   the ET_REL load path
crates/libmwemu/libraries/rs-header/src/elf/relocatable.rs   ET_REL parsing
```

Only the symbol tables and the handlers differ between the three OSes;
placement, interception, the ledger and the analysis are shared. That is why a
`.sys` will inherit the whole use-after-free analysis the day its loader lands.

## Address space

Chosen to match the real layouts, and — more importantly — so the distances
between regions stay inside what the relocations can encode. A module built
with `-mcmodel=kernel` reaches the kernel through `R_X86_64_PLT32`, a signed
32-bit displacement, so module and stub area must be within ±2GB.

| region | Linux | purpose |
| --- | --- | --- |
| kernel text (stubs) | `0xffffffff81000000` | one interceptable slot per imported function |
| kernel data | `0xffffffff82000000` | storage for imported variables (`jiffies`, …) |
| module image | `0xffffffffc0000000` | the loaded `.ko` |
| slab | `0xffff888000000000` | `kmalloc` / `kmem_cache_alloc` chunks |
| vmalloc | `0xffffc90000000000` | `vmalloc` / page allocations |
| kernel stack | `0xffffc90000100000` | |

Chunks are separated by an unmapped redzone, so a linear overflow off the end
of an allocation faults instead of corrupting the next one.

## Detection

| finding | how it is decided |
| --- | --- |
| `use_after_free_read` / `use_after_free_write` | access lands in a quarantined chunk |
| `use_after_free_poison_deref` | the *address* is slab free poison (`0x6b6b…`), i.e. the pointer was loaded out of a freed object |
| `use_after_free_call` | an indirect branch target came out of quarantine |
| `double_free` | free of a chunk already in quarantine |
| `invalid_free` | free of something that is not a chunk base |
| `slab_out_of_bounds` | access past the requested size, inside the slab bucket |
| `memory_leak` | still live after the module's exit path ran |

Each finding carries the faulting instruction, the object, its cache, and both
the allocation and the free site. Repeats of the same (kind, instruction,
object) collapse into a hit count.

Freed memory stays mapped, so execution continues after the first stale
dereference and one run can surface the whole chain rather than stopping at the
first symptom.

## Implemented Linux surface

`mwemu_kernel_surface` (MCP) and `libmwemu::kernel::linux::SURFACE` return this
list at runtime.

**Allocation** — every spelling funnels into one ledger, so it does not matter
which kernel version the driver was built against:

`__kmalloc`, `__kmalloc_noprof`, `__kmalloc_node`, `__kmalloc_node_noprof`,
`__kmalloc_cache_noprof`, `__kmalloc_cache_node_noprof`, `__kmalloc_large_noprof`,
`kmalloc_trace`, `kcalloc`, `kmalloc_array`, `kfree`, `kfree_sensitive`,
`krealloc`, `krealloc_noprof`, `kmemdup`, `kstrdup`, `kstrndup`,
`kmem_cache_create`, `kmem_cache_create_usercopy`, `__kmem_cache_create_args`,
`kmem_cache_destroy`, `kmem_cache_alloc`, `kmem_cache_alloc_noprof`,
`kmem_cache_zalloc`, `kmem_cache_free`, `vmalloc`, `vzalloc`, `__vmalloc`,
`vfree`, `kvmalloc`, `kvmalloc_node_noprof`, `kvzalloc`, `kvfree`,
`__get_free_pages`, `get_zeroed_page`, `free_pages`, `__free_pages`,
`alloc_pages`, `devm_kmalloc`, `devm_kzalloc`, `devm_kfree`.

`kzalloc`, `kcalloc` and friends are inline wrappers in the kernel headers, so
they never appear as symbols — they arrive as `__GFP_ZERO` on one of the above.

**User copies** — `copy_from_user`, `_copy_from_user`, `copy_to_user`,
`_copy_to_user`, `clear_user`, `strncpy_from_user`, `memdup_user`,
`vmemdup_user`, `__check_object_size`, `validate_usercopy_range`.

**String / lib** — `memcpy`, `memmove`, `memset`, `memcmp`, `strlen`, `strnlen`,
`strcmp`, `strncmp`, `strcpy`, `strncpy`, `strscpy`, `sized_strscpy`, `strlcpy`,
`strcat`, `strchr`, `strrchr`, `strstr`, `snprintf`, `scnprintf`, `sprintf`,
`simple_strtoul`, `kstrtoint`, `kstrtoul`, `kstrtouint`, `kstrtou32`.

These run through the guard too: a `memcpy()` into a freed object is a
use-after-free that no instruction-level check would see, because the copy
happens inside the kernel, not in the driver's own code.

**Object lifetime** — `refcount_inc`, `refcount_dec`, `refcount_dec_and_test`,
`refcount_add`, `refcount_sub_and_test`, `refcount_inc_not_zero`,
`refcount_warn_saturate`, `kref_get`, `kref_put`. Modelled for real, not
stubbed: in a driver the refcount *is* the object lifetime, and
`refcount_dec_and_test()` returning true is what triggers the free.

**Locking** — `mutex_lock`, `mutex_unlock`, `mutex_trylock`, `__mutex_init`,
`mutex_init_generic`, `_raw_spin_lock`, `_raw_spin_unlock`,
`_raw_spin_lock_irqsave`, `_raw_spin_unlock_irqrestore`, `down_read`, `up_read`,
`down_write`, `up_write`, `synchronize_rcu`, `rcu_read_lock`, `rcu_read_unlock`,
`__list_add_valid_or_report`, `__list_del_entry_valid_or_report`. No-ops:
single-threaded emulation cannot deadlock.

**Deferred work** — `queue_work_on`, `schedule_work`, `queue_delayed_work_on`,
`flush_work`, `cancel_work_sync`, `cancel_delayed_work_sync`, `mod_timer`,
`add_timer`, `del_timer_sync`, `timer_delete_sync`, `call_rcu`,
`kvfree_call_rcu`, `kthread_create_on_node`, `wake_up_process`, `kthread_stop`.

Callbacks are **queued**, not run inline. "Unregister now, free later" is the
shape of most kernel use-after-free bugs, and running the callback immediately
would close the very window the bug lives in. Drain them with
`Emu::kernel_run_deferred()` / `mwemu_kernel_run_deferred`.

**Logging** — `printk`, `_printk`, `_dev_info`, `_dev_warn`, `_dev_err`,
`__warn_printk`, `panic`, `dump_stack`, `__stack_chk_fail`, `__fortify_panic`.

**Registration** — `__register_chrdev`, `__unregister_chrdev`,
`alloc_chrdev_region`, `register_chrdev_region`, `unregister_chrdev_region`,
`cdev_init`, `cdev_add`, `cdev_del`, `misc_register`, `misc_deregister`,
`class_create`, `class_destroy`, `device_create`, `device_destroy`,
`proc_create`, `remove_proc_entry`, `debugfs_create_file`, `debugfs_create_dir`,
`debugfs_remove`, `try_module_get`, `module_put`.

**Time / misc** — `msleep`, `usleep_range`, `ssleep`, `ktime_get`,
`ktime_get_real_ts64`, `get_random_bytes`, `get_random_u32`, `capable`.

**Imported variables** (resolved to storage, not stubs) — `jiffies`,
`jiffies_64`, `kmalloc_caches`, `current_task`, `cpu_number`, `__preempt_count`,
`__per_cpu_offset`, `__stack_chk_guard`, `__ref_stack_chk_guard`, `system_wq`
and the other workqueue globals, `boot_cpu_data`, `init_task`, `init_net`,
`init_user_ns`, `init_mm`, `page_offset_base`, `vmemmap_base`, `physical_mask`,
`phys_base`, the `param_ops_*` tables, `pv_ops`, `empty_zero_page`, `max_pfn`,
`totalram_pages`.

An import with no implementation is not fatal: it is reported at load time
(`unresolved`), and if it is actually called the call returns 0 and the symbol
is recorded in `KernelEnv::unimplemented`. That way a partially covered kernel
still runs a driver as far as it can go.

## Windows and macOS

The surfaces are declared and their allocators implemented, so a `.sys` or a
kext inherits the lifetime analysis as soon as its loader lands:

- **Windows**: `ExAllocatePool2`, `ExAllocatePoolWithTag`, `ExFreePool`,
  `ExFreePoolWithTag`, `MmAllocateContiguousMemory`, `RtlCopyMemory`,
  `RtlZeroMemory`, `DbgPrint`/`DbgPrintEx`. Declared and still to implement: the
  MDL, IO manager, object manager and synchronisation groups
  (`libmwemu::kernel::windows::SURFACE`). What is missing is the *loader*: a
  `.sys` is a PE with a `DriverEntry`, so it needs the PE path with kernel-space
  placement rather than the ET_REL path.
- **macOS**: `IOMalloc`, `IOMallocZero`, `IOFree`, `kalloc_external`,
  `kfree_external`, `IOLog`. The missing piece is again the loader — a kext is a
  Mach-O `MH_KEXT_BUNDLE` with external relocations.

## Using it

Rust:

```rust
let mut emu = libmwemu::emu64();
emu.load_kernel_module("driver.ko")?;
emu.run_module_init()?;                       // insmod
emu.call_module_symbol("drv_ioctl", &[0, cmd, argp])?;
for f in emu.kernel_findings() {
    println!("{}", f.report());
}
```

CLI — links the module and leaves PC at its init, so a plain run does what
`insmod` does:

```sh
mwemu -f driver.ko -6 -v
```

MCP — see the kernel-mode section of `crates/mwemu-mcp/README.md`.

## Test target

`drivers/linux/tlm` is a deliberately vulnerable telemetry driver used as the
reference target; `make driver` builds it into `test/linux_uaf_driver.ko` and
`cargo test -p libmwemu tests::kernel` drives it end to end.
