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

## macOS (kexts)

The kext loader (`Emu::load_kext_macho64`, `emu/loaders/kext.rs`) links a plain
BSD (kmod) kext against the emulated XNU kernel. Unlike a `.ko` (an ET_REL with
section relocations) an arm64e kext is an already-placed image: its segments
carry final image-relative vmaddrs and its fixups are a chained-pointer list
(`LC_DYLD_CHAINED_FIXUPS`, pointer format `ARM64E_KERNEL` = 7, a 4-byte-stride
variant decoded in `macho64.rs`). Loading places the segments contiguously at
the module base, applies the fixups (rebases become `base + target`, binds the
address of an interceptable kernel stub), then reads `kmod_info` for the module
name and its `start`/`stop` entry points — the kext equivalent of
`init_module`/`cleanup_module`.

The instruction-level memory guard, previously x86-only, is now wired into the
AArch64 load/store handlers too (gated on `kernel_guard`, so ordinary userspace
runs pay only a bool check). That is what lets a use-after-free *read* inside an
arm64e driver be reported, not just the API-level ones — and it makes
`--memory-guard` effective for AArch64 macOS binaries as well.

Implemented XNU surface: `IOMalloc`/`IOMallocZero`/`IOFree` (and the `kalloc`
spellings), `IOLog`/`printf`/`kprintf`, the `IOLock`/`lck_mtx` family as no-ops
(single-threaded), and the generic libkern string/memory helpers (`memcpy`,
`memmove`, `memset`, …) which fall through to the shared, guarded Linux
implementations. Still declared only: the `OSObject`/`OSMetaClass` runtime and
the IOKit matching surface (`libmwemu::kernel::macos::SURFACE`), i.e. C++ IOKit
drivers.

x86_64 kexts use classic external relocations (`LC_DYSYMTAB`) rather than
chained fixups, so only arm64e kexts take this path for now.

## Windows (`.sys`)

