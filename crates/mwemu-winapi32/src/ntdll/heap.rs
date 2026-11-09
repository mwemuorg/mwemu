use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if heap_engine::dispatch(api, emu) {
        return true;
    }
    match api {
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
        "RtlReAllocateHeap" => RtlReAllocateHeap(emu),
        "RtlFreeAnsiString" => RtlFreeAnsiString(emu),
        "HeapAlloc" => RtlAllocateHeap(emu),
        "HeapFree" => RtlFreeHeap(emu),
        "HeapReAlloc" => RtlReAllocateHeap(emu),
        "HeapSize" => {
            let _heap = arg(emu, 0);
            let _flags = arg(emu, 1);
            let ptr = arg(emu, 2) as u64;
            let sz = if ptr != 0 {
                emu.maps.get_mem_size(ptr).unwrap_or(0) as u64
            } else {
                0
            };
            log_red!(emu, "ntdll!HeapSize ptr:0x{:x} = {}", ptr, sz);
            emu.regs_mut().rax = sz;
            stdcall_ret(emu, 3);
        }
        _ => return false,
    }
    true
}

pub fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let count = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

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
        emu.stack_pop32(false);
        emu.stack_pop32(false);
        emu.regs_mut().rax = total;
        return;
    }

    let to_write = std::cmp::min(count, total);
    for (i, key) in keys.iter().take(to_write as usize).enumerate() {
        let addr = emu
            .handle_management
            .heap_base_addr(*key)
            .unwrap_or(*key as u64);
        let _ = emu.maps.write_dword(buffer + (i as u64) * 4, addr as u32);
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    if to_write < count {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = to_write;
    }
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

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let ptr = arg(emu, 0);
    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{:x}", ptr);
    stdcall_ret(emu, 1);
}
