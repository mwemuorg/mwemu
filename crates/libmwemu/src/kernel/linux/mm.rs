//! The Linux kernel allocator surface: slab, vmalloc, pages and the
//! user-copy helpers that move data in and out of them.
//!
//! Everything here funnels into [`Emu::kernel_alloc`] / [`Emu::kernel_free`],
//! so no matter which of the two dozen spellings of "allocate" a driver was
//! compiled against, the chunk lands in one ledger with one provenance record.
//! That single funnel is what makes the lifetime analysis possible.

use crate::api::abi::unix::UnixApi;
use crate::api::dispatch_api;
use crate::api::unix::api_try;
use crate::emu::Emu;
use crate::kernel::KmemCache;
use crate::kernel::heap::{KernelHeap, Region};
use crate::kernel::{HandlerRegistry, kernel_handlers};

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

/// Copy `len` bytes between two guest addresses through the emulator's memory,
/// reporting any access that lands in a freed or out-of-bounds chunk.
fn guarded_copy(emu: &mut Emu, dst: u64, src: u64, len: u64) -> bool {
    let rip = emu.pc();
    // Range-aware: catches an overflow whose tail shoots past the bucket.
    emu.kernel_guard_range(rip, src, len, false);
    emu.kernel_guard_range(rip, dst, len, true);
    emu.kernel_guard_access(rip, src, len as u32, false);
    emu.kernel_guard_access(rip, dst, len as u32, true);

    let mut buf = vec![0u8; len as usize];
    for (i, b) in buf.iter_mut().enumerate() {
        match emu.maps.read_byte(src + i as u64) {
            Some(v) => *b = v,
            None => return false,
        }
    }
    emu.maps.write_bytes(dst, &buf)
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
        "kstrndup" | "kstrndup_noprof" => kstrndup,

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
        "copy_from_user" | "_copy_from_user" | "__copy_from_user" | "_copy_from_user_nocheck" => copy_from_user,
        "copy_to_user" | "_copy_to_user" | "__copy_to_user" | "_copy_to_user_nocheck" => copy_from_user,
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
        guarded_copy(emu, new_ptr, old, copy_len);
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
    if ptr != 0 {
        guarded_copy(emu, ptr, src, len);
    }
    call.set_return(emu, ptr);
}

fn kstrdup(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let src = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(src);
    let ptr = slab_alloc(emu, s.len() as u64 + 1, flags, symbol);
    if ptr != 0 {
        emu.maps.write_string(ptr, &s);
    }
    call.set_return(emu, ptr);
}

fn kstrndup(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let src = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let mut s = emu.maps.read_string(src);
    s.truncate(max as usize);
    let ptr = slab_alloc(emu, s.len() as u64 + 1, flags, symbol);
    if ptr != 0 {
        emu.maps.write_string(ptr, &s);
    }
    call.set_return(emu, ptr);
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

fn copy_from_user(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let ok = guarded_copy(emu, dst, src, len);
    call.set_return(emu, if ok { 0 } else { len });
}

fn clear_user(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let rip = emu.pc();
    emu.kernel_guard_access(rip, dst, len as u32, true);
    emu.maps.write_bytes(dst, &vec![0u8; len as usize]);
    call.set_return(emu, 0);
}

fn strncpy_from_user(emu: &mut Emu, _symbol: &str) {
    let call = UnixApi;
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let max = api_try!(emu, call.arg(emu, 2));
    let mut s = emu.maps.read_string(src);
    s.truncate(max.saturating_sub(1) as usize);
    emu.maps.write_string(dst, &s);
    call.set_return(emu, s.len() as u64);
}

fn memdup_user(emu: &mut Emu, symbol: &str) {
    let call = UnixApi;
    let src = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg(emu, 1));
    let ptr = slab_alloc(emu, len.max(1), 0, symbol);
    if ptr != 0 {
        guarded_copy(emu, ptr, src, len);
    }
    call.set_return(emu, ptr);
}
