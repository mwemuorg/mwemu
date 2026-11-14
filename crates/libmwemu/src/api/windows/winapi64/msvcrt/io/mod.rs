use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

mod fopen;
pub use fopen::fopen;

mod fwrite;
pub use fwrite::fwrite;

mod fflush;
pub use fflush::fflush;

mod fclose;
pub use fclose::fclose;

mod printf;
pub use printf::printf;

mod sprintf;
pub use sprintf::sprintf;

mod swprintf;
pub use swprintf::swprintf;

mod freopen;
pub use freopen::freopen;

mod _wrename;
pub use _wrename::_wrename;

mod _iob;
pub use _iob::_iob;

mod _snprintf;
pub use _snprintf::_snprintf;

mod sscanf;
pub use sscanf::sscanf;

mod fread;
pub use fread::fread;

mod fseek;
pub use fseek::fseek;

mod ftell;
pub use ftell::ftell;

mod feof;
pub use feof::feof;

mod fgets;
pub use fgets::fgets;

mod fprintf;
pub use fprintf::fprintf;

mod vprintf;
pub use vprintf::vprintf;

mod vsprintf;
pub use vsprintf::vsprintf;

mod vfprintf;
pub use vfprintf::vfprintf;

mod fputs;
pub use fputs::fputs;

mod puts;
pub use puts::puts;

mod putchar;
pub use putchar::putchar;

mod getc;
pub use getc::getc;

mod _fileno;
pub use _fileno::_fileno;

mod _setmode;
pub use _setmode::_setmode;

mod _open;
pub use _open::_open;

mod _close;
pub use _close::_close;

mod _read;
pub use _read::_read;

mod _write;
pub use _write::_write;

mod __acrt_iob_func;
pub use __acrt_iob_func::__acrt_iob_func;

mod __iob_func;
pub use __iob_func::__iob_func;

mod __stdio_common_vfprintf;
pub use __stdio_common_vfprintf::__stdio_common_vfprintf;

mod __stdio_common_vsprintf_s;
pub use __stdio_common_vsprintf_s::__stdio_common_vsprintf_s;

mod _wfopen;
pub use _wfopen::_wfopen;

mod setvbuf;
pub use setvbuf::setvbuf;

mod vwprintf;
pub use vwprintf::vwprintf;

mod __stdio_common_vsprintf;
pub use __stdio_common_vsprintf::__stdio_common_vsprintf;

mod __stdio_common_vsscanf;
pub use __stdio_common_vsscanf::__stdio_common_vsscanf;

mod _sprintf_l;
pub use _sprintf_l::_sprintf_l;

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
