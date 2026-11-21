//! The Linux kernel allocator surface: slab, vmalloc, pages and the
//! user-copy helpers that move data in and out of them.
//!
//! Everything here funnels into [`Emu::kernel_alloc`] / [`Emu::kernel_free`],
//! so no matter which of the two dozen spellings of "allocate" a driver was
//! compiled against, the chunk lands in one ledger with one provenance record.
//! That single funnel is what makes the lifetime analysis possible.

use crate::emu::Emu;
use crate::kernel::KmemCache;
use crate::kernel::heap::{KernelHeap, Region};

/// `__GFP_ZERO` — set by `kzalloc`, `kcalloc` and friends, which are inline
/// wrappers in the kernel headers and therefore never appear as symbols.
const GFP_ZERO: u64 = 0x100;

/// Largest request we will honour, mirroring `KMALLOC_MAX_SIZE`. Bigger
/// requests fail with NULL exactly like the real allocator, which is often the
/// error path a driver forgets to handle.
const KMALLOC_MAX_SIZE: u64 = 0x400000;

fn slab_alloc(emu: &mut Emu, size: u64, flags: u64, api: &str) -> u64 {
    if size == 0 || size > KMALLOC_MAX_SIZE {
        log::warn!("{}: refusing {} byte allocation", api, size);
        emu.set_kernel_ret(0);
        return 0;
    }
    let cache = format!("kmalloc-{}", KernelHeap::bucket(size));
    let ptr = emu.kernel_alloc(Region::Slab, size, &cache, api, flags & GFP_ZERO != 0);
    emu.set_kernel_ret(ptr);
    ptr
}

