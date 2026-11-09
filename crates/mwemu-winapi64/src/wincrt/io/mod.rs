use crate::emu::Emu;
use crate::winapi::winapi64::wincrt;

#[path = "../_fgetchar.rs"]
mod _fgetchar;
pub use _fgetchar::_fgetchar;

#[path = "../_open.rs"]
mod _open;
pub use _open::_open;

#[path = "../_snprintf_s.rs"]
mod _snprintf_s;
pub use _snprintf_s::_snprintf_s;

#[path = "../_sscanf_s_l.rs"]
mod _sscanf_s_l;
pub use _sscanf_s_l::_sscanf_s_l;

#[path = "../_wrename.rs"]
mod _wrename;
pub use _wrename::_wrename;

#[path = "../putc.rs"]
mod putc;
pub use putc::putc;

#[path = "../swprintf.rs"]
mod swprintf;
pub use swprintf::swprintf;

#[path = "../_sprintf_p.rs"]
mod _sprintf_p;
pub use _sprintf_p::_sprintf_p;

#[path = "../sscanf_s.rs"]
mod sscanf_s;
pub use sscanf_s::sscanf_s;

#[path = "../fgets.rs"]
mod fgets;
pub use fgets::fgets;

#[path = "../fputs.rs"]
mod fputs;
pub use fputs::fputs;

#[path = "../getchar.rs"]
mod getchar;
pub use getchar::getchar;

#[path = "../_close.rs"]
mod _close;
pub use _close::_close;

#[path = "../_read.rs"]
mod _read;
pub use _read::_read;

#[path = "../_write.rs"]
mod _write;
pub use _write::_write;

#[path = "../_wopen.rs"]
mod _wopen;
pub use _wopen::_wopen;

#[path = "../_fileno.rs"]
mod _fileno;
pub use _fileno::_fileno;

#[path = "../_setmode.rs"]
mod _setmode;
pub use _setmode::_setmode;

#[path = "../_putch.rs"]
mod _putch;
pub use _putch::_putch;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "__acrt_iob_func" => wincrt::__acrt_iob_func(emu),
        "setvbuf" => wincrt::setvbuf(emu),
        "__stdio_common_vfprintf" => wincrt::__stdio_common_vfprintf(emu),
        "puts" => wincrt::puts(emu),
        "fprintf"
        | "sprintf"
        | "snprintf"
        | "sscanf"
        | "_snprintf"
        | "_sprintf_l"
        | "__stdio_common_vsprintf"
        | "__stdio_common_vsscanf"
        | "__stdio_common_vsprintf_s" => wincrt::crt_format_stub(emu, api),
        "printf" | "vprintf" | "vfprintf" | "vsprintf" | "vsnprintf" | "_vsnprintf" | "wprintf"
        | "vwprintf" => wincrt::crt_print_stub(emu, api),
        "_iob" | "__iob_func" => wincrt::crt_iob(emu),
        "_fgetchar" | "fgetc" | "getc" => wincrt::_fgetchar(emu, api),
        "_open" | "freopen" => wincrt::_open(emu, api),
        "_snprintf_s" | "sprintf_s" => wincrt::_snprintf_s(emu, api),
        "_sscanf_s_l" => wincrt::_sscanf_s_l(emu),
        "_wrename" => wincrt::_wrename(emu),
        "putc" | "putchar" => wincrt::putc(emu, api),
        "swprintf" => wincrt::swprintf(emu),
        "_sprintf_p" => wincrt::_sprintf_p(emu),
        "sscanf_s" => wincrt::sscanf_s(emu),
        "fgets" => wincrt::fgets(emu),
        "fputs" => wincrt::fputs(emu),
        "getchar" => wincrt::getchar(emu),
        "_close" => wincrt::_close(emu),
        "_read" => wincrt::_read(emu),
        "_write" => wincrt::_write(emu),
        "_wopen" => wincrt::_wopen(emu),
        "_fileno" => wincrt::_fileno(emu),
        "_setmode" => wincrt::_setmode(emu),
        "_putch" => wincrt::_putch(emu),
        _ => return false,
    }
    true
}
