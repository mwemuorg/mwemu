use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

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

#[path = "../heap_size.rs"]
mod heap_size;
pub use heap_size::*;

#[path = "../local_alloc.rs"]
mod local_alloc;
pub use local_alloc::*;

#[path = "../map_view_of_file.rs"]
mod map_view_of_file;
pub use map_view_of_file::*;

#[path = "../read_process_memory.rs"]
mod read_process_memory;
pub use read_process_memory::*;

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

#[path = "../virtual_query.rs"]
mod virtual_query;
pub use virtual_query::*;

#[path = "../virtual_query_ex.rs"]
mod virtual_query_ex;
pub use virtual_query_ex::*;

#[path = "../write_process_memory.rs"]
mod write_process_memory;
pub use write_process_memory::*;

#[path = "../heap32_first.rs"]
mod heap32_first;
pub use heap32_first::*;

#[path = "../heap32_list_first.rs"]
mod heap32_list_first;
pub use heap32_list_first::*;

#[path = "../heap32_list_next.rs"]
mod heap32_list_next;
pub use heap32_list_next::*;

#[path = "../heap32_next.rs"]
mod heap32_next;
pub use heap32_next::*;

#[path = "../global_alloc.rs"]
mod global_alloc;
pub use global_alloc::*;

#[path = "../global_free.rs"]
mod global_free;
pub use global_free::*;

#[path = "../local_free.rs"]
mod local_free;
pub use local_free::*;

