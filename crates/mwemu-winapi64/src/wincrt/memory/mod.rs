use crate::emu::Emu;
use crate::winapi::winapi64::wincrt;

#[path = "../__2_yapeax_k_z.rs"]
mod __2_yapeax_k_z;
pub use __2_yapeax_k_z::__2_YAPEAX_K_Z;

#[path = "../realloc.rs"]
mod realloc;
pub use realloc::realloc;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "malloc" => wincrt::malloc(emu),
        "calloc" => wincrt::calloc(emu),
        "free" => wincrt::free(emu),
        "realloc" => wincrt::realloc(emu),
        "memcpy" => wincrt::memcpy(emu),
        "memset" => wincrt::crt_memset(emu),
        "memmove" => wincrt::crt_memmove(emu),
        "memcmp" => wincrt::crt_memcmp(emu),
        "memchr" => wincrt::crt_memchr(emu),
        "_msize" => wincrt::crt_msize(emu),
        "_aligned_malloc" => wincrt::crt_aligned_malloc(emu),
        "_aligned_free" => wincrt::crt_aligned_free(emu),
        "??2@YAPEAX_K@Z"
        | "??3@YAXPEAX@Z"
        | "??_V@YAXPEAX@Z"
        | "??0exception@@QEAA@AEBV0@@Z"
        | "??1type_info@@UEAA@XZ" => wincrt::__2_YAPEAX_K_Z(emu, api),
        _ => return false,
    }
    true
}
