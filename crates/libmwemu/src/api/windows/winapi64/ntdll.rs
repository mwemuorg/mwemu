use crate::emu;
use crate::winapi::winapi64::kernel32;

mod file;
mod heap;
mod loader;
mod memory;
mod misc;
mod string;
mod sync;

pub use heap::{RtlAllocateHeap, RtlReAllocateHeap};

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if file::dispatch(api, emu)
        || heap::dispatch(api, emu)
        || loader::dispatch(api, emu)
        || memory::dispatch(api, emu)
        || string::dispatch(api, emu)
        || sync::dispatch(api, emu)
        || misc::dispatch(api, emu)
    {
        return String::new();
    }

    // Not an ntdll-native API. Windows forwards many kernel32/kernelbase
    // functions to ntdll-resident implementations (e.g. EnterCriticalSection ->
    // ntdll!RtlEnterCriticalSection), so an import can land in the ntdll map yet
    // be owned by our kernel32 gateway. Delegate there before giving up — it
    // mirrors kernelbase's fallback and only runs for APIs ntdll didn't handle,
    // so it can't regress anything. (kernel32's gateway never calls back into
    // ntdll, so there's no recursion; it also owns the unimplemented panic /
    // skip-unimplemented handling.)
    kernel32::gateway(addr, emu)
}
