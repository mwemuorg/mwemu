//! Pool allocators and the small lock-free containers built on top of them.
//!
//! Every allocation spelling funnels into the shared ledger, so a stale
//! pointer into an `ExFreePoolWithTag`'d chunk reports as a use-after-free
//! with the pool tag, the allocation site and the free site attached.
//!
//! Lookaside lists are modelled as what they are on a real kernel: a header
//! whose `Allocate`/`Free` members point at the pool routines. The DDK's
//! `ExAllocateFromNPagedLookasideList` is inline and calls through those
//! members when the SList is empty, so pointing them at interceptable stubs is
//! what makes the inline code work unmodified.

use super::structs::*;
use super::tag_name;
use crate::emu::Emu;
use crate::kernel::heap::Region;

/// x64 `SLIST_HEADER` (16 bytes): `Depth:16 | Sequence:48` then
/// `HeaderType:1 | Reserved:3 | NextEntry:60` where the entry address is
/// `NextEntry << 4`. Drivers built with modern compilers manipulate this
/// inline with `cmpxchg16b`, so the imported helpers must keep the same layout.
fn slist_first(emu: &Emu, head: u64) -> u64 {
    let hi = emu.maps.read_qword(head + 8).unwrap_or(0);
    (hi >> 4) << 4
}

fn slist_depth(emu: &Emu, head: u64) -> u16 {
    emu.maps.read_word(head).unwrap_or(0)
}

fn slist_set(emu: &mut Emu, head: u64, first: u64, depth: u16) {
    let lo = emu.maps.read_qword(head).unwrap_or(0);
    let seq = (lo >> 16).wrapping_add(1);
    emu.maps.write_qword(head, (seq << 16) | depth as u64);
    // HeaderType = 1 marks the 16-byte form.
    emu.maps.write_qword(head + 8, (first & !0xf) | 1);
}

pub fn slist_push(emu: &mut Emu, head: u64, entry: u64) -> u64 {
    let first = slist_first(emu, head);
    let depth = slist_depth(emu, head);
    emu.maps.write_qword(entry, first);
    slist_set(emu, head, entry, depth.wrapping_add(1));
    first
}

pub fn slist_pop(emu: &mut Emu, head: u64) -> u64 {
    let first = slist_first(emu, head);
    if first == 0 {
        return 0;
    }
    let depth = slist_depth(emu, head);
    let rip = emu.pc();
    emu.kernel_guard_access(rip, first, 8, false);
    let next = emu.maps.read_qword(first).unwrap_or(0);
    slist_set(emu, head, next, depth.saturating_sub(1));
    first
}

/// `InsertTailList`: link `entry` before `head`.
pub fn list_insert_tail(emu: &mut Emu, head: u64, entry: u64) -> u64 {
    let blink = emu.maps.read_qword(head + 8).unwrap_or(head);
    emu.maps.write_qword(entry, head); // Flink
    emu.maps.write_qword(entry + 8, blink); // Blink
    emu.maps.write_qword(blink, entry);
    emu.maps.write_qword(head + 8, entry);
    if blink == head { 0 } else { blink }
}

/// `InsertHeadList`: link `entry` right after `head`.
pub fn list_insert_head(emu: &mut Emu, head: u64, entry: u64) -> u64 {
    let flink = emu.maps.read_qword(head).unwrap_or(head);
    emu.maps.write_qword(entry, flink);
    emu.maps.write_qword(entry + 8, head);
    emu.maps.write_qword(flink + 8, entry);
    emu.maps.write_qword(head, entry);
    if flink == head { 0 } else { flink }
}

/// `RemoveHeadList`: unlink and return the first entry, 0 when empty.
pub fn list_remove_head(emu: &mut Emu, head: u64) -> u64 {
    let first = emu.maps.read_qword(head).unwrap_or(head);
    if first == head || first == 0 {
        return 0;
    }
    let rip = emu.pc();
    emu.kernel_guard_access(rip, first, 16, false);
    let next = emu.maps.read_qword(first).unwrap_or(head);
    emu.maps.write_qword(head, next);
    emu.maps.write_qword(next + 8, head);
    first
}

