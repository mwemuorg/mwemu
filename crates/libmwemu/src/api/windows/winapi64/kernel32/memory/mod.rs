pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GlobalLock, GlobalUnlock, HeapSetInformation,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::kernel32::memory::{
    IsBadCodePtr, IsBadHugeReadPtr, IsBadHugeWritePtr, IsBadWritePtr,
};
pub use crate::winapi::libraries::unsupported::kernel32::VirtualLock;
use crate::winapi::winapi64::kernel32;

pub use crate::api::windows::libraries::kernel32::memory::*;

pub use crate::winapi::common::ntdll::{DecodePointer, EncodePointer};

pub use crate::winapi::common::heap::GetProcessHeap;

mod virtual_alloc_common;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "IsBadCodePtr" => IsBadCodePtr(emu),
        "IsBadHugeReadPtr" => IsBadHugeReadPtr(emu),
        "IsBadHugeWritePtr" => IsBadHugeWritePtr(emu),
        "IsBadWritePtr" => IsBadWritePtr(emu),
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
    },
        handled(()) => true,
        fallback => false,
    )
}
