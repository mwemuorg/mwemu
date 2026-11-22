use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::{
    fallback, handles, memory as memory_api, nt_api, process, seh4, wait,
};
use crate::emu;
use crate::winapi::registry::native;
use crate::winapi::winapi32::kernel32;

mod file;
mod heap;
mod loader;
mod memory;
mod misc;
mod string;
mod sync;

pub use misc::RtlUnwind;

#[cfg(test)]
pub use crate::winapi::common::ntdll::RtlDecompressBuffer;
#[cfg(test)]
pub use heap::{RtlAllocateHeap, RtlReAllocateHeap};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.rsplit('!').next().unwrap_or(&api);
    gateway_by_name(addr, api, emu)
}

pub(super) fn gateway_by_name(addr: u32, api: &str, emu: &mut emu::Emu) -> String {
    let call = WinApi(Stdcall);
    if seh4::dispatch(emu, "ntdll", api)
        || handles::dispatch_nt(api, emu, call)
        || memory_api::dispatch_nt(api, emu, call)
        || process::dispatch_nt(api, emu, call)
        || wait::dispatch_nt(api, emu, call)
    {
        return String::new();
    }
    // Normalize all Zw aliases, then give real implementations priority.
    let normalized = nt_api::normalize(api);
    let dispatch_name = normalized.as_ref();
    if native::dispatch(dispatch_name, emu, call)
        || file::dispatch(dispatch_name, emu)
        || heap::dispatch(dispatch_name, emu)
        || loader::dispatch(dispatch_name, emu)
        || memory::dispatch(dispatch_name, emu)
        || string::dispatch(dispatch_name, emu)
        || sync::dispatch(dispatch_name, emu)
        || misc::dispatch(dispatch_name, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}
