pub use crate::winapi::libraries::kernel32::entry_points::kernel32::HeapSetInformation;

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::fallback::unsupported_exports;
pub use crate::winapi::libraries::unsupported::kernel32::VirtualLock;
use crate::winapi::winapi32::kernel32;

pub use crate::api::windows::libraries::kernel32::memory::*;

pub use crate::winapi::common::ntdll::{DecodePointer, EncodePointer};

pub use crate::winapi::common::heap::GetProcessHeap;

mod virtual_alloc_common;

mod virtual_query;
pub use virtual_query::VirtualQuery;

mod virtual_query_ex;
pub use virtual_query_ex::VirtualQueryEx;

unsupported_exports!(
    "kernel32",
    Heap32First,
    Heap32ListFirst,
    Heap32ListNext,
    Heap32Next
);

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
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
        "CreateEnclave" => kernel32::CreateEnclave(emu),
        "DecodeSystemPointer" => kernel32::DecodeSystemPointer(emu),
        "DiscardVirtualMemory" => kernel32::DiscardVirtualMemory(emu),
        "EncodeSystemPointer" => kernel32::EncodeSystemPointer(emu),
        "FlushInstructionCache" => kernel32::FlushInstructionCache(emu),
        "FlushViewOfFile" => kernel32::FlushViewOfFile(emu),
        "GetPhysicallyInstalledSystemMemory" => kernel32::GetPhysicallyInstalledSystemMemory(emu),
        "GetProcessHeaps" => kernel32::GetProcessHeaps(emu),
        "GlobalMemoryStatusEx" => kernel32::GlobalMemoryStatusEx(emu),
        "InitializeEnclave" => kernel32::InitializeEnclave(emu),
        "IsEnclaveTypeSupported" => kernel32::IsEnclaveTypeSupported(emu),
        "LoadEnclaveData" => kernel32::LoadEnclaveData(emu),
        "LocalLock" => kernel32::LocalLock(emu),
        "LocalReAlloc" => kernel32::LocalReAlloc(emu),
        "LocalUnlock" => kernel32::LocalUnlock(emu),
        "MapViewOfFileEx" => kernel32::MapViewOfFileEx(emu),
        "MapViewOfFileExNuma" => kernel32::MapViewOfFileExNuma(emu),
        "MapViewOfFileFromApp" => kernel32::MapViewOfFileFromApp(emu),
        "OfferVirtualMemory" => kernel32::OfferVirtualMemory(emu),
        "PrefetchVirtualMemory" => kernel32::PrefetchVirtualMemory(emu),
        "ReclaimVirtualMemory" => kernel32::ReclaimVirtualMemory(emu),
        "RtlFillMemory" => kernel32::RtlFillMemory(emu),
        "RtlMoveMemory" => kernel32::RtlMoveMemory(emu),
        "RtlZeroMemory" => kernel32::RtlZeroMemory(emu),
        "UnmapViewOfFile" => kernel32::UnmapViewOfFile(emu),
        "UnmapViewOfFileEx" => kernel32::UnmapViewOfFileEx(emu),
        "VirtualFreeEx" => kernel32::VirtualFreeEx(emu),
        "RtlCompareMemory" => kernel32::RtlCompareMemory(emu),
        "RtlCopyMemory" => kernel32::RtlCopyMemory(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
