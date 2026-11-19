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

use crate::api::abi::windows::ArgumentError;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::policy::api_try;
use crate::kernel::{HandlerRegistry, kernel_handlers};

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

fn init_lookaside(emu: &mut Emu, symbol: &str, paged: bool) -> Result<(), ArgumentError> {
    let call = WinApi(Stdcall);
    // (Lookaside, Allocate, Free, Flags, Size, Tag, Depth)
    let l = call.arg(emu, 0)?;
    let allocate = call.arg(emu, 1)?;
    let free = call.arg(emu, 2)?;
    let flags = call.arg(emu, 3)?;
    let size = call.arg_u32(emu, 4)? as u64;
    let tag = call.arg_u32(emu, 5)? as u64;
    let depth = call.arg_u16(emu, 6)? as u64;
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
    Ok(())
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- pool allocator ---------------------------------------------------
        // ExAllocatePool2(Flags, NumberOfBytes, Tag) — zeroes unless
        // POOL_FLAG_UNINITIALIZED. ExAllocatePool3 adds (ExtendedParameters, Count).
        "ExAllocatePool2" | "ExAllocatePool3" => ExAllocatePool2,
        // ExAllocatePoolWithTag(PoolType, NumberOfBytes, Tag) — no zeroing.
        "ExAllocatePoolWithTag"
        | "ExAllocatePoolWithTagPriority"
        | "ExAllocatePoolWithQuotaTag"
        | "ExAllocatePoolZero"
        | "ExAllocatePoolUninitialized"
        | "ExAllocatePoolQuotaZero"
        | "ExAllocatePoolQuotaUninitialized"
        | "ExAllocatePoolPriorityZero"
        | "ExAllocatePoolPriorityUninitialized" => ExAllocatePoolWithTag,
        // ExAllocatePool(PoolType, NumberOfBytes) / ExAllocatePoolWithQuota.
        "ExAllocatePool" | "ExAllocatePoolWithQuota" => ExAllocatePool,
        "ExFreePool" | "ExFreePoolWithTag" | "ExFreePool2" => ExFreePool,
        "MmAllocateNonCachedMemory"
        | "MmAllocateContiguousMemory"
        | "MmAllocateContiguousMemorySpecifyCache"
        | "MmAllocateContiguousMemorySpecifyCacheNode"
        | "MmAllocateContiguousNodeMemory"
        | "MmAllocateContiguousMemoryEx" => MmAllocateNonCachedMemory,
        "MmFreeNonCachedMemory"
        | "MmFreeContiguousMemory"
        | "MmFreeContiguousMemorySpecifyCache" => MmFreeNonCachedMemory,
        "MmAllocateMappingAddress" => MmAllocateMappingAddress,
        "MmFreeMappingAddress" => MmFreeNonCachedMemory,
        // HalAllocateCommonBuffer(Adapter, Length, &LogicalAddress, CacheEnabled)
        "HalAllocateCommonBuffer" => HalAllocateCommonBuffer,
        "HalFreeCommonBuffer" => HalFreeCommonBuffer,

        // --- lookaside lists --------------------------------------------------
        "ExInitializeNPagedLookasideList" => ExInitializeNPagedLookasideList,
        "ExInitializePagedLookasideList" => ExInitializePagedLookasideList,
        "ExInitializeLookasideListEx" => ExInitializeLookasideListEx,
        "ExDeleteNPagedLookasideList"
        | "ExDeletePagedLookasideList"
        | "ExDeleteLookasideListEx" => ExDeleteNPagedLookasideList,
        // Out-of-line forms of the inline allocate/free helpers.
        "ExAllocateFromNPagedLookasideList"
        | "ExAllocateFromPagedLookasideList"
        | "ExAllocateFromLookasideListEx" => ExAllocateFromNPagedLookasideList,
        "ExFreeToNPagedLookasideList"
        | "ExFreeToPagedLookasideList"
        | "ExFreeToLookasideListEx" => ExFreeToNPagedLookasideList,

        // --- SList ----------------------------------------------------------------
        "ExpInterlockedPushEntrySList"
        | "ExInterlockedPushEntrySList"
        | "InterlockedPushEntrySList" => ExpInterlockedPushEntrySList,
        "ExpInterlockedPopEntrySList"
        | "ExInterlockedPopEntrySList"
        | "InterlockedPopEntrySList" => ExpInterlockedPopEntrySList,
        "ExpInterlockedFlushSList" | "ExInterlockedFlushSList" | "InterlockedFlushSList" => ExpInterlockedFlushSList,
        "ExQueryDepthSList" => ExQueryDepthSList,
        "InitializeSListHead" | "ExInitializeSListHead" | "RtlInitializeSListHead" => InitializeSListHead,

        // --- interlocked doubly-linked lists ----------------------------------
        "ExInterlockedInsertTailList" | "ExfInterlockedInsertTailList" => ExInterlockedInsertTailList,
        "ExInterlockedInsertHeadList" | "ExfInterlockedInsertHeadList" => ExInterlockedInsertHeadList,
        "ExInterlockedRemoveHeadList" | "ExfInterlockedRemoveHeadList" => ExInterlockedRemoveHeadList,
        "ExInterlockedRemoveTailList" | "ExfInterlockedRemoveTailList" => ExInterlockedRemoveTailList,
        // Singly-linked (SINGLE_LIST_ENTRY) variants.
        "ExInterlockedPushEntryList" | "ExfInterlockedPushEntryList" => ExInterlockedPushEntryList,
        "ExInterlockedPopEntryList" | "ExfInterlockedPopEntryList" => ExInterlockedPopEntryList,
        "ExInterlockedAddLargeInteger" | "ExInterlockedAddLargeStatistic" => ExInterlockedAddLargeInteger,
        "ExInterlockedAddUlong" => ExInterlockedAddUlong,
        // x86-only exports that an x64 driver never imports, kept for the
        // surface listing and for hand-written test drivers.
        "InterlockedIncrement" => InterlockedIncrement,
        "InterlockedDecrement" => InterlockedDecrement,
        "InterlockedExchange" => InterlockedExchange,
        "InterlockedCompareExchange" => InterlockedCompareExchange,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn ExAllocatePool2(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let tag = api_try!(emu, call.arg(emu, 2));
    const POOL_FLAG_UNINITIALIZED: u64 = 0x2;
    let ptr = emu.kernel_alloc(
        Region::Slab,
        size,
        &tag_name(tag),
        symbol,
        flags & POOL_FLAG_UNINITIALIZED == 0,
    );
    call.set_return(emu, ptr);
}

fn ExAllocatePoolWithTag(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 1));
    let tag = api_try!(emu, call.arg(emu, 2));
    let zero = symbol.ends_with("Zero");
    let ptr = emu.kernel_alloc(Region::Slab, size, &tag_name(tag), symbol, zero);
    call.set_return(emu, ptr);
}

