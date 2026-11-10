use crate::api::windows::common::{fallback, memory as memory_api, memory::Call, nt_api};
use crate::emu;
use crate::winapi::winapi32::kernel32;
use crate::windows::constants::errors::*;

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
pub use misc::RtlDecompressBuffer;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if memory_api::dispatch_nt(api, emu, Call::Api) {
        return String::new();
    }
    // Normalize all Zw aliases, then give real implementations priority.
    let normalized = nt_api::normalize(api);
    let dispatch_name = normalized.as_ref();
    if file::dispatch(dispatch_name, emu)
        || heap::dispatch(dispatch_name, emu)
        || loader::dispatch(dispatch_name, emu)
        || memory::dispatch(dispatch_name, emu)
        || string::dispatch(dispatch_name, emu)
        || sync::dispatch(dispatch_name, emu)
        || misc::dispatch(dispatch_name, emu)
    {
        return String::new();
    }

    // A known NT signature has a defined failure result even in strict mode.
    if let Some(words) = nt_api::signature(dispatch_name) {
        return fallback::return_value(addr, api, words, STATUS_NOT_IMPLEMENTED, emu);
    }
    fallback::unimplemented(addr, api, emu)
}
