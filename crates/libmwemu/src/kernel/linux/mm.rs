//! The Linux kernel allocator surface: slab, vmalloc, pages and the
//! user-copy helpers that move data in and out of them.
//!
//! Everything here funnels into [`Emu::kernel_alloc`] / [`Emu::kernel_free`],
//! so no matter which of the two dozen spellings of "allocate" a driver was
//! compiled against, the chunk lands in one ledger with one provenance record.
//! That single funnel is what makes the lifetime analysis possible.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::{MemoryError, api_try, memory};
use crate::emu::Emu;
use crate::kernel::KmemCache;
use crate::kernel::heap::{KernelHeap, Region};
use crate::kernel::{HandlerRegistry, kernel_handlers};
use crate::maps::mem64::Permission;
use crate::syscall::linux::constants::errors::{EFAULT, ENOMEM};

/// `__GFP_ZERO` — set by `kzalloc`, `kcalloc` and friends, which are inline
/// wrappers in the kernel headers and therefore never appear as symbols.
const GFP_ZERO: u64 = 0x100;

/// Largest request we will honour, mirroring `KMALLOC_MAX_SIZE`. Bigger
/// requests fail with NULL exactly like the real allocator, which is often the
/// error path a driver forgets to handle.
const KMALLOC_MAX_SIZE: u64 = 0x400000;

fn slab_alloc(emu: &mut Emu, size: u64, flags: u64, api: &str) -> u64 {
    let call = UnixApi;
    if size == 0 || size > KMALLOC_MAX_SIZE {
        log::warn!("{}: refusing {} byte allocation", api, size);
        call.set_return(emu, 0);
        return 0;
    }
    let cache = format!("kmalloc-{}", KernelHeap::bucket(size));
    let ptr = emu.kernel_alloc(Region::Slab, size, &cache, api, flags & GFP_ZERO != 0);
    call.set_return(emu, ptr);
    ptr
}

fn large_alloc(emu: &mut Emu, size: u64, zeroed: bool, api: &str, region: Region) -> u64 {
    let call = UnixApi;
    let ptr = emu.kernel_alloc(region, size, region.label(), api, zeroed);
    call.set_return(emu, ptr);
    ptr
}

pub(super) fn guard_range(emu: &mut Emu, address: u64, size: u64, write: bool) {
    if size != 0 {
        let pc = emu.pc();
        emu.kernel_guard_range(pc, address, size, write);
        emu.kernel_guard_access(pc, address, size.min(u32::MAX as u64) as u32, write);
    }
}

/// Copy `len` bytes between two guest addresses through the emulator's memory,
/// reporting any access that lands in a freed or out-of-bounds chunk.
pub(super) fn guarded_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) -> Result<(), MemoryError> {
    guard_range(emu, src, len, false);
    guard_range(emu, dst, len, true);
    memory::copy(emu, dst, src, len)
}