A `.sys` is a PE64 with `DriverEntry` as its entry point and imports from
`ntoskrnl.exe`, `HAL.dll`, `FLTMGR.SYS`, `WDFLDR.SYS`. The loader
(`Emu::load_sys_driver`, `emu/loaders/sys.rs`, auto-detected by
`load_code`/`load_kernel_module` from the native subsystem plus the kernel
imports) maps the image at the kernel module base, applies the base
relocations, seeds `__security_cookie` through the Load Config directory
(the driver's own `__security_init_cookie` fast-fails otherwise) and binds
every import to the emulated kernel: a stub for a function, zeroed storage
for a data export (`IoDeviceObjectType`, `SeExports`, `NtBuildNumber`, …).
Only x64 drivers for now.

A driver is driven the way the I/O manager drives it, and the objects it
reads are built with the real x64 layouts (`kernel/windows/structs.rs`),
because `IoGetCurrentIrpStackLocation`, `ExInitializeFastMutex`,
`MmGetSystemAddressForMdlSafe` and friends are inline in the DDK headers:

- `Emu::run_module_init()` / `run_driver_entry()` builds a `DRIVER_OBJECT`
  (every `MajorFunction` slot pre-filled with `IopInvalidDeviceRequest`) and
  a registry path, calls `DriverEntry`, then captures the dispatch table
  (exposed as module symbols `IRP_MJ_DEVICE_CONTROL`, …), `DriverUnload`,
  `AddDevice` and the devices created through `IoCreateDevice(Secure)`.
- `Emu::kernel_ioctl(device, code, input, output_len)` sends
  `IRP_MJ_DEVICE_CONTROL` with the buffering the control code's method asks
  for (buffered: `SystemBuffer`; direct: `MdlAddress`; neither:
  `Type3InputBuffer`/`UserBuffer`), with the caller's buffers mapped in the
  user-mode range so `ProbeForRead`/`MmUserProbeAddress` checks behave. The
  result carries the NTSTATUS, `IoStatus`, the output buffer, whether
  `IofCompleteRequest` ran, and the privileged operations the request made.
  `kernel_create`/`kernel_close`/`kernel_read`/`kernel_write`/`kernel_send_irp`
  cover the other requests a user-mode client can send.
- `Emu::kernel_ioctl_candidates()` harvests control codes from the driver's
  code (immediates and `add/sub reg, -base; cmp reg, range` switch tables);
  `kernel_scan_ioctls()` sends each with several input sizes and reports
  what it reached — the BYOVD starting point.
- Every request runs under an instruction budget (`kernel_set_call_budget`,
  default 5M) so a driver polling a register cannot hang the run.
- A bugcheck, `__fastfail` (`int 0x29`), raised status, `ProbeForRead`
  violation or unhandled exception *aborts* the call: `kernel_call` (and so
  every `kernel_ioctl`) returns an error naming the cause instead of whatever
  was in `rax`, the reason is logged as `ABORT:` in the kernel log, and the
  next call starts clean. A fuzzer therefore cannot mistake a crash for a
  successful request.
- `MmGetSystemRoutineAddress` resolves names in the modelled surface to
  stubs and returns NULL for anything else, so a driver probing for an
  optional API takes its fallback path.

### Privileged operations

A BYOVD (bring-your-own-vulnerable-driver) hunt is a search for an IOCTL
that hands user mode a ring-0 primitive. Those are logged, not blocked, in
`KernelEnv::privops` (`Emu::kernel_privops()`, each with the faulting PC and
step): `rdmsr`/`wrmsr` (the MSR file is modelled, `kernel_set_msr` presets
one), `in`/`out` (`kernel_set_port`), `map_io_space`/`unmap_io_space`
(`MmMapIoSpace`, `MmMapVideoDisplay`, `ZwMapViewOfSection` on
`\Device\PhysicalMemory`), `phys_read` (`MmCopyMemory`), `virt_to_phys`,
`pci_read`/`pci_write` (`HalGetBusDataByOffset`), `ipi_call`
(`KeIpiGenericCall`). `cli`/`sti`, `rdpmc` and the port and MSR instructions
are implemented in the engine for kernel mode. A physical-memory window is a
pool chunk, so a stale use after `MmUnmapIoSpace` reports like any other
use-after-free.

CLI: `mwemu -f driver.sys -6` runs DriverEntry and prints devices, links and
dispatch; `--ioctl CODE[:INPUT_HEX[:OUT_LEN]]` sends one request;
`--ioctl-scan` runs the scan. MCP: `mwemu_kernel_driver`, `mwemu_kernel_ioctl`,
`mwemu_kernel_send_irp`, `mwemu_kernel_scan_ioctls`, `mwemu_kernel_privops`,
`mwemu_kernel_preset`. Python: `kernel_ioctl`, `kernel_devices`,
`kernel_dispatch_table`, `kernel_scan_ioctls`, `kernel_privops`,
`kernel_set_msr`, `kernel_set_port`.

### Implemented Windows surface

`libmwemu::kernel::windows::SURFACE` (and `mwemu_kernel_surface` with
`os=windows`) lists it at run time, grouped by file:

- **pool** — every `ExAllocatePool*`/`ExFreePool*` spelling, `MmAllocate*`,
  lookaside lists (the header's `Allocate`/`Free` point at the pool stubs so
  the DDK's inline `ExAllocateFromNPagedLookasideList` works), `SLIST`
  push/pop/flush in the real 16-byte x64 layout, `ExInterlocked*List`.
- **io** — `IoCreateDevice`/`IoCreateDeviceSecure`, symbolic links,
  `IofCompleteRequest`, IRP allocation/initialisation, stack-location helpers,
  `IoBuildDeviceIoControlRequest`, `IofCallDriver` (a modelled lower driver
  completes it), MDLs, work items (queued, not run), device interfaces,
  remove locks, `IoGetDeviceObjectPointer` against the driver's own devices.
- **mm** — `MmMapIoSpace`, `MmCopyMemory`, `MmGetPhysicalAddress`,
  `MmGetSystemRoutineAddress` (resolves to an interceptable stub, so a
  run-time lookup of `ZwQuerySystemInformation` is still intercepted), MDL
  probe/lock/map, `MmAllocatePagesForMdl`, `ProbeForRead`/`Write`, HAL bus
  data and address translation.
- **rtl** — `UNICODE_STRING`/`ANSI_STRING` family, `RtlGetVersion` (Windows
  10 22H2), registry helpers (answer "not found" so defaults apply), the
  security-descriptor/ACL/SID group that `wdmsec`'s statically linked
  `IoCreateDeviceSecure` depends on, `RtlImageNtHeader`/
  `RtlImageDirectoryEntryToData`, bitmaps, AVL table stubs.
- **crt** — `memcpy`/`memset`/`strlen`/`wcs*`/`_wcsnicmp`/…, the `printf`
  family (narrow, wide, `va_list`; `%ws`, `%wZ`, `%Z`, `%I64x`), `DbgPrint`/
  `DbgPrintEx`/`vDbgPrintEx`, `__C_specific_handler`.
- **sync** — spin locks, fast mutexes, ERESOURCE, push locks, rundown
  protection (uncontended by construction), events/semaphores/mutexes
  (modelled for real), DPCs and timers (queued, drained by
  `kernel_run_deferred`), `KeIpiGenericCall` and
  `KeExpandKernelStackAndCallout` (tail calls into the driver), processor,
  affinity and time queries, `KeBugCheckEx` (stops the run).
- **misc** — `Ps*` (a fake System process and thread, notify routines
  recorded, system threads queued), `Ob*` (handles remembered), `Se*`,
  `Zw*` (registry and files answered benignly; `ZwQuerySystemInformation`
  reports a one-module kernel; `\Device\PhysicalMemory` sections logged),
  `Cm*`, `Flt*`, `Etw*`, `Po*`, `WdfVersionBind` (fills the `WdfFunctions`
  table with per-index stubs; the KMDF object model itself is not modelled,
  so a pure KMDF driver gets through DriverEntry but never sees
  `EvtDeviceAdd`).

An import outside the surface still resolves to a stub that returns 0 and
is listed in `module.unresolved` at load time and `KernelEnv::unimplemented`
when called. Not modelled yet: SEH dispatch in kernel mode (`__try` around
`ProbeForRead` ends the run instead of unwinding), `mov cr*`, APC delivery,
x86 (32-bit) and ARM64 drivers, the KMDF object model.

`drivers/windows/tlm` is the Windows twin of the Linux target: the same
stale-hot-channel use-after-free plus an `MmMapIoSpace` and an
`rdmsr`/`wrmsr` IOCTL behind a magic value. `make windows-driver` builds it
with mingw-w64 into `test/windows_uaf_driver.sys` and
`cargo test -p libmwemu tests::kernel::windows` drives it end to end.

## Using it

Rust:

```rust
let mut emu = libmwemu::emu64();
emu.load_kernel_module("driver.ko")?;   // or driver.sys: the format is detected
emu.run_module_init()?;                       // insmod
emu.call_module_symbol("drv_ioctl", &[0, cmd, argp])?;
for f in emu.kernel_findings() {
    println!("{}", f.report());
}
```

Windows:

```rust
let mut emu = libmwemu::emu64();
emu.load_kernel_module("driver.sys")?;
emu.run_module_init()?;                                   // DriverEntry
let r = emu.kernel_ioctl(0, 0x9c402118, &0xc000_0082u32.to_le_bytes(), 8)?;
for op in &r.privops { println!("{}", op.report()); }      // rdmsr 0xc0000082 -> ...
```

CLI — links the module and leaves PC at its init, so a plain run does what
`insmod` does:

```sh
mwemu -f driver.ko -6 -v
mwemu -f driver.sys -6 --ioctl-scan
```

MCP — see the kernel-mode section of `crates/mwemu-mcp/README.md`.

## Test target

`drivers/macos/tlm` is the kext counterpart: the same deliberately vulnerable
telemetry driver as a BSD kmod. `make macos-driver` builds it into
`test/macos_uaf_driver.kext` (arm64e, needs the macOS SDK) and
`cargo test -p libmwemu tests::kernel::macos_uaf` drives it end to end. Like the
Linux target it is loaded only inside mwemu, never `kextload`ed.

`drivers/linux/tlm` is a deliberately vulnerable telemetry driver used as the
reference target; `make driver` builds it into `test/linux_uaf_driver.ko` and
`cargo test -p libmwemu tests::kernel` drives it end to end.
