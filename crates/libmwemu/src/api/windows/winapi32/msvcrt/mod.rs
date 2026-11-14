use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt;
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
use str_cmp_ca::StrCmpCA;
mod __p___argc;
use __p___argc::__p___argc;
mod __p___argv;
use __p___argv::__p___argv;
mod __p__acmdln;
use __p__acmdln::__p__acmdln;
mod __p__global_int;
use __p__global_int::__p__global_int;
mod __set_app_type;
use __set_app_type::__set_app_type;
mod _errno;
use _errno::_errno;
mod _initterm;
use _initterm::_initterm;
mod _initterm_e;
use _initterm_e::_initterm_e;
mod _lock;
use _lock::_lock;
mod _onexit;
use _onexit::_onexit;
mod exit;
use exit::exit;
mod fclose;
use fclose::fclose;
mod fflush;
use fflush::fflush;
mod fopen;
use fopen::fopen;
mod free;
use free::free;
mod fwrite;
use fwrite::fwrite;
mod malloc;
use malloc::malloc;
mod realloc;
use realloc::realloc;
mod strtok;
use strtok::strtok;

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
    if crt::dispatch(api, emu) {
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
