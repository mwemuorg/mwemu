# Kernel-mode emulation

mwemu loads driver images into an emulated kernel address space, resolves their imports, and calls their entry points and callbacks. Kernel API models provide allocation, memory access, logging, and selected operating-system services. The allocation ledger records object lifetimes for memory-safety analysis.

## Supported images

| Platform | Image and architecture | Loading |
| --- | --- | --- |
| Linux | ELF64 `ET_REL` `.ko`, x86-64 or AArch64 | `Emu::load_kernel_module` places sections, resolves symbols, and applies supported relocations. |
| Windows | PE32+ `.sys`, x64 | `Emu::load_sys_driver` maps the image, applies base relocations, binds imports, and initializes the security cookie. `load_kernel_module` also recognizes these drivers. |
| macOS | Mach-O BSD kmod kext, arm64e | `Emu::load_kext_macho64` maps segments, applies supported chained fixups, and reads the name and start/stop addresses from `kmod_info`. |

Loading does not run initialization. Call `run_module_init()` to enter the module. For Windows, this calls `run_driver_entry()` with a constructed `DRIVER_OBJECT` and registry path. `run_module_exit()` calls the recorded exit or unload routine when one exists.

Windows auto-detection accepts x64 PE images with the native subsystem and no `ntdll.dll` import. This is a loader heuristic, not a guarantee that every such image is a supported driver. The macOS path does not support the classic external relocations used by x86-64 kexts.

## Symbol discovery and handler coverage

Windows, Linux and macOS derive callable names and dispatch from the same platform registries. `SURFACE` lists exact names. `is_implemented(name)` also recognizes guarded partial models such as supported Linux `netif_*` prefixes. A registered name may still call a partial handler or a shared stub.

| Platform | Rust discovery | MCP discovery |
| --- | --- | --- |
| Windows | `libmwemu::kernel::windows::SURFACE` and `windows::is_implemented` | `mwemu_kernel_surface` with `{"os":"windows"}` |
| Linux | `libmwemu::kernel::linux::SURFACE` and `linux::is_implemented` | `mwemu_kernel_surface` with `{"os":"linux"}`. Linux is the default. |
| macOS | `libmwemu::kernel::macos::SURFACE` and `macos::is_implemented` | `mwemu_kernel_surface` with `{"os":"macos"}` |

Windows and Linux classify data exports separately and resolve them to storage. These exports do not appear in the callable surface. Function imports can resolve to interceptable stubs even without a handler.

The loaders report missing imports differently. Windows adds names without a handler or data model to `module.unresolved`. Linux uses that field for linker resolution failures, so assigning a stub can hide the absence of a handler from the list. The macOS loader currently leaves it empty. Use dispatch discovery and runtime diagnostics to assess API coverage.

Windows `MmGetSystemRoutineAddress` uses the same handler discovery and data-export classification. It returns an interceptable address or data storage for a recognized name, and `NULL` otherwise. Static imports can reach the unsupported-call policy while a dynamic lookup of the same unknown name returns `NULL`.

### Windows models

| Group | Current behavior |
| --- | --- |
| `alloc` | Pool and page allocation, lookaside lists, and list operations. Modeled allocations use the shared lifetime ledger. |
| `io` | Devices and symbolic links, IRP construction/completion, MDLs, work items, interfaces, and partial lower-driver behavior. |
| `mm` | Physical-memory windows, copies, address translation, MDL mapping, probes, routine lookup, and partial HAL operations. |
| `rtl` | String descriptors and conversion, version queries, PE inspection, bitmaps, and partial registry/security/table routines. |
| `sync` | Partial synchronization objects, lock/IRQL stubs, DPCs and timers, callback transfers, processor/time queries, and bugcheck handling. |
| `crt` | Memory and string operations, narrow/wide formatting, checked `va_list` reads, debug logging, and limited runtime support. |
| `misc` | Partial process, thread, handle, native-service, security, filter, tracing, power, and framework routines. |

Windows kernel imports use their registered Nt/Zw spellings. The selected API mode applies when a call reaches fallback.

`WdfVersionBind` fills a function table with stubs. It does not provide the KMDF object model or establish that a KMDF driver can complete initialization. Kernel SEH unwinding and APC delivery are not modeled. Windows x86 and ARM64 drivers are unsupported.

### Linux models

