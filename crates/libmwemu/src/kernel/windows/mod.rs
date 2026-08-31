//! The Windows kernel (ntoskrnl / WDM) surface for `.sys` drivers.
//!
//! The allocator family is implemented, because it shares the ledger with
//! Linux and therefore inherits the whole lifetime analysis for free: an
//! `ExFreePoolWithTag` followed by a stale dereference reports exactly like a
//! `kfree` does. Everything else is declared in [`SURFACE`] and answered with a
//! benign success so a driver keeps running through its own logic.
//!
//! What is still missing before a real `.sys` can be driven end to end is the
//! *loader*, not this file: a `.sys` is a PE image with a `DriverEntry`, so it
//! goes through the PE path with kernel-space placement rather than through the
//! ET_REL path used for `.ko`.

use crate::emu::Emu;
use crate::kernel::heap::Region;

/// Render a four-character pool tag the way the debugger does.
fn tag_name(tag: u64) -> String {
    let bytes = (tag as u32).to_le_bytes();
    let s: String = bytes
        .iter()
        .map(|b| {
            if b.is_ascii_graphic() {
                *b as char
            } else {
                '.'
            }
        })
        .collect();
    format!("pool-{}", s)
}

pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- pool allocator ---------------------------------------------------
        // ExAllocatePool2(Flags, NumberOfBytes, Tag) — the modern form, which
        // zeroes by default (POOL_FLAG_UNINITIALIZED clears that).
        "ExAllocatePool2" | "ExAllocatePool3" => {
            let flags = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let tag = emu.kernel_arg(2);
            const POOL_FLAG_UNINITIALIZED: u64 = 0x2;
            let ptr = emu.kernel_alloc(
                Region::Slab,
                size,
                &tag_name(tag),
                symbol,
                flags & POOL_FLAG_UNINITIALIZED == 0,
            );
            emu.set_kernel_ret(ptr);
        }
        // ExAllocatePoolWithTag(PoolType, NumberOfBytes, Tag) — legacy, no zeroing.
        "ExAllocatePoolWithTag"
        | "ExAllocatePoolWithTagPriority"
        | "ExAllocatePoolWithQuotaTag"
        | "ExAllocatePool" => {
            let size = emu.kernel_arg(1);
            let tag = emu.kernel_arg(2);
            let ptr = emu.kernel_alloc(Region::Slab, size, &tag_name(tag), symbol, false);
            emu.set_kernel_ret(ptr);
        }
        "ExFreePool" | "ExFreePoolWithTag" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }
        "MmAllocateNonCachedMemory"
        | "MmAllocateContiguousMemory"
        | "MmAllocateContiguousMemorySpecifyCache" => {
            let size = emu.kernel_arg(0);
            let ptr = emu.kernel_alloc(Region::Pages, size, "contiguous", symbol, false);
            emu.set_kernel_ret(ptr);
        }
        "MmFreeNonCachedMemory"
        | "MmFreeContiguousMemory"
        | "MmFreeContiguousMemorySpecifyCache" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }

        // --- memory helpers ---------------------------------------------------
        "RtlCopyMemory" | "memcpy" | "RtlMoveMemory" | "memmove" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, src, len as u32, false);
            emu.kernel_guard_access(rip, dst, len as u32, true);
            let mut buf = vec![0u8; len as usize];
            for (i, b) in buf.iter_mut().enumerate() {
                *b = emu.maps.read_byte(src + i as u64).unwrap_or(0);
            }
            emu.maps.write_bytes(dst, &buf);
            emu.set_kernel_ret(dst);
        }
        "RtlZeroMemory" | "RtlFillMemory" | "memset" => {
            let dst = emu.kernel_arg(0);
            let (len, byte) = if symbol == "memset" {
                (emu.kernel_arg(2), emu.kernel_arg(1) as u8)
            } else if symbol == "RtlFillMemory" {
                (emu.kernel_arg(1), emu.kernel_arg(2) as u8)
            } else {
                (emu.kernel_arg(1), 0u8)
            };
            let rip = emu.pc();
            emu.kernel_guard_access(rip, dst, len as u32, true);
            emu.maps.write_bytes(dst, &vec![byte; len as usize]);
            emu.set_kernel_ret(dst);
        }

        // --- logging ------------------------------------------------------------
        "DbgPrint" | "DbgPrintEx" | "vDbgPrintEx" => {
            let fmt = if symbol == "DbgPrint" { 0 } else { 2 };
            let fmt_addr = emu.kernel_arg(fmt);
            let line = crate::kernel::linux::printk::format(emu, fmt_addr, fmt + 1);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }

        // --- everything else --------------------------------------------------
        // Declared but not modelled: succeed quietly so the driver's own logic
        // keeps running, and let the caller see it in `unimplemented`.
        _ => return false,
    }
    true
}