/// `RemoveTailList`.
pub fn list_remove_tail(emu: &mut Emu, head: u64) -> u64 {
    let last = emu.maps.read_qword(head + 8).unwrap_or(head);
    if last == head || last == 0 {
        return 0;
    }
    let rip = emu.pc();
    emu.kernel_guard_access(rip, last, 16, false);
    let prev = emu.maps.read_qword(last + 8).unwrap_or(head);
    emu.maps.write_qword(head + 8, prev);
    emu.maps.write_qword(prev, head);
    last
}

fn init_lookaside(emu: &mut Emu, symbol: &str, paged: bool) {
    // (Lookaside, Allocate, Free, Flags, Size, Tag, Depth)
    let l = emu.kernel_arg(0);
    let allocate = emu.kernel_arg(1);
    let free = emu.kernel_arg(2);
    let flags = emu.kernel_arg(3);
    let size = emu.kernel_arg(4) as u32 as u64;
    let tag = emu.kernel_arg(5) as u32 as u64;
    let depth = emu.kernel_arg(6) as u16 as u64;
    emu.maps.write_bytes(l, &[0u8; LOOKASIDE_SIZE as usize]);
    emu.maps.write_qword(l + 8, 1); // HeaderType = 1 (16-byte SLIST_HEADER)
    emu.maps
        .write_word(l + LOOKASIDE_DEPTH, depth.max(4) as u16);
    emu.maps.write_word(l + LOOKASIDE_MAX_DEPTH, 256);
    let pool_type = if paged { 1 } else { 0 } | (flags as u32 & 0xff);
    emu.maps.write_dword(l + LOOKASIDE_TYPE, pool_type);
    emu.maps.write_dword(l + LOOKASIDE_TAG, tag as u32);
    emu.maps.write_dword(l + LOOKASIDE_SIZE_FIELD, size as u32);
    let alloc_stub = if allocate != 0 {
        allocate
    } else {
        emu.kernel_stub_for("ExAllocatePoolWithTag").unwrap_or(0)
    };
    let free_stub = if free != 0 {
        free
    } else {
        emu.kernel_stub_for("ExFreePool").unwrap_or(0)
    };
    emu.maps.write_qword(l + LOOKASIDE_ALLOCATE, alloc_stub);
    emu.maps.write_qword(l + LOOKASIDE_FREE, free_stub);
    log::debug!(
        "{}: list 0x{:x} size {} tag {}",
        symbol,
        l,
        size,
        tag_name(tag)
    );
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- pool allocator ---------------------------------------------------
        // ExAllocatePool2(Flags, NumberOfBytes, Tag) — zeroes unless
        // POOL_FLAG_UNINITIALIZED. ExAllocatePool3 adds (ExtendedParameters, Count).
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
        // ExAllocatePoolWithTag(PoolType, NumberOfBytes, Tag) — no zeroing.
        "ExAllocatePoolWithTag"
        | "ExAllocatePoolWithTagPriority"
        | "ExAllocatePoolWithQuotaTag"
        | "ExAllocatePoolZero"
        | "ExAllocatePoolUninitialized"
        | "ExAllocatePoolQuotaZero"
        | "ExAllocatePoolQuotaUninitialized"
        | "ExAllocatePoolPriorityZero"
        | "ExAllocatePoolPriorityUninitialized" => {
            let size = emu.kernel_arg(1);
            let tag = emu.kernel_arg(2);
            let zero = symbol.ends_with("Zero");
            let ptr = emu.kernel_alloc(Region::Slab, size, &tag_name(tag), symbol, zero);
            emu.set_kernel_ret(ptr);
        }
        // ExAllocatePool(PoolType, NumberOfBytes) / ExAllocatePoolWithQuota.
        "ExAllocatePool" | "ExAllocatePoolWithQuota" => {
            let size = emu.kernel_arg(1);
            let ptr = emu.kernel_alloc(Region::Slab, size, "pool-none", symbol, false);
            emu.set_kernel_ret(ptr);
        }
        "ExFreePool" | "ExFreePoolWithTag" | "ExFreePool2" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
            emu.set_kernel_ret(0);
        }
        "MmAllocateNonCachedMemory"
        | "MmAllocateContiguousMemory"
        | "MmAllocateContiguousMemorySpecifyCache"
        | "MmAllocateContiguousMemorySpecifyCacheNode"
        | "MmAllocateContiguousNodeMemory"
        | "MmAllocateContiguousMemoryEx" => {
            let size = emu.kernel_arg(0);
            let ptr = emu.kernel_alloc(Region::Pages, size, "contiguous", symbol, true);
            emu.set_kernel_ret(ptr);
        }
        "MmFreeNonCachedMemory"
        | "MmFreeContiguousMemory"
        | "MmFreeContiguousMemorySpecifyCache" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }
        "MmAllocateMappingAddress" => {
            let size = emu.kernel_arg(0);
            let ptr = emu.kernel_alloc(Region::Pages, size, "mapping-address", symbol, true);
            emu.set_kernel_ret(ptr);
        }
        "MmFreeMappingAddress" => {
            let ptr = emu.kernel_arg(0);
            emu.kernel_free(ptr, symbol);
        }
        // HalAllocateCommonBuffer(Adapter, Length, &LogicalAddress, CacheEnabled)
        "HalAllocateCommonBuffer" => {
            let size = emu.kernel_arg(1);
            let logical = emu.kernel_arg(2);
            let ptr = emu.kernel_alloc(Region::Pages, size, "common-buffer", symbol, true);
            if logical != 0 {
                emu.maps.write_qword(logical, ptr & 0xffff_ffff);
            }
            emu.set_kernel_ret(ptr);
        }
        "HalFreeCommonBuffer" => {
            let ptr = emu.kernel_arg(2);
            emu.kernel_free(ptr, symbol);
        }

        // --- lookaside lists --------------------------------------------------
        "ExInitializeNPagedLookasideList" => init_lookaside(emu, symbol, false),
        "ExInitializePagedLookasideList" => init_lookaside(emu, symbol, true),
        "ExInitializeLookasideListEx" => {
            // (Lookaside, Allocate, Free, PoolType, Flags, Size, Tag, Depth)
            let l = emu.kernel_arg(0);
            let allocate = emu.kernel_arg(1);
            let free = emu.kernel_arg(2);
            let pool_type = emu.kernel_arg(3);
            let size = emu.kernel_arg(5);
            let tag = emu.kernel_arg(6);
            emu.maps.write_bytes(l, &[0u8; LOOKASIDE_SIZE as usize]);
            emu.maps.write_qword(l + 8, 1);
            emu.maps.write_word(l + LOOKASIDE_DEPTH, 4);
            emu.maps.write_word(l + LOOKASIDE_MAX_DEPTH, 256);
            emu.maps.write_dword(l + LOOKASIDE_TYPE, pool_type as u32);
            emu.maps.write_dword(l + LOOKASIDE_TAG, tag as u32);
            emu.maps.write_dword(l + LOOKASIDE_SIZE_FIELD, size as u32);
            let a = if allocate != 0 {
                allocate
            } else {
                emu.kernel_stub_for("ExAllocatePoolWithTag").unwrap_or(0)
            };
            let f = if free != 0 {
                free
            } else {
                emu.kernel_stub_for("ExFreePool").unwrap_or(0)
            };
            emu.maps.write_qword(l + LOOKASIDE_ALLOCATE, a);
            emu.maps.write_qword(l + LOOKASIDE_FREE, f);
            emu.set_kernel_ret(0);
        }
        "ExDeleteNPagedLookasideList"
        | "ExDeletePagedLookasideList"
        | "ExDeleteLookasideListEx" => {
            // Entries parked on the SList are returned to the pool.
            let l = emu.kernel_arg(0);
            loop {
                let e = slist_pop(emu, l + LOOKASIDE_LIST_HEAD);
                if e == 0 {
                    break;
                }
                emu.kernel_free(e, symbol);
            }
            emu.set_kernel_ret(0);
        }
        // Out-of-line forms of the inline allocate/free helpers.
        "ExAllocateFromNPagedLookasideList"
        | "ExAllocateFromPagedLookasideList"
        | "ExAllocateFromLookasideListEx" => {
            let l = emu.kernel_arg(0);
            let mut e = slist_pop(emu, l + LOOKASIDE_LIST_HEAD);
            if e == 0 {
                let size = emu.maps.read_dword(l + LOOKASIDE_SIZE_FIELD).unwrap_or(0) as u64;
                let tag = emu.maps.read_dword(l + LOOKASIDE_TAG).unwrap_or(0) as u64;
                e = emu.kernel_alloc(Region::Slab, size, &tag_name(tag), symbol, false);
            }
            emu.set_kernel_ret(e);
        }
        "ExFreeToNPagedLookasideList"
        | "ExFreeToPagedLookasideList"
        | "ExFreeToLookasideListEx" => {
            let l = emu.kernel_arg(0);
            let e = emu.kernel_arg(1);
            let depth = slist_depth(emu, l + LOOKASIDE_LIST_HEAD);
            let max = emu.maps.read_word(l + LOOKASIDE_DEPTH).unwrap_or(4);
            if depth < max {
                slist_push(emu, l + LOOKASIDE_LIST_HEAD, e);
            } else {
                emu.kernel_free(e, symbol);
            }
            emu.set_kernel_ret(0);
        }

        // --- SList ----------------------------------------------------------------
        "ExpInterlockedPushEntrySList"
        | "ExInterlockedPushEntrySList"
        | "InterlockedPushEntrySList" => {
            let head = emu.kernel_arg(0);
            let entry = emu.kernel_arg(1);
            let old = slist_push(emu, head, entry);
            emu.set_kernel_ret(old);
        }
        "ExpInterlockedPopEntrySList"
        | "ExInterlockedPopEntrySList"
        | "InterlockedPopEntrySList" => {
            let head = emu.kernel_arg(0);
            let e = slist_pop(emu, head);
            emu.set_kernel_ret(e);
        }
        "ExpInterlockedFlushSList" | "ExInterlockedFlushSList" | "InterlockedFlushSList" => {
            let head = emu.kernel_arg(0);
            let first = slist_first(emu, head);
            slist_set(emu, head, 0, 0);
            emu.set_kernel_ret(first);
        }
        "ExQueryDepthSList" => {
            let head = emu.kernel_arg(0);
            let d = slist_depth(emu, head);
            emu.set_kernel_ret(d as u64);
        }
        "InitializeSListHead" | "ExInitializeSListHead" | "RtlInitializeSListHead" => {
            let head = emu.kernel_arg(0);
            emu.maps.write_qword(head, 0);
            emu.maps.write_qword(head + 8, 1);
            emu.set_kernel_ret(0);
        }

        // --- interlocked doubly-linked lists ----------------------------------
        "ExInterlockedInsertTailList" | "ExfInterlockedInsertTailList" => {
            let head = emu.kernel_arg(0);
            let entry = emu.kernel_arg(1);
            let old = list_insert_tail(emu, head, entry);
            emu.set_kernel_ret(old);
        }
        "ExInterlockedInsertHeadList" | "ExfInterlockedInsertHeadList" => {
            let head = emu.kernel_arg(0);
            let entry = emu.kernel_arg(1);
            let old = list_insert_head(emu, head, entry);
            emu.set_kernel_ret(old);
        }
        "ExInterlockedRemoveHeadList" | "ExfInterlockedRemoveHeadList" => {
            let head = emu.kernel_arg(0);
            let e = list_remove_head(emu, head);
            emu.set_kernel_ret(e);
        }
        "ExInterlockedRemoveTailList" | "ExfInterlockedRemoveTailList" => {
            let head = emu.kernel_arg(0);
            let e = list_remove_tail(emu, head);
            emu.set_kernel_ret(e);
        }
        // Singly-linked (SINGLE_LIST_ENTRY) variants.
        "ExInterlockedPushEntryList" | "ExfInterlockedPushEntryList" => {
            let head = emu.kernel_arg(0);
            let entry = emu.kernel_arg(1);
            let old = emu.maps.read_qword(head).unwrap_or(0);
            emu.maps.write_qword(entry, old);
            emu.maps.write_qword(head, entry);
            emu.set_kernel_ret(old);
        }
        "ExInterlockedPopEntryList" | "ExfInterlockedPopEntryList" => {
            let head = emu.kernel_arg(0);
            let first = emu.maps.read_qword(head).unwrap_or(0);
            if first != 0 {
                let rip = emu.pc();
                emu.kernel_guard_access(rip, first, 8, false);
                let next = emu.maps.read_qword(first).unwrap_or(0);
                emu.maps.write_qword(head, next);
            }
            emu.set_kernel_ret(first);
        }
        "ExInterlockedAddLargeInteger" | "ExInterlockedAddLargeStatistic" => {
            let addend = emu.kernel_arg(0);
            let inc = emu.kernel_arg(1);
            let old = emu.maps.read_qword(addend).unwrap_or(0);
            emu.maps.write_qword(addend, old.wrapping_add(inc));
            emu.set_kernel_ret(old);
        }
        "ExInterlockedAddUlong" => {
            let addend = emu.kernel_arg(0);
            let inc = emu.kernel_arg(1) as u32;
            let old = emu.maps.read_dword(addend).unwrap_or(0);
            emu.maps.write_dword(addend, old.wrapping_add(inc));
            emu.set_kernel_ret(old as u64);
        }
        // x86-only exports that an x64 driver never imports, kept for the
        // surface listing and for hand-written test drivers.
        "InterlockedIncrement" => {
            let p = emu.kernel_arg(0);
            let v = emu.maps.read_dword(p).unwrap_or(0).wrapping_add(1);
            emu.maps.write_dword(p, v);
            emu.set_kernel_ret(v as u64);
        }
        "InterlockedDecrement" => {
            let p = emu.kernel_arg(0);
            let v = emu.maps.read_dword(p).unwrap_or(0).wrapping_sub(1);
            emu.maps.write_dword(p, v);
            emu.set_kernel_ret(v as u64);
        }
        "InterlockedExchange" => {
            let p = emu.kernel_arg(0);
            let v = emu.kernel_arg(1) as u32;
            let old = emu.maps.read_dword(p).unwrap_or(0);
            emu.maps.write_dword(p, v);
            emu.set_kernel_ret(old as u64);
        }
        "InterlockedCompareExchange" => {
            let p = emu.kernel_arg(0);
            let v = emu.kernel_arg(1) as u32;
            let cmp = emu.kernel_arg(2) as u32;
            let old = emu.maps.read_dword(p).unwrap_or(0);
            if old == cmp {
                emu.maps.write_dword(p, v);
            }
            emu.set_kernel_ret(old as u64);
        }
        _ => return false,
    }
    true
}

