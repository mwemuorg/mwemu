use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::{fallback, memory as memory_api, nt_api};
use crate::emu;
use crate::winapi::winapi64::kernel32;

mod file;
mod heap;
mod loader;
mod memory;
mod misc;
mod string;
mod sync;

#[cfg(test)]
pub use heap::{RtlAllocateHeap, RtlReAllocateHeap};
#[cfg(test)]
pub use memory::{NtCreateSection, NtFreeVirtualMemory, NtMapViewOfSection, NtUnmapViewOfSection};
#[cfg(test)]
pub use misc::{NtQueryInformationProcess, NtQuerySystemInformation, RtlDecompressBuffer};
#[cfg(test)]
pub use string::{RtlInitAnsiString, RtlInitUnicodeString};
#[cfg(test)]
pub use sync::NtDelayExecution;

pub fn try_dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    let call = WinApi(Stdcall);
    if memory_api::dispatch_nt(api, emu, call) {
        return true;
    }
    let normalized = nt_api::normalize(api);
    let api = normalized.as_ref();
    file::dispatch(api, emu)
        || heap::dispatch(api, emu)
        || loader::dispatch(api, emu)
        || memory::dispatch(api, emu)
        || string::dispatch(api, emu)
        || sync::dispatch(api, emu)
        || misc::dispatch(api, emu)
}

pub(crate) fn dispatch(addr: u64, api: &str, emu: &mut emu::Emu) -> bool {
    if try_dispatch(api, emu) {
        return true;
    }
    if nt_api::signature(api).is_some() {
        fallback::unimplemented(addr, api, emu);
        return true;
    }
    false
}

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if dispatch(addr, api, emu) {
        return String::new();
    }

    // Forwarded kernel32 APIs can reside in the ntdll map. Its native fallback
    // calls dispatch(), not this gateway, so this delegation cannot recurse.
    kernel32::gateway(addr, emu)
}