fn large_alloc(emu: &mut Emu, size: u64, zeroed: bool, api: &str, region: Region) -> u64 {
    let ptr = emu.kernel_alloc(region, size, region.label(), api, zeroed);
    emu.set_kernel_ret(ptr);
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
    match symbol {
        // --- kmalloc family -------------------------------------------------
        // The kernel headers inline `kmalloc()` into one of several out-of-line
        // helpers depending on version and whether the size is a compile-time
        // constant, so all the spellings have to be accepted.
        "__kmalloc"
        | "__kmalloc_noprof"
        | "kmalloc"
        | "kmalloc_noprof"
        | "__kmalloc_large_noprof"
        | "__kmalloc_large_node_noprof" => {
            let size = emu.kernel_arg(0);
            let flags = emu.kernel_arg(1);
            slab_alloc(emu, size, flags, symbol);
        }
        "__kmalloc_node" | "__kmalloc_node_noprof" => {
            let size = emu.kernel_arg(0);
            let flags = emu.kernel_arg(1);
            slab_alloc(emu, size, flags, symbol);
        }
        // (cache, flags, size) — the constant-size path.
        "kmalloc_trace" | "__kmalloc_cache_noprof" | "kmalloc_trace_noprof" => {
            let flags = emu.kernel_arg(1);
            let size = emu.kernel_arg(2);
            slab_alloc(emu, size, flags, symbol);
        }
        "__kmalloc_cache_node_noprof" => {
            let flags = emu.kernel_arg(1);
            let size = emu.kernel_arg(3);
            slab_alloc(emu, size, flags, symbol);
        }
        "kcalloc" | "kcalloc_noprof" | "kmalloc_array" | "kmalloc_array_noprof" => {
            let n = emu.kernel_arg(0);
            let size = emu.kernel_arg(1);
            let flags = emu.kernel_arg(2) | GFP_ZERO;
            slab_alloc(emu, n.saturating_mul(size), flags, symbol);
        }
        "kfree" | "kfree_sensitive" | "kzfree" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }
        "krealloc"
        | "krealloc_noprof"
        | "__krealloc_noprof"
        | "krealloc_node_align_noprof"
        | "__krealloc_node_align_noprof" => {
            let old = emu.kernel_arg(0);
            let new_size = emu.kernel_arg(1);
            // krealloc(objp, size, flags) puts flags at arg2, but the newer
            // krealloc_node_align_noprof(objp, size, align, flags, nid) puts it
            // at arg3 (no kmalloc token unless CONFIG_KMALLOC_PARTITION_CACHES).
            let flags = if symbol.contains("node_align") {
                emu.kernel_arg(3)
            } else {
                emu.kernel_arg(2)
            };
            if new_size == 0 {
                emu.kernel_free(old, symbol);
                emu.set_kernel_ret(0);
                return true;
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
            emu.set_kernel_ret(new_ptr);
        }
        "kmemdup" | "kmemdup_noprof" | "kvmemdup" => {
            let src = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let flags = emu.kernel_arg(2);
            let ptr = slab_alloc(emu, len, flags, symbol);
            if ptr != 0 {
                guarded_copy(emu, ptr, src, len);
            }
            emu.set_kernel_ret(ptr);
        }
        "kstrdup" | "kstrdup_noprof" | "kstrdup_const" => {
            let src = emu.kernel_arg(0);
            let flags = emu.kernel_arg(1);
            let s = emu.maps.read_string(src);
            let ptr = slab_alloc(emu, s.len() as u64 + 1, flags, symbol);
            if ptr != 0 {
                emu.maps.write_string(ptr, &s);
            }
            emu.set_kernel_ret(ptr);
        }
        "kstrndup" | "kstrndup_noprof" => {
            let src = emu.kernel_arg(0);
            let max = emu.kernel_arg(1);
            let flags = emu.kernel_arg(2);
            let mut s = emu.maps.read_string(src);
            s.truncate(max as usize);
            let ptr = slab_alloc(emu, s.len() as u64 + 1, flags, symbol);
            if ptr != 0 {
                emu.maps.write_string(ptr, &s);
            }
            emu.set_kernel_ret(ptr);
        }

        // --- kmem_cache -----------------------------------------------------
        // A named cache is worth modelling properly: object lifetimes in real
        // drivers hang off them, and the cache name is what identifies the
        // object in a KASAN report.
        "kmem_cache_create"
        | "kmem_cache_create_usercopy"
        | "__kmem_cache_create_args"
        | "__kmem_cache_create" => {
            let name_ptr = emu.kernel_arg(0);
            let obj_size = emu.kernel_arg(1);
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
            emu.set_kernel_ret(handle);
        }
        "kmem_cache_destroy" => {
            let handle = emu.kernel_arg(0);
            if let Some(kernel) = emu.kernel.as_mut() {
                kernel.caches.remove(&handle);
            }
            emu.kernel_free(handle, symbol);
            emu.set_kernel_ret(0);
        }
        "kmem_cache_alloc"
        | "kmem_cache_alloc_noprof"
        | "kmem_cache_alloc_node"
        | "kmem_cache_alloc_node_noprof"
        | "kmem_cache_zalloc" => {
            let handle = emu.kernel_arg(0);
            let flags = emu.kernel_arg(1);
            let (name, size) = emu
                .kernel
                .as_ref()
                .and_then(|k| k.caches.get(&handle))
                .map(|c| (c.name.clone(), c.obj_size))
                .unwrap_or_else(|| ("kmem_cache-unknown".to_string(), 128));
            let zeroed = flags & GFP_ZERO != 0 || symbol == "kmem_cache_zalloc";
            let ptr = emu.kernel_alloc(Region::Slab, size, &name, symbol, zeroed);
            emu.set_kernel_ret(ptr);
        }
        "kmem_cache_free" => {
            let ptr = emu.kernel_arg(1);
            emu.kernel_free(ptr, symbol);
        }

        // --- vmalloc / kvmalloc ---------------------------------------------
        "vmalloc"
        | "vmalloc_noprof"
        | "__vmalloc"
        | "__vmalloc_noprof"
        | "vmalloc_user"
        | "vmalloc_node"
        | "vmalloc_node_noprof" => {
            let size = emu.kernel_arg(0);
            large_alloc(emu, size, false, symbol, Region::Vmalloc);
        }
        "vzalloc" | "vzalloc_noprof" | "vcalloc" | "vcalloc_noprof" => {
            let size = emu.kernel_arg(0);
            large_alloc(emu, size, true, symbol, Region::Vmalloc);
        }
        "vfree" | "vfree_atomic" | "kvfree" | "kvfree_sensitive" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }
        "kvmalloc" | "kvmalloc_node" | "kvmalloc_node_noprof" | "kvzalloc" => {
            let size = emu.kernel_arg(0);
            let flags = emu.kernel_arg(1);
            let zeroed = flags & GFP_ZERO != 0 || symbol == "kvzalloc";
            // kvmalloc tries the slab first and only falls back to vmalloc for
            // big requests — same threshold as the kernel (PAGE_SIZE * 2).
            if size <= 0x2000 {
                slab_alloc(emu, size, flags, symbol);
            } else {
                large_alloc(emu, size, zeroed, symbol, Region::Vmalloc);
            }
        }

        // --- raw pages ------------------------------------------------------
        "__get_free_pages" | "get_free_pages" | "alloc_pages" | "alloc_pages_noprof" => {
            let order = emu.kernel_arg(1);
            large_alloc(
                emu,
                0x1000u64 << order.min(20),
                false,
                symbol,
                Region::Pages,
            );
        }
        "get_zeroed_page" | "get_zeroed_page_noprof" => {
            large_alloc(emu, 0x1000, true, symbol, Region::Pages);
        }
        "free_pages" | "__free_pages" | "free_page" | "__free_page" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }

        // --- devm_* (device-managed) ----------------------------------------
        "devm_kmalloc" | "devm_kzalloc" | "devm_kmalloc_noprof" => {
            let size = emu.kernel_arg(1);
            let flags = emu.kernel_arg(2);
            let zeroed = flags & GFP_ZERO != 0 || symbol == "devm_kzalloc";
            let ptr = emu.kernel_alloc(Region::Slab, size, "devm", symbol, zeroed);
            emu.set_kernel_ret(ptr);
        }
        "devm_kfree" => {
            let ptr = emu.kernel_arg(1);
            emu.kernel_free(ptr, symbol);
        }

        // --- user copies ----------------------------------------------------
        // There is no user address space here: the caller sets up a buffer and
        // passes its address, so a copy is an ordinary guarded memcpy. The
        // return value follows the kernel convention: bytes *not* copied.
        "copy_from_user" | "_copy_from_user" | "__copy_from_user" | "_copy_from_user_nocheck" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let ok = guarded_copy(emu, dst, src, len);
            emu.set_kernel_ret(if ok { 0 } else { len });
        }
        "copy_to_user" | "_copy_to_user" | "__copy_to_user" | "_copy_to_user_nocheck" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let len = emu.kernel_arg(2);
            let ok = guarded_copy(emu, dst, src, len);
            emu.set_kernel_ret(if ok { 0 } else { len });
        }
        "clear_user" | "__clear_user" => {
            let dst = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let rip = emu.pc();
            emu.kernel_guard_access(rip, dst, len as u32, true);
            emu.maps.write_bytes(dst, &vec![0u8; len as usize]);
            emu.set_kernel_ret(0);
        }
        "strncpy_from_user" => {
            let dst = emu.kernel_arg(0);
            let src = emu.kernel_arg(1);
            let max = emu.kernel_arg(2);
            let mut s = emu.maps.read_string(src);
            s.truncate(max.saturating_sub(1) as usize);
            emu.maps.write_string(dst, &s);
            emu.set_kernel_ret(s.len() as u64);
        }
        "memdup_user" | "vmemdup_user" | "memdup_user_nul" => {
            let src = emu.kernel_arg(0);
            let len = emu.kernel_arg(1);
            let ptr = slab_alloc(emu, len.max(1), 0, symbol);
            if ptr != 0 {
                guarded_copy(emu, ptr, src, len);
            }
            emu.set_kernel_ret(ptr);
        }

        _ => return false,
    }
    true
}
