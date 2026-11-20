use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::{fallback, handles, memory as memory_api, nt_api, process, wait};
use crate::emu;
use crate::winapi::registry::native;
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
pub use misc::{NtQuerySystemInformation, RtlDecompressBuffer};
#[cfg(test)]
pub use sync::NtDelayExecution;

pub fn try_dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    let call = WinApi(Stdcall);
    if handles::dispatch_nt(api, emu, call)
        || memory_api::dispatch_nt(api, emu, call)
        || process::dispatch_nt(api, emu, call)
        || wait::dispatch_nt(api, emu, call)
    {
        return true;
    }
    let normalized = nt_api::normalize(api);
    let api = normalized.as_ref();
    native::dispatch(api, emu, call)
        || file::dispatch(api, emu)
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
    let api = api.rsplit('!').next().unwrap_or(&api);
    gateway_by_name(addr, api, emu)
}

pub(super) fn gateway_by_name(addr: u64, api: &str, emu: &mut emu::Emu) -> String {
    if dispatch(addr, api, emu) {
        return String::new();
    }

    // Forwarded kernel32 APIs can reside in the ntdll map. Its native fallback
    // calls dispatch(), not this gateway, so this delegation cannot recurse.
    kernel32::gateway_by_name(addr, api, emu)
}
