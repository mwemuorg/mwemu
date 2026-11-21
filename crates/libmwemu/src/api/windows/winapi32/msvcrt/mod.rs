use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt::{self, Runtime::Msvcrt};
use crate::winapi::common::crt_stdio;
use crate::winapi::winapi32::kernel32;
//use crate::endpoint;

mod io;
mod math;
mod memory;
mod runtime;
mod strings;

pub use io::*;
pub use math::*;
pub use memory::*;
pub use runtime::*;
pub use strings::*;

mod str_cmp_ca;
use crate::api::windows::libraries::msvcrt::memory::free;
use crate::api::windows::libraries::msvcrt::memory::malloc;
use crate::api::windows::libraries::msvcrt::memory::realloc;
use crate::api::windows::libraries::msvcrt::runtime::_lock;
use str_cmp_ca::StrCmpCA;
/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    buffers::allocate(emu, size, 16, 0, BufferKind::Crt)
}

/// Releases a pointer allocated by `allocate_memory` (or any alloc_ map).
fn release(emu: &mut emu::Emu, addr: u64) {
    buffers::free(emu, addr, BufferKind::Crt);
}

// msvcrt is an exception and these functions dont have to compensate the stack.

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    gateway_by_name(api, addr, emu)
}

pub fn gateway_by_name(api: &str, addr: u32, emu: &mut emu::Emu) -> String {
    if crt::dispatch(api, emu)
        || crt::dispatch_runtime(api, emu, Msvcrt)
        || crt_stdio::gateway(api, emu, Msvcrt)
    {
        return String::new();
    }
    if runtime::dispatch(api, addr, emu)
        || strings::dispatch(api, emu)
        || io::dispatch(api, emu)
        || memory::dispatch(api, addr, emu)
        || math::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}
