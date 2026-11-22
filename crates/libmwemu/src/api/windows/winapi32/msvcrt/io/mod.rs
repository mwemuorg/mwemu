use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

pub use crate::api::windows::libraries::msvcrt::io::*;

mod _wrename;
pub use _wrename::_wrename;

mod __stdio_common_vfprintf;
pub use __stdio_common_vfprintf::__stdio_common_vfprintf;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "printf" => msvcrt::printf(emu),
        "_wrename" => msvcrt::_wrename(emu),
        "fprintf" => msvcrt::fprintf(emu),
        "vprintf" => msvcrt::vprintf(emu),
        "vfprintf" => msvcrt::vfprintf(emu),
        "puts" => msvcrt::puts(emu),
        "putchar" | "putc" | "_putch" => msvcrt::putchar(emu, api),
        "__stdio_common_vfprintf" => msvcrt::__stdio_common_vfprintf(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