fn ExAllocatePool(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 1));
    let ptr = emu.kernel_alloc(Region::Slab, size, "pool-none", symbol, false);
    call.set_return(emu, ptr);
}

fn ExFreePool(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(ptr, symbol);
    call.set_return(emu, 0);
}

fn MmAllocateNonCachedMemory(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let ptr = emu.kernel_alloc(Region::Pages, size, "contiguous", symbol, true);
    call.set_return(emu, ptr);
}

fn MmFreeNonCachedMemory(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    emu.kernel_free(ptr, symbol);
}

fn MmAllocateMappingAddress(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let ptr = emu.kernel_alloc(Region::Pages, size, "mapping-address", symbol, true);
    call.set_return(emu, ptr);
}

fn HalAllocateCommonBuffer(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 1));
    let logical = api_try!(emu, call.arg(emu, 2));
    let ptr = emu.kernel_alloc(Region::Pages, size, "common-buffer", symbol, true);
    if logical != 0 {
        emu.maps.write_qword(logical, ptr & 0xffff_ffff);
    }
    call.set_return(emu, ptr);
}

fn HalFreeCommonBuffer(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 2));
    emu.kernel_free(ptr, symbol);
}

fn ExInitializeNPagedLookasideList(emu: &mut Emu, symbol: &str) {
    api_try!(emu, init_lookaside(emu, symbol, false));
}

fn ExInitializePagedLookasideList(emu: &mut Emu, symbol: &str) {
    api_try!(emu, init_lookaside(emu, symbol, true));
}

fn ExInitializeLookasideListEx(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // (Lookaside, Allocate, Free, PoolType, Flags, Size, Tag, Depth)
    let l = api_try!(emu, call.arg(emu, 0));
    let allocate = api_try!(emu, call.arg(emu, 1));
    let free = api_try!(emu, call.arg(emu, 2));
    let pool_type = api_try!(emu, call.arg(emu, 3));
    let size = api_try!(emu, call.arg(emu, 5));
    let tag = api_try!(emu, call.arg(emu, 6));
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
    call.set_return(emu, 0);
}

fn ExDeleteNPagedLookasideList(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // Entries parked on the SList are returned to the pool.
    let l = api_try!(emu, call.arg(emu, 0));
    loop {
        let e = slist_pop(emu, l + LOOKASIDE_LIST_HEAD);
        if e == 0 {
            break;
        }
        emu.kernel_free(e, symbol);
    }
    call.set_return(emu, 0);
}

