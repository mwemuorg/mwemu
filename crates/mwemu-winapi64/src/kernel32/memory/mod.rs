use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../decode_pointer.rs"]
mod decode_pointer;
pub use decode_pointer::*;

#[path = "../encode_pointer.rs"]
mod encode_pointer;
pub use encode_pointer::*;

#[path = "../get_process_heap.rs"]
mod get_process_heap;
pub use get_process_heap::*;

#[path = "../heap_alloc.rs"]
mod heap_alloc;
pub use heap_alloc::*;

#[path = "../heap_create.rs"]
mod heap_create;
pub use heap_create::*;

#[path = "../heap_destroy.rs"]
mod heap_destroy;
pub use heap_destroy::*;

#[path = "../heap_free.rs"]
mod heap_free;
pub use heap_free::*;

#[path = "../heap_re_alloc.rs"]
mod heap_re_alloc;
pub use heap_re_alloc::*;

#[path = "../heap_set_information.rs"]
mod heap_set_information;
pub use heap_set_information::*;

#[path = "../local_alloc.rs"]
mod local_alloc;
pub use local_alloc::*;

#[path = "../map_view_of_file.rs"]
mod map_view_of_file;
pub use map_view_of_file::*;

#[path = "../read_process_memory.rs"]
mod read_process_memory;
pub use read_process_memory::*;

#[path = "../local_free.rs"]
mod local_free;
pub use local_free::*;

#[path = "../virtual_alloc_common.rs"]
mod virtual_alloc_common;

#[path = "../virtual_alloc.rs"]
mod virtual_alloc;
pub use virtual_alloc::*;

#[path = "../virtual_alloc_ex.rs"]
mod virtual_alloc_ex;
pub use virtual_alloc_ex::*;

#[path = "../virtual_alloc_ex_numa.rs"]
mod virtual_alloc_ex_numa;
pub use virtual_alloc_ex_numa::*;

#[path = "../virtual_free.rs"]
mod virtual_free;
pub use virtual_free::*;

#[path = "../virtual_lock.rs"]
mod virtual_lock;
pub use virtual_lock::*;

#[path = "../virtual_protect.rs"]
mod virtual_protect;
pub use virtual_protect::*;

#[path = "../virtual_protect_ex.rs"]
mod virtual_protect_ex;
pub use virtual_protect_ex::*;

#[path = "../write_process_memory.rs"]
mod write_process_memory;
pub use write_process_memory::*;

#[path = "../global_alloc.rs"]
mod global_alloc;
pub use global_alloc::*;

#[path = "../global_free.rs"]
mod global_free;
pub use global_free::*;

#[path = "../global_lock.rs"]
mod global_lock;
pub use global_lock::*;

#[path = "../global_unlock.rs"]
mod global_unlock;
pub use global_unlock::*;

#[path = "../heap_size.rs"]
mod heap_size;
pub use heap_size::*;

#[path = "../flush_instruction_cache.rs"]
mod flush_instruction_cache;
pub use flush_instruction_cache::FlushInstructionCache;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "FlushInstructionCache" => kernel32::FlushInstructionCache(emu),
        "DecodePointer" => kernel32::DecodePointer(emu),
        "EncodePointer" => kernel32::EncodePointer(emu),
        "GetProcessHeap" => kernel32::GetProcessHeap(emu),
        "GlobalAlloc" => kernel32::GlobalAlloc(emu),
        "GlobalFree" => kernel32::GlobalFree(emu),
        "GlobalLock" => kernel32::GlobalLock(emu),
        "GlobalUnlock" => kernel32::GlobalUnlock(emu),
        "HeapAlloc" => kernel32::HeapAlloc(emu),
        "HeapCreate" => kernel32::HeapCreate(emu),
        "HeapDestroy" => kernel32::HeapDestroy(emu),
        "HeapSetInformation" => kernel32::HeapSetInformation(emu),
        "HeapFree" => kernel32::HeapFree(emu),
        "HeapReAlloc" => kernel32::HeapReAlloc(emu),
        "HeapSize" => kernel32::HeapSize(emu),
        "IsBadReadPtr" => kernel32::IsBadReadPtr(emu),
        "LocalAlloc" => kernel32::LocalAlloc(emu),
        "LocalFree" => kernel32::LocalFree(emu),
        "MapViewOfFile" => kernel32::MapViewOfFile(emu),
        "ReadProcessMemory" => kernel32::ReadProcessMemory(emu),
        "VirtualAlloc" => kernel32::VirtualAlloc(emu),
        "VirtualAllocEx" => kernel32::VirtualAllocEx(emu),
        "VirtualAllocExNuma" => kernel32::VirtualAllocExNuma(emu),
        "VirtualFree" => kernel32::VirtualFree(emu),
        "VirtualLock" => kernel32::VirtualLock(emu),
        "VirtualProtect" => kernel32::VirtualProtect(emu),
        "VirtualProtectEx" => kernel32::VirtualProtectEx(emu),
        "WriteProcessMemory" => kernel32::WriteProcessMemory(emu),
        "VirtualQuery" => kernel32::VirtualQuery(emu, addr),
        "VirtualQueryEx" => kernel32::VirtualQueryEx(emu, addr),
        "GetProcessWorkingSetSize" => kernel32::GetProcessWorkingSetSize(emu),
        "SetProcessWorkingSetSize" => kernel32::SetProcessWorkingSetSize(emu),
        "GlobalFlags" => kernel32::GlobalFlags(emu),
        "GlobalSize" => kernel32::GlobalSize(emu),
        "GlobalReAlloc" => kernel32::GlobalReAlloc(emu, addr),
        _ => return false,
    }
    true
}