| Group | Current behavior |
| --- | --- |
| `alloc` | Slab, cache, vmalloc, page, and managed allocations. Frees, copies, and user-copy helpers. |
| `string` | Memory/string operations, integer conversion, and formatted output. Bulk-copy models use range-aware guard checks. |
| `sync` | Refcount operations and release callbacks, with simplified locking and synchronization. |
| `logging` | Kernel/device logs, warnings, and selected fatal runtime paths. |
| `registration` | Character, misc, class, device, proc, and debugfs registration models. |
| `net` | Partial network-device, packet-buffer, USB, and wireless models, including tracked allocations and registration capture. |
| `io` | Shared MMIO windows, DMA buffers, and partial IRQ/resource models. These do not emulate a physical device. |
| `misc` | Deferred work, timers, RCU callbacks, tasks, time, random values, and other partial services. |

Compiler and kernel versions can change which allocation wrapper names appear as imports. Discovery describes the names accepted by this emulator rather than a complete export list for a particular Linux version. Data exports such as `jiffies`, `current_task`, and workqueue globals have separate storage models.

`kernel_registered_drivers()` exposes captured registration information for a harness to inspect and invoke driver callbacks. Initialization alone does not exercise probe, request, or teardown paths.

### macOS models

The kext gateway shares selected memory and string routines with Linux. IOKit size-based allocation, free, console logging and single-threaded lock models are available. `IOMallocAligned` accepts power-of-two alignments through 16 bytes. Typed XNU allocators, structured logging and the IOKit C++ object runtime use the unknown-import path.

## Call frames, failures, and unsupported imports

The kernel gateway checks the x64 return slot or obtains the AArch64 link register before dispatch. An unreadable argument or return frame stops the call with the symbol and entry PC/SP. Earlier writes and allocations remain visible. Kernel formatters check their required variadic arguments before publishing output.

