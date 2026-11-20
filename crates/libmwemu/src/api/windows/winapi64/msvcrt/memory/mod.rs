use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

pub use crate::api::windows::libraries::msvcrt::memory::*;

mod __2_yapaxi_z;
pub use __2_yapaxi_z::__2_YAPAXI_Z;

mod __3_yaxpax_z;
pub use __3_yaxpax_z::__3_YAXPAX_Z;

mod _aligned_free;
pub use _aligned_free::_aligned_free;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "memcmp" => msvcrt::memcmp(emu),
        "memchr" => msvcrt::memchr(emu),
        "malloc" => msvcrt::malloc(emu),
        "free" => msvcrt::free(emu),
        "realloc" => msvcrt::realloc(emu),
        "??2@YAPAXI@Z" => msvcrt::__2_YAPAXI_Z(emu),
        "??3@YAXPAX@Z" => msvcrt::__3_YAXPAX_Z(emu),
        "calloc" => msvcrt::calloc(emu),
        "memset" => msvcrt::memset(emu),
        "memcpy" => msvcrt::memcpy(emu),
        "memmove" => msvcrt::memmove(emu),
        "_msize" => msvcrt::_msize(emu),
        "_aligned_malloc" => msvcrt::_aligned_malloc(emu),
        "_aligned_free" => msvcrt::_aligned_free(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