fn accessible(emu: &Emu, addr: u64, len: u64, access: Permission) -> u64 {
    let Ok(size) = usize::try_from(len) else {
        return 0;
    };
    emu.maps
        .first_inaccessible_address(addr, size, access)
        .map_or(len, |first| first.saturating_sub(addr))
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- kmalloc family -------------------------------------------------
        // The kernel headers inline `kmalloc()` into one of several out-of-line
        // helpers depending on version and whether the size is a compile-time
        // constant, so all the spellings have to be accepted.
        "__kmalloc"
        | "__kmalloc_noprof"
        | "kmalloc"
        | "kmalloc_noprof"
        | "__kmalloc_large_noprof"
        | "__kmalloc_large_node_noprof" => __kmalloc,
        "__kmalloc_node" | "__kmalloc_node_noprof" => __kmalloc,
        // (cache, flags, size) — the constant-size path.
        "kmalloc_trace" | "__kmalloc_cache_noprof" | "kmalloc_trace_noprof" => kmalloc_trace,
        "__kmalloc_cache_node_noprof" => __kmalloc_cache_node_noprof,
        "kcalloc" | "kcalloc_noprof" | "kmalloc_array" | "kmalloc_array_noprof" => kcalloc,
        "kfree" | "kfree_sensitive" | "kzfree" => free_allocation,
        "krealloc"
        | "krealloc_noprof"
        | "__krealloc_noprof"
        | "krealloc_node_align_noprof"
        | "__krealloc_node_align_noprof" => krealloc,
        "kmemdup" | "kmemdup_noprof" | "kvmemdup" => kmemdup,
        "kstrdup" | "kstrdup_noprof" | "kstrdup_const" => kstrdup,
        "kstrndup" | "kstrndup_noprof" => kstrdup,

        // --- kmem_cache -----------------------------------------------------
        // A named cache is worth modelling properly: object lifetimes in real
        // drivers hang off them, and the cache name is what identifies the
        // object in a KASAN report.
        "kmem_cache_create"
        | "kmem_cache_create_usercopy"
        | "__kmem_cache_create_args"
        | "__kmem_cache_create" => kmem_cache_create,
        "kmem_cache_destroy" => kmem_cache_destroy,
        "kmem_cache_alloc"
        | "kmem_cache_alloc_noprof"
        | "kmem_cache_alloc_node"
        | "kmem_cache_alloc_node_noprof"
        | "kmem_cache_zalloc" => kmem_cache_alloc,
        "kmem_cache_free" => kmem_cache_free,

        // --- vmalloc / kvmalloc ---------------------------------------------
        "vmalloc"
        | "vmalloc_noprof"
        | "__vmalloc"
        | "__vmalloc_noprof"
        | "vmalloc_user"
        | "vmalloc_node"
        | "vmalloc_node_noprof" => vmalloc,
        "vzalloc" | "vzalloc_noprof" | "vcalloc" | "vcalloc_noprof" => vzalloc,
        "vfree" | "vfree_atomic" | "kvfree" | "kvfree_sensitive" => free_allocation,
        "kvmalloc" | "kvmalloc_node" | "kvmalloc_node_noprof" | "kvzalloc" => kvmalloc,

        // --- raw pages ------------------------------------------------------
        "__get_free_pages" | "get_free_pages" | "alloc_pages" | "alloc_pages_noprof" => __get_free_pages,
        "get_zeroed_page" | "get_zeroed_page_noprof" => get_zeroed_page,
        "free_pages" | "__free_pages" | "free_page" | "__free_page" => free_allocation,

        // --- devm_* (device-managed) ----------------------------------------
        "devm_kmalloc" | "devm_kzalloc" | "devm_kmalloc_noprof" => devm_kmalloc,
        "devm_kfree" => kmem_cache_free,

        // --- user copies ----------------------------------------------------
        // There is no user address space here: the caller sets up a buffer and
        // passes its address, so a copy is an ordinary guarded memcpy. The
        // return value follows the kernel convention: bytes *not* copied.
        "copy_from_user" | "_copy_from_user" | "__copy_from_user" | "_copy_from_user_nocheck" => copy_user,
        "copy_to_user" | "_copy_to_user" | "__copy_to_user" | "_copy_to_user_nocheck" => copy_user,
        "clear_user" | "__clear_user" => clear_user,
        "strncpy_from_user" => strncpy_from_user,
        "memdup_user" | "vmemdup_user" | "memdup_user_nul" => memdup_user,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn __kmalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    slab_alloc(emu, size, flags, symbol);
}

fn kmalloc_trace(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let flags = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    slab_alloc(emu, size, flags, symbol);
}

fn __kmalloc_cache_node_noprof(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let flags = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 3));
    slab_alloc(emu, size, flags, symbol);
}

fn kcalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let n = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2)) | GFP_ZERO;
    slab_alloc(emu, n.saturating_mul(size), flags, symbol);
}

fn free_allocation(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(ptr, symbol);
}

fn krealloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let old = api_try!(emu, call.arg(emu, 0));
    let new_size = api_try!(emu, call.arg(emu, 1));
    // krealloc(objp, size, flags) puts flags at arg2, but the newer
    // krealloc_node_align_noprof(objp, size, align, flags, nid) puts it
    // at arg3 (no kmalloc token unless CONFIG_KMALLOC_PARTITION_CACHES).
    let flags = if symbol.contains("node_align") {
        api_try!(emu, call.arg(emu, 3))
    } else {
        api_try!(emu, call.arg(emu, 2))
    };
    if new_size == 0 {
        emu.kernel_free(old, symbol);
        call.set_return(emu, 0);
        return;
    }
    let copy_len = emu
        .kernel
        .as_ref()
        .and_then(|k| k.heap.chunk_at(old).map(|c| c.req_size.min(new_size)))
        .unwrap_or(0);
    let new_ptr = slab_alloc(emu, new_size, flags, symbol);
    if new_ptr != 0 && old != 0 {
        if guarded_copy(emu, new_ptr, old, copy_len).is_err() {
            emu.kernel_free(new_ptr, symbol);
            call.set_return(emu, 0);
            return;
        }
        emu.kernel_free(old, symbol);
    }
    call.set_return(emu, new_ptr);
}

fn kmemdup(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let src = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let ptr = slab_alloc(emu, len, flags, symbol);
    if ptr != 0 && guarded_copy(emu, ptr, src, len).is_err() {
        emu.kernel_free(ptr, symbol);
        call.set_return(emu, 0);
        return;
    }
    call.set_return(emu, ptr);
}

fn kstrdup(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let bounded = symbol.starts_with("kstrndup");
    let limit = if bounded {
        api_try!(emu, call.arg(emu, 1))
    } else {
        u64::MAX
    };
    let flags = api_try!(emu, call.arg(emu, 1 + usize::from(bounded)));
    if source == 0 {
        call.set_return(emu, 0);
        return;
    }
    let size = api_try!(emu, memory::length(emu, source, limit));
    guard_range(emu, source, size + u64::from(size < limit), false);
    let destination = slab_alloc(emu, size + 1, flags, symbol);
    if destination != 0 {
        guard_range(emu, destination, size + 1, true);
        if memory::terminated_copy(emu, destination, source, size).is_err() {
            emu.kernel_free(destination, symbol);
            call.set_return(emu, 0);
            return;
        }
    }
    call.set_return(emu, destination);
}