#[path = "../flush_instruction_cache.rs"]
mod flush_instruction_cache;
pub use flush_instruction_cache::FlushInstructionCache;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "GlobalAlloc" => kernel32::GlobalAlloc(emu),
        "GlobalFree" => kernel32::GlobalFree(emu),
        "LocalFree" => kernel32::LocalFree(emu),
        "DecodePointer" => kernel32::DecodePointer(emu),
        "EncodePointer" => kernel32::EncodePointer(emu),
        "GetProcessHeap" => kernel32::GetProcessHeap(emu),
        "HeapAlloc" => kernel32::HeapAlloc(emu),
        "HeapCreate" => kernel32::HeapCreate(emu),
        "HeapDestroy" => kernel32::HeapDestroy(emu),
        "HeapFree" => kernel32::HeapFree(emu),
        "HeapReAlloc" => kernel32::HeapReAlloc(emu),
        "HeapSetInformation" => kernel32::HeapSetInformation(emu),
        "HeapSize" => kernel32::HeapSize(emu),
        "LocalAlloc" => kernel32::LocalAlloc(emu),
        "MapViewOfFile" => kernel32::MapViewOfFile(emu),
        "ReadProcessMemory" => kernel32::ReadProcessMemory(emu),
        "VirtualAlloc" => kernel32::VirtualAlloc(emu),
        "VirtualAllocEx" => kernel32::VirtualAllocEx(emu),
        "VirtualAllocExNuma" => kernel32::VirtualAllocExNuma(emu),
        "VirtualLock" => kernel32::VirtualLock(emu),
        "VirtualFree" => kernel32::VirtualFree(emu),
        "VirtualProtect" => kernel32::VirtualProtect(emu),
        "VirtualProtectEx" => kernel32::VirtualProtectEx(emu),
        "VirtualQuery" => kernel32::VirtualQuery(emu),
        "VirtualQueryEx" => kernel32::VirtualQueryEx(emu),
        "WriteProcessMemory" => kernel32::WriteProcessMemory(emu),
        "GetNumaAvailableMemoryNode" => kernel32::GetNumaAvailableMemoryNode(emu),
        "GetNumaAvailableMemoryNodeEx" => kernel32::GetNumaAvailableMemoryNodeEx(emu),
        "GetProcessWorkingSetSize" => kernel32::GetProcessWorkingSetSize(emu),
        "GlobalFix" => kernel32::GlobalFix(emu),
        "GlobalFlags" => kernel32::GlobalFlags(emu),
        "GlobalLock" => kernel32::GlobalLock(emu),
        "GlobalMemoryStatus" => kernel32::GlobalMemoryStatus(emu),
        "GlobalReAlloc" => kernel32::GlobalReAlloc(emu),
        "GlobalSize" => kernel32::GlobalSize(emu),
        "GlobalUnWire" => kernel32::GlobalUnWire(emu),
        "GlobalUnfix" => kernel32::GlobalUnfix(emu),
        "GlobalUnlock" => kernel32::GlobalUnlock(emu),
        "GlobalWire" => kernel32::GlobalWire(emu),
        "Heap32First" => kernel32::Heap32First(emu),
        "Heap32ListFirst" => kernel32::Heap32ListFirst(emu),
        "Heap32ListNext" => kernel32::Heap32ListNext(emu),
        "Heap32Next" => kernel32::Heap32Next(emu),
        "IsBadCodePtr" => kernel32::IsBadCodePtr(emu),
        "IsBadHugeReadPtr" => kernel32::IsBadHugeReadPtr(emu),
        "IsBadHugeWritePtr" => kernel32::IsBadHugeWritePtr(emu),
        "IsBadReadPtr" => kernel32::IsBadReadPtr(emu),
        "IsBadStringPtrA" => kernel32::IsBadStringPtrA(emu),
        "IsBadStringPtrW" => kernel32::IsBadStringPtrW(emu),
        "IsBadWritePtr" => kernel32::IsBadWritePtr(emu),
        "LocalCompact" => kernel32::LocalCompact(emu),
        "LocalFlags" => kernel32::LocalFlags(emu),
        "LocalShrink" => kernel32::LocalShrink(emu),
        "LocalSize" => kernel32::LocalSize(emu),
        "RemoveSecureMemoryCacheCallback" => kernel32::RemoveSecureMemoryCacheCallback(emu),
        "ReturnMem16Data" => kernel32::ReturnMem16Data(emu),
        "SetProcessWorkingSetSize" => kernel32::SetProcessWorkingSetSize(emu),
        "AddSecureMemoryCacheCallback" => kernel32::AddSecureMemoryCacheCallback(emu),
        "AllocateUserPhysicalPages" => kernel32::AllocateUserPhysicalPages(emu),
        "AllocateUserPhysicalPagesNuma" => kernel32::AllocateUserPhysicalPagesNuma(emu),
        "CreateEnclave" => kernel32::CreateEnclave(emu),
        "DecodeSystemPointer" => kernel32::DecodeSystemPointer(emu),
        "DiscardVirtualMemory" => kernel32::DiscardVirtualMemory(emu),
        "EncodeSystemPointer" => kernel32::EncodeSystemPointer(emu),
        "FlushInstructionCache" => kernel32::FlushInstructionCache(emu),
        "FlushViewOfFile" => kernel32::FlushViewOfFile(emu),
        "FreeUserPhysicalPages" => kernel32::FreeUserPhysicalPages(emu),
        "GetLargePageMinimum" => kernel32::GetLargePageMinimum(emu),
        "GetMemoryErrorHandlingCapabilities" => kernel32::GetMemoryErrorHandlingCapabilities(emu),
        "GetPhysicallyInstalledSystemMemory" => kernel32::GetPhysicallyInstalledSystemMemory(emu),
        "GetProcessHeaps" => kernel32::GetProcessHeaps(emu),
        "GetProcessWorkingSetSizeEx" => kernel32::GetProcessWorkingSetSizeEx(emu),
        "GetWriteWatch" => kernel32::GetWriteWatch(emu),
        "GlobalCompact" => kernel32::GlobalCompact(emu),
        "GlobalMemoryStatusEx" => kernel32::GlobalMemoryStatusEx(emu),
        "HeapCompact" => kernel32::HeapCompact(emu),
        "HeapLock" => kernel32::HeapLock(emu),
        "HeapQueryInformation" => kernel32::HeapQueryInformation(emu),
        "HeapSummary" => kernel32::HeapSummary(emu),
        "HeapUnlock" => kernel32::HeapUnlock(emu),
        "HeapValidate" => kernel32::HeapValidate(emu),
        "HeapWalk" => kernel32::HeapWalk(emu),
        "InitializeEnclave" => kernel32::InitializeEnclave(emu),
        "IsEnclaveTypeSupported" => kernel32::IsEnclaveTypeSupported(emu),
        "K32EmptyWorkingSet" => kernel32::K32EmptyWorkingSet(emu),
        "K32GetProcessMemoryInfo" => kernel32::K32GetProcessMemoryInfo(emu),
        "K32GetWsChanges" => kernel32::K32GetWsChanges(emu),
        "K32GetWsChangesEx" => kernel32::K32GetWsChangesEx(emu),
        "K32QueryWorkingSet" => kernel32::K32QueryWorkingSet(emu),
        "K32QueryWorkingSetEx" => kernel32::K32QueryWorkingSetEx(emu),
        "LoadEnclaveData" => kernel32::LoadEnclaveData(emu),
        "LocalLock" => kernel32::LocalLock(emu),
        "LocalReAlloc" => kernel32::LocalReAlloc(emu),
        "LocalUnlock" => kernel32::LocalUnlock(emu),
        "MapUserPhysicalPages" => kernel32::MapUserPhysicalPages(emu),
        "MapUserPhysicalPagesScatter" => kernel32::MapUserPhysicalPagesScatter(emu),
        "MapViewOfFileEx" => kernel32::MapViewOfFileEx(emu),
        "MapViewOfFileExNuma" => kernel32::MapViewOfFileExNuma(emu),
        "MapViewOfFileFromApp" => kernel32::MapViewOfFileFromApp(emu),
        "OfferVirtualMemory" => kernel32::OfferVirtualMemory(emu),
        "PrefetchVirtualMemory" => kernel32::PrefetchVirtualMemory(emu),
        "ReclaimVirtualMemory" => kernel32::ReclaimVirtualMemory(emu),
        "ResetWriteWatch" => kernel32::ResetWriteWatch(emu),
        "RtlFillMemory" => kernel32::RtlFillMemory(emu),
        "RtlMoveMemory" => kernel32::RtlMoveMemory(emu),
        "RtlZeroMemory" => kernel32::RtlZeroMemory(emu),
        "SetProcessWorkingSetSizeEx" => kernel32::SetProcessWorkingSetSizeEx(emu),
        "Toolhelp32ReadProcessMemory" => kernel32::Toolhelp32ReadProcessMemory(emu),
        "UnmapViewOfFile" => kernel32::UnmapViewOfFile(emu),
        "UnmapViewOfFileEx" => kernel32::UnmapViewOfFileEx(emu),
        "UnregisterBadMemoryNotification" => kernel32::UnregisterBadMemoryNotification(emu),
        "VirtualFreeEx" => kernel32::VirtualFreeEx(emu),
        "VirtualUnlock" => kernel32::VirtualUnlock(emu),
        "RtlCompareMemory" => kernel32::RtlCompareMemory(emu),
        "RtlCopyMemory" => kernel32::RtlCopyMemory(emu),
        _ => return false,
    }
    true
}
