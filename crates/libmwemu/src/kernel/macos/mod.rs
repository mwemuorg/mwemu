//! The XNU / IOKit surface for `.kext` bundles.
//!
//! Same shape as the Windows side: the allocators are implemented so a kext
//! shares the lifetime ledger, and the rest of the surface is declared in
//! [`SURFACE`] as the list of what still has to be built.
//!
//! The missing piece for end-to-end kext support is again the loader — a kext
//! executable is a Mach-O `MH_KEXT_BUNDLE` with external relocations, so it
//! needs the Mach-O counterpart of the ET_REL path used for `.ko`.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::heap::Region;
use crate::kernel::linux::{printk, string};

pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, {
        // --- allocators ---------------------------------------------------------
        "IOMalloc" | "IOMallocAligned" | "kalloc" | "kalloc_external" => IOMalloc(emu, symbol),
        "IOMallocZero" | "IOMallocZeroData" | "kalloc_type_impl_external" => IOMallocZero(emu, symbol),
        "IOFree"
        | "IOFreeAligned"
        | "IOFreeData"
        | "kfree"
        | "kfree_external"
        | "kfree_type_impl_external" => IOFree(emu, symbol),

        // --- logging ---------------------------------------------------------------
        "IOLog" | "printf" | "kprintf" | "os_log_internal" => IOLog(emu, symbol),

        // --- locking (no-ops: emulation is single-threaded) --------------------
        // IOLockAlloc hands back a non-null, mapped token the driver stores and
        // passes back to lock/unlock; it is never dereferenced in this model, so
        // reusing the kernel data base avoids a tracked allocation (which would
        // otherwise look like a leak).
        "IOLockAlloc" | "IORWLockAlloc" | "lck_mtx_alloc_init" | "IOSimpleLockAlloc" => lock_token_stub(emu, symbol),
        "IOLockLock" | "IOLockUnlock" | "IOLockFree" | "IORWLockRead" | "IORWLockWrite"
        | "IORWLockUnlock" | "IORWLockFree" | "lck_mtx_lock" | "lck_mtx_unlock"
        | "lck_mtx_free" | "IOSimpleLockLock" | "IOSimpleLockUnlock" | "IOSimpleLockFree" => lock_stub(emu, symbol),
    },
        handled(()) => true,
        fallback => string::dispatch(symbol, emu),
    )
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

fn IOMalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    let ptr = emu.kernel_alloc(Region::Slab, size, "kalloc", symbol, false);
    call.set_return(emu, ptr);
}

fn IOMallocZero(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    let ptr = emu.kernel_alloc(Region::Slab, size, "kalloc", symbol, true);
    call.set_return(emu, ptr);
}

fn IOFree(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(ptr, symbol);
}

fn IOLog(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, printk::format(emu, fmt, 1));
    emu.kernel_log_line(line);
    call.set_return(emu, 0);
}

fn lock_token_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let token = emu.kernel.as_ref().map(|k| k.layout.data_base).unwrap_or(0);
    call.set_return(emu, token);
}

fn lock_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    call.set_return(emu, 0);
}