fn kmem_cache_create(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let obj_size = api_try!(emu, call.arg(emu, 1));
    let name = emu.maps.read_string(name_ptr);
    let handle = emu.kernel_alloc(Region::Slab, 0x100, "kmem_cache", symbol, true);
    if let Some(kernel) = emu.kernel.as_mut().filter(|_| handle != 0) {
        kernel.caches.insert(
            handle,
            KmemCache {
                handle,
                name: name.clone(),
                obj_size,
            },
        );
    }
    log::info!(
        "kmem_cache_create(\"{}\", size={}) -> 0x{:x}",
        name,
        obj_size,
        handle
    );
    call.set_return(emu, handle);
}

fn kmem_cache_destroy(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let handle = api_try!(emu, call.arg(emu, 0));
    if let Some(kernel) = emu.kernel.as_mut() {
        kernel.caches.remove(&handle);
    }
    emu.kernel_free(handle, symbol);
    call.set_return(emu, 0);
}

fn kmem_cache_alloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let handle = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let (name, size) = emu
        .kernel
        .as_ref()
        .and_then(|k| k.caches.get(&handle))
        .map(|c| (c.name.clone(), c.obj_size))
        .unwrap_or_else(|| ("kmem_cache-unknown".to_string(), 128));
    let zeroed = flags & GFP_ZERO != 0 || symbol == "kmem_cache_zalloc";
    let ptr = emu.kernel_alloc(Region::Slab, size, &name, symbol, zeroed);
    call.set_return(emu, ptr);
}

fn kmem_cache_free(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let ptr = api_try!(emu, call.arg(emu, 1));
    emu.kernel_free(ptr, symbol);
}

fn vmalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    large_alloc(emu, size, false, symbol, Region::Vmalloc);
}

fn vzalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    large_alloc(emu, size, true, symbol, Region::Vmalloc);
}

fn kvmalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let zeroed = flags & GFP_ZERO != 0 || symbol == "kvzalloc";
    // kvmalloc tries the slab first and only falls back to vmalloc for
    // big requests — same threshold as the kernel (PAGE_SIZE * 2).
    if size <= 0x2000 {
        slab_alloc(emu, size, flags, symbol);
    } else {
        large_alloc(emu, size, zeroed, symbol, Region::Vmalloc);
    }
}

fn __get_free_pages(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let order = api_try!(emu, call.arg(emu, 1));
    large_alloc(
        emu,
        0x1000u64 << order.min(20),
        false,
        symbol,
        Region::Pages,
    );
}

fn get_zeroed_page(emu: &mut Emu, symbol: &str) {
    large_alloc(emu, 0x1000, true, symbol, Region::Pages);
}

fn devm_kmalloc(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let size = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let zeroed = flags & GFP_ZERO != 0 || symbol == "devm_kzalloc";
    let ptr = emu.kernel_alloc(Region::Slab, size, "devm", symbol, zeroed);
    call.set_return(emu, ptr);
}

fn copy_user(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    guard_range(emu, src, len, false);
    guard_range(emu, dst, len, true);
    let writable = accessible(emu, dst, len, Permission::WRITE);
    let copied = writable.min(accessible(emu, src, len, Permission::READ));
    if memory::copy(emu, dst, src, copied).is_err() {
        call.set_return(emu, len);
        return;
    }
    if copied < writable && matches!(symbol, "copy_from_user" | "_copy_from_user") {
        api_try!(emu, memory::fill(emu, dst + copied, 0, writable - copied));
    }
    call.set_return(emu, len - copied);
}

fn clear_user(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    guard_range(emu, dst, len, true);
    let prefix = accessible(emu, dst, len, Permission::WRITE);
    let cleared = if memory::fill(emu, dst, 0, prefix).is_ok() {
        prefix
    } else {
        0
    };
    call.set_return(emu, len - cleared);
}

fn strncpy_from_user(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let limit = api_try!(emu, call.arg_i64(emu, 2)).max(0) as u64;
    let prefix = accessible(emu, src, limit, Permission::READ);
    let len = api_try!(emu, memory::length(emu, src, prefix));
    let count = len + u64::from(len < prefix);
    let result = if guarded_copy(emu, dst, src, count).is_err() || (len == prefix && prefix < limit)
    {
        EFAULT
    } else {
        len
    };
    call.set_return(emu, result);
}

fn memdup_user(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let src = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let nul = symbol == "memdup_user_nul";
    let size = len.saturating_add(u64::from(nul)).max(1);
    let ptr = slab_alloc(emu, size, 0, symbol);
    let result = if ptr == 0 {
        ENOMEM
    } else if guarded_copy(emu, ptr, src, len).is_err() {
        emu.kernel_free(ptr, symbol);
        EFAULT
    } else {
        if nul && !emu.maps.write_byte(ptr + len, 0) {
            emu.kernel_free(ptr, symbol);
            call.set_return(emu, EFAULT);
            return;
        }
        ptr
    };
    call.set_return(emu, result);
}
