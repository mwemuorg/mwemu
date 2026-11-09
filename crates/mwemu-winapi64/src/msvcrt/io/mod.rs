use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

#[path = "../fopen.rs"]
mod fopen;
pub use fopen::fopen;

#[path = "../fwrite.rs"]
mod fwrite;
pub use fwrite::fwrite;

#[path = "../fflush.rs"]
mod fflush;
pub use fflush::fflush;

#[path = "../fclose.rs"]
mod fclose;
pub use fclose::fclose;

#[path = "../printf.rs"]
mod printf;
pub use printf::printf;

#[path = "../sprintf.rs"]
mod sprintf;
pub use sprintf::sprintf;

#[path = "../swprintf.rs"]
mod swprintf;
pub use swprintf::swprintf;

#[path = "../freopen.rs"]
mod freopen;
pub use freopen::freopen;

#[path = "../_wrename.rs"]
mod _wrename;
pub use _wrename::_wrename;

#[path = "../_iob.rs"]
mod _iob;
pub use _iob::_iob;

#[path = "../_snprintf.rs"]
mod _snprintf;
pub use _snprintf::_snprintf;

#[path = "../sscanf.rs"]
mod sscanf;
pub use sscanf::sscanf;

#[path = "../fread.rs"]
mod fread;
pub use fread::fread;

#[path = "../fseek.rs"]
mod fseek;
pub use fseek::fseek;

#[path = "../ftell.rs"]
mod ftell;
pub use ftell::ftell;

#[path = "../feof.rs"]
mod feof;
pub use feof::feof;

#[path = "../fgets.rs"]
mod fgets;
pub use fgets::fgets;

#[path = "../fprintf.rs"]
mod fprintf;
pub use fprintf::fprintf;

#[path = "../vprintf.rs"]
mod vprintf;
pub use vprintf::vprintf;

#[path = "../vsprintf.rs"]
mod vsprintf;
pub use vsprintf::vsprintf;

#[path = "../vfprintf.rs"]
mod vfprintf;
pub use vfprintf::vfprintf;

#[path = "../fputs.rs"]
mod fputs;
pub use fputs::fputs;

#[path = "../puts.rs"]
mod puts;
pub use puts::puts;

#[path = "../putchar.rs"]
mod putchar;
pub use putchar::putchar;

#[path = "../getc.rs"]
mod getc;
pub use getc::getc;

#[path = "../_fileno.rs"]
mod _fileno;
pub use _fileno::_fileno;

#[path = "../_setmode.rs"]
mod _setmode;
pub use _setmode::_setmode;

#[path = "../_open.rs"]
mod _open;
pub use _open::_open;

#[path = "../_close.rs"]
mod _close;
pub use _close::_close;

#[path = "../_read.rs"]
mod _read;
pub use _read::_read;

#[path = "../_write.rs"]
mod _write;
pub use _write::_write;

#[path = "../__acrt_iob_func.rs"]
mod __acrt_iob_func;
pub use __acrt_iob_func::__acrt_iob_func;

#[path = "../__iob_func.rs"]
mod __iob_func;
pub use __iob_func::__iob_func;

#[path = "../__stdio_common_vfprintf.rs"]
mod __stdio_common_vfprintf;
pub use __stdio_common_vfprintf::__stdio_common_vfprintf;

#[path = "../__stdio_common_vsprintf_s.rs"]
mod __stdio_common_vsprintf_s;
pub use __stdio_common_vsprintf_s::__stdio_common_vsprintf_s;

#[path = "../_wfopen.rs"]
mod _wfopen;
pub use _wfopen::_wfopen;

#[path = "../setvbuf.rs"]
mod setvbuf;
pub use setvbuf::setvbuf;

#[path = "../vwprintf.rs"]
mod vwprintf;
pub use vwprintf::vwprintf;

#[path = "../__stdio_common_vsprintf.rs"]
mod __stdio_common_vsprintf;
pub use __stdio_common_vsprintf::__stdio_common_vsprintf;

#[path = "../__stdio_common_vsscanf.rs"]
mod __stdio_common_vsscanf;
pub use __stdio_common_vsscanf::__stdio_common_vsscanf;

#[path = "../_sprintf_l.rs"]
mod _sprintf_l;
pub use _sprintf_l::_sprintf_l;

#[path = "../wprintf.rs"]
mod wprintf;
pub use wprintf::wprintf;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "fopen" => msvcrt::fopen(emu),
        "fwrite" => msvcrt::fwrite(emu),
        "fflush" => msvcrt::fflush(emu),
        "fclose" => msvcrt::fclose(emu),
        "printf" => msvcrt::printf(emu),
        "sprintf" => msvcrt::sprintf(emu),
        "swprintf" => msvcrt::swprintf(emu),
        "freopen" => msvcrt::freopen(emu),
        "_wrename" => msvcrt::_wrename(emu),
        "_iob" => msvcrt::_iob(emu),
        "_snprintf" | "snprintf" | "sprintf_s" | "_snprintf_s" | "_sprintf_p" => {
            msvcrt::_snprintf(emu, api)
        }
        "sscanf" | "_sscanf_s_l" | "sscanf_s" => msvcrt::sscanf(emu, api),
        "fread" => msvcrt::fread(emu),
        "fseek" => msvcrt::fseek(emu),
        "ftell" => msvcrt::ftell(emu),
        "feof" => msvcrt::feof(emu),
        "fgets" => msvcrt::fgets(emu),
        "fprintf" => msvcrt::fprintf(emu),
        "vprintf" => msvcrt::vprintf(emu),
        "vsprintf" | "_vsnprintf" | "vsnprintf" => msvcrt::vsprintf(emu, api),
        "vfprintf" => msvcrt::vfprintf(emu),
        "fputs" => msvcrt::fputs(emu),
        "puts" => msvcrt::puts(emu),
        "putchar" | "putc" | "_putch" => msvcrt::putchar(emu, api),
        "getc" | "fgetc" | "_fgetchar" | "getchar" => msvcrt::getc(emu, api),
        "_fileno" => msvcrt::_fileno(emu),
        "_setmode" => msvcrt::_setmode(emu),
        "_open" | "_wopen" => msvcrt::_open(emu, api),
        "_close" => msvcrt::_close(emu),
        "_read" => msvcrt::_read(emu),
        "_write" => msvcrt::_write(emu),
        "__acrt_iob_func" => msvcrt::__acrt_iob_func(emu),
        "__iob_func" => msvcrt::__iob_func(emu),
        "__stdio_common_vfprintf" => msvcrt::__stdio_common_vfprintf(emu),
        "__stdio_common_vsprintf_s" => msvcrt::__stdio_common_vsprintf_s(emu),
        "_wfopen" => msvcrt::_wfopen(emu),
        "setvbuf" => msvcrt::setvbuf(emu),
        "vwprintf" => msvcrt::vwprintf(emu),
        "__stdio_common_vsprintf" => msvcrt::__stdio_common_vsprintf(emu),
        "__stdio_common_vsscanf" => msvcrt::__stdio_common_vsscanf(emu),
        "_sprintf_l" => msvcrt::_sprintf_l(emu),
        "wprintf" => msvcrt::wprintf(emu),
        _ => return false,
    }
    true
}
