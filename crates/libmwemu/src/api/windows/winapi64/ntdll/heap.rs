use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if heap_engine::dispatch(api, emu) {
        return true;
    }
    match api {
        // kernel32 Heap* are forwarders to these Rtl workers (same arg layout:
        // rcx=heap, rdx=flags, r8=ptr/size), so GetProcAddress-resolved callers
        // that land on the ntdll target hit the same handlers.
        "RtlAllocateHeap" | "HeapAlloc" => RtlAllocateHeap(emu),
        "RtlFreeHeap" | "HeapFree" => RtlFreeHeap(emu),
        "RtlReAllocateHeap" | "HeapReAlloc" => RtlReAllocateHeap(emu),
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlSizeHeap" | "HeapSize" => RtlSizeHeap(emu),
        "RtlFreeAnsiString" => RtlFreeAnsiString(emu),
        _ => return false,
    }
    true
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

pub fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));

    let keys = emu.handle_management.heap_handle_keys();
    let total = keys.len() as u64;
    log_red!(
        emu,
        "ntdll!RtlGetProcessHeaps count={} buffer=0x{:x} total={}",
        count,
        buffer,
        total
    );

    if buffer == 0 {
        call.set_return(emu, total);
        return;
    }

    let to_write = std::cmp::min(count, total);
    for (i, key) in keys.iter().take(to_write as usize).enumerate() {
        let addr = emu
            .handle_management
            .heap_base_addr(*key)
            .unwrap_or(*key as u64);
        let _ = emu.maps.write_qword(buffer + (i as u64) * 8, addr);
    }

    if to_write < count {
        call.set_return(emu, 0);
    } else {
        call.set_return(emu, to_write);
    }
}

fn RtlSizeHeap(emu: &mut emu::Emu) {
    heap_engine::size_api(emu);
}

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{}", ptr);
}