If the gateway finds no handler, it records the name once in `KernelEnv::unimplemented`. Windows applies the [Windows API policy](API.md#windows-api-policy), including status-only NT results where a signature exists. The Linux and macOS kernel fallback returns integer zero. The [handler ABI](API.md#handler-abi) describes the adapters and their supported argument layouts.

`kernel_call` returns an error for a policy stop or kernel abort. Clear a pending API fault explicitly before another call. A fresh call clears the separate kernel-abort marker. Guest state changes from the previous call remain visible.

Each `kernel_call` has a default budget of 5,000,000 instructions, combined with any tighter caller limit. `kernel_set_call_budget` changes it. Zero disables this per-call limit. Budget exhaustion is an error, not a successful guest return.

## Memory and findings

Each platform has a layout for synthetic kernel text, imported data, module code, slab/pool memory, large allocations, and the stack. The kernel boundary intercepts control transfers to function stubs. Linux x86-64 module and stub addresses stay within the signed displacement range used by relative calls.

Allocation models record the requested size, bucket size, cache, API, and allocation/free sites. Freeing a chunk fills it with poison bytes and keeps it mapped in quarantine. The allocator does not reuse it. Gaps separate allocations. These choices make stale accesses easier to detect, but they do not reproduce a real kernel allocator.

Kernel initialization enables `kernel_guard`. Instrumented x86 and AArch64 accesses consult the ledger, and selected host-side memory models use explicit guard checks. The single-address check identifies the chunk containing the starting address. Bulk range checks can also detect a copy whose length crosses the requested boundary. Coverage depends on the access path. An unmapped address or an empty finding list does not prove memory safety.

| Finding tag | Meaning |
| --- | --- |
| `use_after_free_read` / `use_after_free_write` | A checked access touches a quarantined allocation. |
| `use_after_free_poison_deref` | The address matches the free-poison pattern, including supported nearby offsets. |
| `use_after_free_call` | A branch targets an unmapped free-poison address. This is pattern detection, not general pointer-provenance tracking. |
| `double_free` | The allocation was already freed. |
| `invalid_free` | The pointer is not the base of a tracked allocation. |
| `slab_out_of_bounds` | A checked address or range exceeds the requested size of a tracked allocation. |
| `invalid_address` | A modeled memory operation reports an invalid address. |
| `memory_leak` | A tracked allocation is still live when `kernel_check_leaks()` runs. |

Findings retain the instruction, address, access size, and available allocation provenance. Repeated findings with the same kind, instruction, and object increase a hit count. Poison-only findings may have no original object identity. Freed memory can remain readable after a report, but later faults or invalid control flow can still stop the run.

Call `kernel_check_leaks()` explicitly after the intended cleanup and exit paths. It reports all remaining tracked allocations, so running it before cleanup can include objects that are still intentionally live. `kernel_set_fail_alloc(Some(index))` makes one allocation attempt return null. The index is zero-based. `kernel_alloc_count()` exposes the attempt count, and `None` disables injection.

## Deferred callbacks

Work, timer, RCU, DPC, and thread models can queue callbacks. `kernel_run_deferred()` processes up to 64 queued callbacks per invocation, including newly queued work within that bound. It returns the number attempted and logs callback failures. It is not an operating-system scheduler or a guarantee that every callback succeeded.

Some APIs transfer directly to a guest callback instead of queueing it, such as `KeIpiGenericCall`. Locking and interrupt models are simplified and do not validate real concurrency, race behavior, or deadlock freedom.

## Windows requests and privileged operations

`run_driver_entry()` builds x64 driver structures, calls the entry point, and captures the dispatch table, unload/AddDevice callbacks, and created devices. `kernel_devices()` and `kernel_dispatch_table()` expose the resulting state. A normal return still needs its guest `NTSTATUS` checked.

`kernel_ioctl(device, code, input, output_len)` constructs `IRP_MJ_DEVICE_CONTROL` using the method encoded in the control code: buffered, direct, or neither. The request builder maps buffers in the emulated user address range. A device value of zero selects the first created device. `kernel_create`, `kernel_close`, `kernel_read`, `kernel_write`, and `kernel_send_irp` cover other request paths.

Request setup failures return `Err`. Once dispatch runs, a call failure can instead appear in `IrpResult.error` with a failure status. Inspect that field together with `status`, `io_status`, `information`, `completed`, and `output`. `Ok(IrpResult)` alone does not mean a completed or successful request. Results also include observed privileged operations and newly recorded unsupported names.

`kernel_ioctl_candidates()` extracts candidate control codes from instruction patterns. `kernel_scan_ioctls()` tries candidates with several buffer sizes and retains selected results. This is a heuristic exploration aid, not exhaustive IOCTL discovery or proof of a vulnerability.

`kernel_privops()` reports modeled MSR, port, physical-memory, address-translation, PCI, and cross-processor operations with their PC and instruction count. `kernel_set_msr` and `kernel_set_port` supply emulated values. Physical-memory windows use tracked allocations. These observations show which modeled operations a path reached. They do not reproduce hardware behavior or establish exploitability.

## Usage

Load and initialize a Linux module, then inspect its result and findings:

```rust
let mut emu = libmwemu::emu64();
emu.load_kernel_module("driver.ko")?;
let result = emu.run_module_init()?;
println!("module init returned {result:#x}");
for finding in emu.kernel_findings() {
    println!("{}", finding.report());
}
```

Use `call_module_symbol(name, args)` or `kernel_call(address, args)` to exercise additional paths with the driver-specific arguments. Initialization does not infer suitable request data. Load a macOS kext with `load_kext_macho64` before using the same lifecycle methods.

For Windows, choose the unsupported-call policy explicitly:

```rust
use libmwemu::ApiMode;

let mut emu = libmwemu::emu64();
emu.set_api_mode(ApiMode::Strict);
emu.load_kernel_module("driver.sys")?;
let status = emu.run_module_init()?;
println!("DriverEntry returned {status:#x}");
```

CLI examples:

```sh
mwemu -f driver.ko -6 -v
mwemu -f driver.sys -6 --api-mode strict
mwemu -f driver.sys -6 --api-mode simulate-fail --ioctl-scan
```

`--ioctl CODE[:INPUT_HEX[:OUTPUT_LEN]]` sends one Windows request after initialization. Its control code and input layout must match the driver. The CLI uses the kernel initialization path and reports Windows devices and dispatch entries.

The [MCP kernel tools](../crates/mwemu-mcp/README.md#emulating-a-driver-and-finding-lifetime-bugs) expose module calls, findings, deferred work, leaks, and surface discovery. Windows tools also expose driver requests, IOCTL scans, privileged operations, and presets. Python bindings provide corresponding module-call, finding, device, request, and preset methods.

## Tests

| Target | Build command | Output | Required toolchain |
| --- | --- | --- | --- |
| `drivers/linux/tlm` | `make driver` | `test/linux_uaf_driver.ko` | Linux kernel headers and a matching build toolchain. |
| `drivers/windows/tlm` | `make windows-driver` | `test/windows_uaf_driver.sys` | mingw-w64 with DDK headers. |
| `drivers/macos/tlm` | `make macos-driver` | `test/macos_uaf_driver.kext` | macOS SDK and arm64e toolchain support. |

These deliberately vulnerable telemetry drivers exercise lifetime errors. The Windows target also exercises physical-memory and MSR paths. Load the artifacts inside mwemu for testing.

```sh
cargo test -p libmwemu tests::kernel -- --nocapture
```

The kernel suite includes self-contained ABI, dispatch/discovery, and memory tests. Artifact-dependent tests return early when their driver is absent. A green suite without those artifacts does not establish end-to-end driver coverage. Follow [TEST.md](../TEST.md) and the host-specific instructions in [AGENTS.md](../AGENTS.md) for the full validation setup.