pub const SURFACE: &[&str] = &[
    "ExAllocatePool",
    "ExAllocatePool2",
    "ExAllocatePool3",
    "ExAllocatePoolWithTag",
    "ExAllocatePoolWithTagPriority",
    "ExAllocatePoolWithQuota",
    "ExAllocatePoolWithQuotaTag",
    "ExAllocatePoolZero",
    "ExAllocatePoolUninitialized",
    "ExAllocatePoolQuotaZero",
    "ExAllocatePoolQuotaUninitialized",
    "ExAllocatePoolPriorityZero",
    "ExAllocatePoolPriorityUninitialized",
    "ExFreePool",
    "ExFreePool2",
    "ExFreePoolWithTag",
    "MmAllocateNonCachedMemory",
    "MmFreeNonCachedMemory",
    "MmAllocateContiguousMemory",
    "MmAllocateContiguousMemorySpecifyCache",
    "MmAllocateContiguousMemorySpecifyCacheNode",
    "MmAllocateContiguousNodeMemory",
    "MmAllocateContiguousMemoryEx",
    "MmFreeContiguousMemory",
    "MmFreeContiguousMemorySpecifyCache",
    "MmAllocateMappingAddress",
    "MmFreeMappingAddress",
    "HalAllocateCommonBuffer",
    "HalFreeCommonBuffer",
    "ExInitializeNPagedLookasideList",
    "ExInitializePagedLookasideList",
    "ExInitializeLookasideListEx",
    "ExDeleteNPagedLookasideList",
    "ExDeletePagedLookasideList",
    "ExDeleteLookasideListEx",
    "ExAllocateFromNPagedLookasideList",
    "ExAllocateFromPagedLookasideList",
    "ExAllocateFromLookasideListEx",
    "ExFreeToNPagedLookasideList",
    "ExFreeToPagedLookasideList",
    "ExFreeToLookasideListEx",
    "ExpInterlockedPushEntrySList",
    "ExpInterlockedPopEntrySList",
    "ExpInterlockedFlushSList",
    "ExInterlockedPushEntrySList",
    "ExInterlockedPopEntrySList",
    "ExInterlockedFlushSList",
    "ExQueryDepthSList",
    "ExInitializeSListHead",
    "ExInterlockedInsertTailList",
    "ExInterlockedInsertHeadList",
    "ExInterlockedRemoveHeadList",
    "ExInterlockedRemoveTailList",
    "ExInterlockedPushEntryList",
    "ExInterlockedPopEntryList",
    "ExInterlockedAddLargeInteger",
    "ExInterlockedAddLargeStatistic",
    "ExInterlockedAddUlong",
    "InterlockedIncrement",
    "InterlockedDecrement",
    "InterlockedExchange",
    "InterlockedCompareExchange",
];
