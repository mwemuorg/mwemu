use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod decode_pointer;
pub use decode_pointer::DecodePointer;

mod encode_pointer;
pub use encode_pointer::EncodePointer;

mod get_process_heap;
pub use get_process_heap::GetProcessHeap;

mod heap_alloc;
pub use heap_alloc::HeapAlloc;

mod heap_create;
pub use heap_create::HeapCreate;

mod heap_destroy;
pub use heap_destroy::HeapDestroy;

mod heap_free;
pub use heap_free::HeapFree;

mod heap_re_alloc;
pub use heap_re_alloc::HeapReAlloc;

mod heap_set_information;
pub use heap_set_information::HeapSetInformation;

mod local_alloc;
pub use local_alloc::LocalAlloc;

mod map_view_of_file;
pub use map_view_of_file::MapViewOfFile;

mod read_process_memory;
pub use read_process_memory::ReadProcessMemory;

mod local_free;
pub use local_free::LocalFree;

mod virtual_alloc_common;

mod virtual_alloc;
pub use virtual_alloc::VirtualAlloc;

mod virtual_alloc_ex;
pub use virtual_alloc_ex::VirtualAllocEx;

mod virtual_alloc_ex_numa;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;

mod virtual_free;
pub use virtual_free::VirtualFree;

mod virtual_lock;
pub use virtual_lock::VirtualLock;

mod virtual_protect;
pub use virtual_protect::VirtualProtect;

mod virtual_protect_ex;
pub use virtual_protect_ex::VirtualProtectEx;

mod write_process_memory;
pub use write_process_memory::WriteProcessMemory;

mod global_alloc;
pub use global_alloc::GlobalAlloc;

mod global_free;
pub use global_free::GlobalFree;

mod global_lock;
pub use global_lock::GlobalLock;

mod global_unlock;
pub use global_unlock::GlobalUnlock;

mod heap_size;
pub use heap_size::HeapSize;

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
