//! The XNU / IOKit surface for `.kext` bundles.
//!
//! Same shape as the Windows side: the allocators are implemented so a kext
//! shares the lifetime ledger, and the rest of the surface is declared in
//! [`SURFACE`] as the list of what still has to be built.
//!
//! The missing piece for end-to-end kext support is again the loader — a kext
//! executable is a Mach-O `MH_KEXT_BUNDLE` with external relocations, so it
//! needs the Mach-O counterpart of the ET_REL path used for `.ko`.

use crate::emu::Emu;
use crate::kernel::heap::Region;

pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- allocators ---------------------------------------------------------
        "IOMalloc" | "IOMallocAligned" | "kalloc" | "kalloc_external" => {
            let size = emu.kernel_arg(0);
            let ptr = emu.kernel_alloc(Region::Slab, size, "kalloc", symbol, false);
            emu.set_kernel_ret(ptr);
        }
        "IOMallocZero" | "IOMallocZeroData" | "kalloc_type_impl_external" => {
            let size = emu.kernel_arg(0);
            let ptr = emu.kernel_alloc(Region::Slab, size, "kalloc", symbol, true);
            emu.set_kernel_ret(ptr);
        }
        "IOFree"
        | "IOFreeAligned"
        | "IOFreeData"
        | "kfree"
        | "kfree_external"
        | "kfree_type_impl_external" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }

        // --- logging ---------------------------------------------------------------
        "IOLog" | "printf" | "kprintf" | "os_log_internal" => {
            let fmt = emu.kernel_arg(0);
            let line = crate::kernel::linux::printk::format(emu, fmt, 1);
            emu.kernel_log_line(line);
            emu.set_kernel_ret(0);
        }

        // --- locking (no-ops: emulation is single-threaded) --------------------
        // IOLockAlloc hands back a non-null, mapped token the driver stores and
        // passes back to lock/unlock; it is never dereferenced in this model, so
        // reusing the kernel data base avoids a tracked allocation (which would
        // otherwise look like a leak).
        "IOLockAlloc" | "IORWLockAlloc" | "lck_mtx_alloc_init" | "IOSimpleLockAlloc" => {
            let token = emu.kernel.as_ref().map(|k| k.layout.data_base).unwrap_or(0);
            emu.set_kernel_ret(token);
        }
        "IOLockLock" | "IOLockUnlock" | "IOLockFree" | "IORWLockRead" | "IORWLockWrite"
        | "IORWLockUnlock" | "IORWLockFree" | "lck_mtx_lock" | "lck_mtx_unlock"
        | "lck_mtx_free" | "IOSimpleLockLock" | "IOSimpleLockUnlock" | "IOSimpleLockFree" => {
            emu.set_kernel_ret(0);
        }

        // Generic libkern string/memory helpers (memcpy, memset, memcmp,
        // strlen, …) behave exactly as on Linux and run through the same guard,
        // so a copy into a freed object is caught inside the kernel call.
        _ => return crate::kernel::linux::string::dispatch(symbol, emu),
    }
    true
}

/// The XNU surface a kext realistically imports. Groups beyond `alloc` and
/// `logging` are declarations of what still has to be implemented.
pub const SURFACE: &[(&str, &[&str])] = &[
    (
        "alloc",
        &[
            "IOMalloc",
            "IOMallocZero",
            "IOMallocAligned",
            "IOMallocData",
            "IOFree",
            "IOFreeAligned",
            "IOFreeData",
            "kalloc_external",
            "kfree_external",
            "kalloc_type_impl_external",
            "kfree_type_impl_external",
        ],
    ),
    ("logging", &["IOLog", "kprintf", "os_log_internal"]),
    (
        "objects",
        &[
            "OSObject::retain",
            "OSObject::release",
            "OSObject::taggedRetain",
            "OSMetaClass::allocClassWithName",
            "OSDynamicCast",
            "OSSafeReleaseNULL",
        ],
    ),
    (
        "sync",
        &[
            "lck_mtx_alloc_init",
            "lck_mtx_lock",
            "lck_mtx_unlock",
            "lck_mtx_free",
            "lck_spin_lock",
            "lck_spin_unlock",
            "IOLockAlloc",
            "IOLockLock",
            "IOLockUnlock",
            "IOLockFree",
        ],
    ),
    (
        "iokit",
        &[
            "IOService::start",
            "IOService::stop",
            "IOMemoryDescriptor::withAddress",
            "IOBufferMemoryDescriptor::inTaskWithOptions",
            "IOUserClient::externalMethod",
            "copyin",
            "copyout",
        ],
    ),
];
