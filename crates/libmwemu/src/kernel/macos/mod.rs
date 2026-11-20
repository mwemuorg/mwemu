//! XNU and IOKit imports for emulated kexts.

use crate::api::abi::unix::UnixApi;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::heap::{MIN_ALLOCATION_ALIGNMENT, Region};
use crate::kernel::linux::{printk, string};
use crate::kernel::{Handler, HandlerRegistry, ignore_call, kernel_handlers};

const ALLOCATIONS: HandlerRegistry = kernel_handlers! {
    "IOMalloc" | "IOMallocData" | "kalloc" | "kalloc_external" => IOMalloc,
    "IOMallocZero" | "IOMallocZeroData" => IOMallocZero,
    "IOMallocAligned" => IOMallocAligned,
    "IOFree" | "IOFreeAligned" | "IOFreeData" | "kfree" | "kfree_external" => IOFree,
};

const LOGGING: HandlerRegistry = kernel_handlers! {
    "IOLog" | "kprintf" | "printf" => IOLog,
};

const LOCKS: HandlerRegistry = kernel_handlers! {
    "IOLockAlloc" | "IORWLockAlloc" | "lck_mtx_alloc_init" | "IOSimpleLockAlloc" => lock_token_stub,
    "IOLockLock" | "IOLockUnlock" | "IOLockFree" | "IORWLockRead" | "IORWLockWrite"
    | "IORWLockUnlock" | "IORWLockFree" | "lck_mtx_lock" | "lck_mtx_unlock"
    | "lck_mtx_free" | "IOSimpleLockLock" | "IOSimpleLockUnlock" | "IOSimpleLockFree" => ignore_call,
};

const SHARED_STRINGS: &[&str] = &[
    "memcpy",
    "memmove",
    "memset",
    "memcmp",
    "bcmp",
    "strlen",
    "strnlen",
    "strcmp",
    "strncmp",
    "strcasecmp",
    "strncasecmp",
    "strcpy",
    "strncpy",
    "strlcpy",
    "strcat",
    "strncat",
    "strchr",
    "strrchr",
    "strstr",
    "snprintf",
    "sprintf",
];

pub const SURFACE: &[(&str, &[&str])] = &[
    ("alloc", ALLOCATIONS.names),
    ("logging", LOGGING.names),
    ("sync", LOCKS.names),
    ("string", SHARED_STRINGS),
];

fn handler(symbol: &str) -> Option<Handler> {
    for registry in [ALLOCATIONS, LOGGING, LOCKS] {
        if let Some(handler) = registry.find(symbol) {
            return Some(handler);
        }
    }
    if SHARED_STRINGS.contains(&symbol) {
        return string::handlers().find(symbol);
    }
    None
}

pub fn is_implemented(symbol: &str) -> bool {
    handler(symbol).is_some()
}

pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    let Some(handler) = handler(symbol) else {
        return false;
    };
    handler(emu, symbol);
    true
}

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

fn IOMallocAligned(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let alignment = api_try!(emu, call.arg(emu, 1));
    if alignment.is_power_of_two() && alignment <= MIN_ALLOCATION_ALIGNMENT {
        IOMalloc(emu, symbol);
    } else {
        call.set_return(emu, 0);
    }
}

fn IOFree(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(ptr, symbol);
}

fn IOLog(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let fmt = api_try!(emu, call.arg(emu, 0));
    let line = api_try!(emu, printk::format(emu, fmt, 1));
    emu.kernel_log_line(line);
    if symbol == "printf" {
        call.set_return(emu, 0);
    }
}

fn lock_token_stub(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let token = emu.kernel.as_ref().map(|k| k.layout.data_base).unwrap_or(0);
    call.set_return(emu, token);
}
