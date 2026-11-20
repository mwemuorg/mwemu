use crate::api::dispatch_api;
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::common::ntdll::free_counted_string;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if heap_engine::dispatch(api, emu) {
        return true;
    }
    dispatch_api!(api, {
        // kernel32 Heap* are forwarders to these Rtl workers (same arg layout:
        // rcx=heap, rdx=flags, r8=ptr/size), so GetProcAddress-resolved callers
        // that land on the ntdll target hit the same handlers.
        "RtlAllocateHeap" | "HeapAlloc" => RtlAllocateHeap(emu),
        "RtlFreeHeap" | "HeapFree" => RtlFreeHeap(emu),
        "RtlReAllocateHeap" | "HeapReAlloc" => RtlReAllocateHeap(emu),
        "RtlGetProcessHeaps" => heap_engine::process_heaps(emu, true),
        "RtlSizeHeap" | "HeapSize" => RtlSizeHeap(emu),
        "RtlFreeAnsiString" => free_counted_string(emu, api),
    },
        handled(()) => true,
        fallback => false,
    )
}

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    heap_engine::alloc_api(emu);
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    heap_engine::free_api(emu);
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    heap_engine::realloc_api(emu);
}

fn RtlSizeHeap(emu: &mut emu::Emu) {
    heap_engine::size_api(emu);
}
