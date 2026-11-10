use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

#[path = "../__2_yapaxi_z.rs"]
mod __2_yapaxi_z;
pub use __2_yapaxi_z::__2_YAPAXI_Z;

#[path = "../__3_yaxpax_z.rs"]
mod __3_yaxpax_z;
pub use __3_yaxpax_z::__3_YAXPAX_Z;

#[path = "../calloc.rs"]
mod calloc;
pub use calloc::calloc;

#[path = "../memset.rs"]
mod memset;
pub use memset::memset;

#[path = "../memcpy.rs"]
mod memcpy;
pub use memcpy::memcpy;

#[path = "../memmove.rs"]
mod memmove;
pub use memmove::memmove;

#[path = "../memcmp.rs"]
mod memcmp;
pub use memcmp::memcmp;

#[path = "../memchr.rs"]
mod memchr;
pub use memchr::memchr;

#[path = "../_msize.rs"]
mod _msize;
pub use _msize::_msize;

#[path = "../_aligned_malloc.rs"]
mod _aligned_malloc;
pub use _aligned_malloc::_aligned_malloc;

#[path = "../_aligned_free.rs"]
mod _aligned_free;
pub use _aligned_free::_aligned_free;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "malloc" => msvcrt::malloc(emu),
        "free" => msvcrt::free(emu),
        "realloc" => msvcrt::realloc(emu),
        "??2@YAPAXI@Z" => msvcrt::__2_YAPAXI_Z(emu, addr),
        "??3@YAXPAX@Z" => msvcrt::__3_YAXPAX_Z(emu),
        "calloc" => msvcrt::calloc(emu),
        "memset" => msvcrt::memset(emu),
        "memcpy" => msvcrt::memcpy(emu),
        "memmove" => msvcrt::memmove(emu),
        "memcmp" => msvcrt::memcmp(emu),
        "memchr" => msvcrt::memchr(emu),
        "_msize" => msvcrt::_msize(emu),
        "_aligned_malloc" => msvcrt::_aligned_malloc(emu),
        "_aligned_free" => msvcrt::_aligned_free(emu, addr),
        _ => return false,
    }
    true
}
