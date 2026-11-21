use crate::api::dispatch_api;
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::common::ntdll::free_counted_string;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if heap_engine::dispatch(api, emu) {
        return true;
    }
    dispatch_api!(api, {
        "RtlGetProcessHeaps" => heap_engine::process_heaps(emu, true),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
        "RtlReAllocateHeap" => RtlReAllocateHeap(emu),
        "RtlFreeAnsiString" => free_counted_string(emu, api),
        "HeapAlloc" => RtlAllocateHeap(emu),
        "HeapFree" => RtlFreeHeap(emu),
        "HeapReAlloc" => RtlReAllocateHeap(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    heap_engine::alloc_api(emu);
}

pub fn RtlFreeHeap(emu: &mut emu::Emu) {
    heap_engine::free_api(emu);
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    heap_engine::realloc_api(emu);
}
