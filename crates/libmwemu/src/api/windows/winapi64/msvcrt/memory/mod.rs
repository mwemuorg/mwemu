use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

mod malloc;
pub use malloc::malloc;

mod free;
pub use free::free;

mod realloc;
pub use realloc::realloc;

mod __2_yapaxi_z;
pub use __2_yapaxi_z::__2_YAPAXI_Z;

mod __3_yaxpax_z;
pub use __3_yaxpax_z::__3_YAXPAX_Z;

mod calloc;
pub use calloc::calloc;

mod memset;
pub use memset::memset;

mod memcpy;
pub use memcpy::memcpy;

mod memmove;
pub use memmove::memmove;

mod memcmp;
pub use memcmp::memcmp;

mod memchr;
pub use memchr::memchr;

mod _msize;
pub use _msize::_msize;

mod _aligned_malloc;
pub use _aligned_malloc::_aligned_malloc;

mod _aligned_free;
pub use _aligned_free::_aligned_free;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "malloc" => msvcrt::malloc(emu),
        "free" => msvcrt::free(emu),
        "realloc" => msvcrt::realloc(emu),
        "??2@YAPAXI@Z" => msvcrt::__2_YAPAXI_Z(emu),
        "??3@YAXPAX@Z" => msvcrt::__3_YAXPAX_Z(emu),
        "calloc" => msvcrt::calloc(emu),
        "memset" => msvcrt::memset(emu),
        "memcpy" => msvcrt::memcpy(emu),
        "memmove" => msvcrt::memmove(emu),
        "memcmp" => msvcrt::memcmp(emu),
        "memchr" => msvcrt::memchr(emu),
        "_msize" => msvcrt::_msize(emu),
        "_aligned_malloc" => msvcrt::_aligned_malloc(emu),
        "_aligned_free" => msvcrt::_aligned_free(emu),
        _ => return false,
    }
    true
}