fn ExAllocateFromNPagedLookasideList(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let l = api_try!(emu, call.arg(emu, 0));
    let mut e = slist_pop(emu, l + LOOKASIDE_LIST_HEAD);
    if e == 0 {
        let size = emu.maps.read_dword(l + LOOKASIDE_SIZE_FIELD).unwrap_or(0) as u64;
        let tag = emu.maps.read_dword(l + LOOKASIDE_TAG).unwrap_or(0) as u64;
        e = emu.kernel_alloc(Region::Slab, size, &tag_name(tag), symbol, false);
    }
    call.set_return(emu, e);
}

fn ExFreeToNPagedLookasideList(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let l = api_try!(emu, call.arg(emu, 0));
    let e = api_try!(emu, call.arg(emu, 1));
    let depth = slist_depth(emu, l + LOOKASIDE_LIST_HEAD);
    let max = emu.maps.read_word(l + LOOKASIDE_DEPTH).unwrap_or(4);
    if depth < max {
        slist_push(emu, l + LOOKASIDE_LIST_HEAD, e);
    } else {
        emu.kernel_free(e, symbol);
    }
    call.set_return(emu, 0);
}

fn ExpInterlockedPushEntrySList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let entry = api_try!(emu, call.arg(emu, 1));
    let old = slist_push(emu, head, entry);
    call.set_return(emu, old);
}

fn ExpInterlockedPopEntrySList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let e = slist_pop(emu, head);
    call.set_return(emu, e);
}

fn ExpInterlockedFlushSList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let first = slist_first(emu, head);
    slist_set(emu, head, 0, 0);
    call.set_return(emu, first);
}

fn ExQueryDepthSList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let d = slist_depth(emu, head);
    call.set_return(emu, d as u64);
}

fn InitializeSListHead(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_qword(head, 0);
    emu.maps.write_qword(head + 8, 1);
    call.set_return(emu, 0);
}

fn ExInterlockedInsertTailList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let entry = api_try!(emu, call.arg(emu, 1));
    let old = list_insert_tail(emu, head, entry);
    call.set_return(emu, old);
}

fn ExInterlockedInsertHeadList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let entry = api_try!(emu, call.arg(emu, 1));
    let old = list_insert_head(emu, head, entry);
    call.set_return(emu, old);
}

fn ExInterlockedRemoveHeadList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let e = list_remove_head(emu, head);
    call.set_return(emu, e);
}

fn ExInterlockedRemoveTailList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let e = list_remove_tail(emu, head);
    call.set_return(emu, e);
}

fn ExInterlockedPushEntryList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let entry = api_try!(emu, call.arg(emu, 1));
    let old = emu.maps.read_qword(head).unwrap_or(0);
    emu.maps.write_qword(entry, old);
    emu.maps.write_qword(head, entry);
    call.set_return(emu, old);
}

fn ExInterlockedPopEntryList(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let head = api_try!(emu, call.arg(emu, 0));
    let first = emu.maps.read_qword(head).unwrap_or(0);
    if first != 0 {
        let rip = emu.pc();
        emu.kernel_guard_access(rip, first, 8, false);
        let next = emu.maps.read_qword(first).unwrap_or(0);
        emu.maps.write_qword(head, next);
    }
    call.set_return(emu, first);
}

fn ExInterlockedAddLargeInteger(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let addend = api_try!(emu, call.arg(emu, 0));
    let inc = api_try!(emu, call.arg(emu, 1));
    let old = emu.maps.read_qword(addend).unwrap_or(0);
    emu.maps.write_qword(addend, old.wrapping_add(inc));
    call.set_return(emu, old);
}

fn ExInterlockedAddUlong(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let addend = api_try!(emu, call.arg(emu, 0));
    let inc = api_try!(emu, call.arg_u32(emu, 1));
    let old = emu.maps.read_dword(addend).unwrap_or(0);
    emu.maps.write_dword(addend, old.wrapping_add(inc));
    call.set_return(emu, old as u64);
}

fn InterlockedIncrement(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let v = emu.maps.read_dword(p).unwrap_or(0).wrapping_add(1);
    emu.maps.write_dword(p, v);
    call.set_return(emu, v as u64);
}

fn InterlockedDecrement(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let v = emu.maps.read_dword(p).unwrap_or(0).wrapping_sub(1);
    emu.maps.write_dword(p, v);
    call.set_return(emu, v as u64);
}

fn InterlockedExchange(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let v = api_try!(emu, call.arg_u32(emu, 1));
    let old = emu.maps.read_dword(p).unwrap_or(0);
    emu.maps.write_dword(p, v);
    call.set_return(emu, old as u64);
}

fn InterlockedCompareExchange(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    let v = api_try!(emu, call.arg_u32(emu, 1));
    let cmp = api_try!(emu, call.arg_u32(emu, 2));
    let old = emu.maps.read_dword(p).unwrap_or(0);
    if old == cmp {
        emu.maps.write_dword(p, v);
    }
    call.set_return(emu, old as u64);
}