/// The ntoskrnl surface a driver realistically imports, grouped by subsystem.
/// Entries outside the `alloc`/`string`/`logging` groups are declarations: they
/// name what still has to be implemented for full `.sys` support.
pub const SURFACE: &[(&str, &[&str])] = &[
    (
        "alloc",
        &[
            "ExAllocatePool",
            "ExAllocatePool2",
            "ExAllocatePool3",
            "ExAllocatePoolWithTag",
            "ExAllocatePoolWithTagPriority",
            "ExAllocatePoolWithQuotaTag",
            "ExFreePool",
            "ExFreePoolWithTag",
            "MmAllocateNonCachedMemory",
            "MmFreeNonCachedMemory",
            "MmAllocateContiguousMemory",
            "MmFreeContiguousMemory",
        ],
    ),
    (
        "string",
        &[
            "RtlCopyMemory",
            "RtlMoveMemory",
            "RtlZeroMemory",
            "RtlFillMemory",
            "memcpy",
            "memset",
        ],
    ),
    ("logging", &["DbgPrint", "DbgPrintEx", "vDbgPrintEx"]),
    (
        "mdl",
        &[
            "IoAllocateMdl",
            "IoFreeMdl",
            "MmProbeAndLockPages",
            "MmUnlockPages",
            "MmMapLockedPagesSpecifyCache",
            "MmUnmapLockedPages",
            "MmGetSystemAddressForMdlSafe",
        ],
    ),
    (
        "io",
        &[
            "IoCreateDevice",
            "IoDeleteDevice",
            "IoCreateSymbolicLink",
            "IoDeleteSymbolicLink",
            "IoCompleteRequest",
            "IofCompleteRequest",
            "IoGetCurrentIrpStackLocation",
            "IoAllocateIrp",
            "IoFreeIrp",
            "IoBuildDeviceIoControlRequest",
            "IoCallDriver",
            "IofCallDriver",
        ],
    ),
    (
        "objects",
        &[
            "ObReferenceObjectByHandle",
            "ObfReferenceObject",
            "ObfDereferenceObject",
            "ObReferenceObjectByPointer",
            "ZwClose",
            "ZwCreateFile",
            "ZwReadFile",
            "ZwWriteFile",
            "ZwQueryInformationFile",
        ],
    ),
    (
        "sync",
        &[
            "KeInitializeSpinLock",
            "KeAcquireSpinLock",
            "KeReleaseSpinLock",
            "KeAcquireSpinLockAtDpcLevel",
            "KeReleaseSpinLockFromDpcLevel",
            "ExAcquireFastMutex",
            "ExReleaseFastMutex",
            "KeWaitForSingleObject",
            "KeSetEvent",
            "KeInitializeEvent",
            "KeInitializeMutex",
        ],
    ),
    (
        "lifetime",
        &[
            "ExInterlockedInsertTailList",
            "ExInterlockedRemoveHeadList",
            "InterlockedIncrement",
            "InterlockedDecrement",
            "InterlockedExchange",
            "InterlockedCompareExchange",
        ],
    ),
];
