use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::winapi::winapi64::crt_stdio;

mod runtime;
pub use runtime::*;
mod memory;
pub use memory::*;
mod io;
pub use io::*;
mod strings;
pub use strings::*;
mod math;
pub use math::*;

mod __acrt_iob_func;
use __acrt_iob_func::__acrt_iob_func;
mod __p___argc;
use __p___argc::__p___argc;
mod __p___argv;
use __p___argv::__p___argv;
mod __p__commode;
use __p__commode::__p__commode;
mod __p__environ;
use __p__environ::__p__environ;
mod __p__fmode;
use __p__fmode::__p__fmode;
mod __stdio_common_vfprintf;
use __stdio_common_vfprintf::__stdio_common_vfprintf;
mod _configure_narrow_argv;
use _configure_narrow_argv::_configure_narrow_argv;
mod _crt_atexit;
use _crt_atexit::_crt_atexit;
mod _get_initial_narrow_environment;
use _get_initial_narrow_environment::_get_initial_narrow_environment;
mod _initialize_narrow_environment;
use _initialize_narrow_environment::_initialize_narrow_environment;
mod _initialize_onexit_table;
use _initialize_onexit_table::_initialize_onexit_table;
mod _register_onexit_function;
use _register_onexit_function::_register_onexit_function;
mod _set_app_type;
use _set_app_type::_set_app_type;
mod _set_new_mode;
use _set_new_mode::_set_new_mode;
mod abort;
use abort::abort;
mod calloc;
use calloc::calloc;
mod crt_abs;
use crt_abs::crt_abs;
mod crt_acmdln;
use crt_acmdln::crt_acmdln;
mod crt_aligned_free;
use crt_aligned_free::crt_aligned_free;
mod crt_aligned_malloc;
use crt_aligned_malloc::crt_aligned_malloc;
mod crt_atoi;
use crt_atoi::crt_atoi;
mod crt_beginthread;
use crt_beginthread::crt_beginthread;
mod crt_beginthreadex;
use crt_beginthreadex::crt_beginthreadex;
mod crt_clock;
use crt_clock::crt_clock;
mod crt_controlfp;
use crt_controlfp::crt_controlfp;
mod crt_controlfp_s;
use crt_controlfp_s::crt_controlfp_s;
mod crt_cxx_throw;
use crt_cxx_throw::crt_cxx_throw;
mod crt_dllonexit;
use crt_dllonexit::crt_dllonexit;
mod crt_endthread;
use crt_endthread::crt_endthread;
mod crt_except_handler;
use crt_except_handler::crt_except_handler;
mod crt_exit;
use crt_exit::crt_exit;
mod crt_format_stub;
use crt_format_stub::crt_format_stub;
mod crt_ftol;
use crt_ftol::crt_ftol;
mod crt_getenv;
use crt_getenv::crt_getenv;
mod crt_getmainargs;
use crt_getmainargs::crt_getmainargs;
mod crt_initterm;
use crt_initterm::crt_initterm;
mod crt_initterm_e;
use crt_initterm_e::crt_initterm_e;
mod crt_iob;
use crt_iob::crt_iob;
mod crt_isalnum;
use crt_isalnum::crt_isalnum;
mod crt_isalpha;
use crt_isalpha::crt_isalpha;
mod crt_isdigit;
use crt_isdigit::crt_isdigit;
mod crt_islower;
use crt_islower::crt_islower;
mod crt_isspace;
use crt_isspace::crt_isspace;
mod crt_isupper;
use crt_isupper::crt_isupper;
mod crt_itoa;
use crt_itoa::crt_itoa;
mod crt_localtime;
use crt_localtime::crt_localtime;
mod crt_lock_noop;
use crt_lock_noop::crt_lock_noop;
mod crt_memchr;
use crt_memchr::crt_memchr;
mod crt_memcmp;
use crt_memcmp::crt_memcmp;
mod crt_memmove;
use crt_memmove::crt_memmove;
mod crt_memset;
use crt_memset::crt_memset;
mod crt_msize;
use crt_msize::crt_msize;
mod crt_onexit;
use crt_onexit::crt_onexit;
mod crt_print_stub;
use crt_print_stub::crt_print_stub;
mod crt_rand;
use crt_rand::crt_rand;
mod crt_srand;
use crt_srand::crt_srand;
mod crt_strcat;
use crt_strcat::crt_strcat;
mod crt_strchr;
use crt_strchr::crt_strchr;
mod crt_strcmp;
use crt_strcmp::crt_strcmp;
mod crt_strcpy;
use crt_strcpy::crt_strcpy;
mod crt_strftime;
use crt_strftime::crt_strftime;
mod crt_stricmp;
use crt_stricmp::crt_stricmp;
mod crt_strncpy;
use crt_strncpy::crt_strncpy;
mod crt_strnicmp;
use crt_strnicmp::crt_strnicmp;
mod crt_strrchr;
use crt_strrchr::crt_strrchr;
mod crt_strstr;
use crt_strstr::crt_strstr;
mod crt_strtok;
use crt_strtok::crt_strtok;
mod crt_strtol;
use crt_strtol::crt_strtol;
mod crt_strtoul;
use crt_strtoul::crt_strtoul;
mod crt_time;
use crt_time::crt_time;
mod crt_tolower;
use crt_tolower::crt_tolower;
mod crt_toupper;
use crt_toupper::crt_toupper;
mod crt_towlower;
use crt_towlower::crt_towlower;
mod crt_towupper;
use crt_towupper::crt_towupper;
mod crt_wcscat;
use crt_wcscat::crt_wcscat;
mod crt_wcscmp;
use crt_wcscmp::crt_wcscmp;
mod crt_wcscpy;
use crt_wcscpy::crt_wcscpy;
mod crt_wcsicmp;
use crt_wcsicmp::crt_wcsicmp;
mod crt_wcslen;
use crt_wcslen::crt_wcslen;
mod crt_wcsncpy;
use crt_wcsncpy::crt_wcsncpy;
mod crt_wtoi;
use crt_wtoi::crt_wtoi;
mod crt_xcpt_filter;
use crt_xcpt_filter::crt_xcpt_filter;
mod free;
use free::free;
mod malloc;
use malloc::malloc;
mod memcpy;
use memcpy::memcpy;
mod puts;
use puts::puts;
mod set_invalid_parameter_handler;
use set_invalid_parameter_handler::set_invalid_parameter_handler;
mod setvbuf;
use setvbuf::setvbuf;
mod signal;
use signal::signal;
mod strlen;
use strlen::strlen;
mod strncmp;
use strncmp::strncmp;

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
pub(super) fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    buffers::allocate(emu, size, 16, 0, BufferKind::Crt)
}

/// Classification of a pointer allocated by `allocate_memory`.
enum AllocKind {
    Arena { size: usize },
    Map { base: u64, size: usize },
    Invalid,
}

fn classify(emu: &emu::Emu, addr: u64) -> AllocKind {
    if let Some(heap) = emu.heap_arenas.first()
        && let Some(size) = heap.allocation_size(addr)
    {
        return AllocKind::Arena { size };
    }

    match emu.maps.get_mem_by_addr(addr) {
        Some(mem) if mem.get_base() == addr && mem.get_name().starts_with("alloc_") => {
            AllocKind::Map {
                base: addr,
                size: mem.size(),
            }
        }
        _ => AllocKind::Invalid,
    }
}

/// Releases a pointer allocated by `allocate_memory` (or any alloc_ map).
pub(super) fn release(emu: &mut emu::Emu, addr: u64) {
    buffers::free(emu, addr, BufferKind::Crt);
}

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    gateway_by_name(api, emu)
}

pub fn gateway_by_name(api: &str, emu: &mut emu::Emu) -> String {
    if crt_stdio::gateway(api, emu) {
        return String::new();
    }
    if crt::dispatch(api, emu) {
        return String::new();
    }
    if runtime::dispatch(api, emu)
        || memory::dispatch(api, emu)
        || io::dispatch(api, emu)
        || strings::dispatch(api, emu)
        || math::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(0u64, api, emu)
}

/*
extern "C" char** __cdecl _get_initial_narrow_environment()
{
    return common_get_initial_environment<char>();
}
*/
// char*** CDECL __p___argv(void) { return &MSVCRT___argv; }
// int* CDECL __p___argc(void) { return &MSVCRT___argc; }
/*
FILE * CDECL __acrt_iob_func(int index)
{
    return &__iob_func()[index];
}
*/

/*
_ACRTIMP int __cdecl __stdio_common_vfprintf(unsigned __int64,FILE*,const char*,_locale_t,__ms_va_list);
*/
fn parse_format_specifiers(fmt: &str) -> Vec<&str> {
    let mut specs = Vec::new();
    let mut chars = fmt.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%'
            && let Some(next) = chars.next()
            && next != '%'
        {
            // Skip %% (literal %)
            specs.push(match next {
                'd' | 'i' => "int",
                'x' | 'X' => "hex",
                'p' => "ptr",
                's' => "str",
                // Add other format specifiers as needed
                _ => "unknown",
            });
        }
    }
    specs
}

/*
int _crt_atexit(
    _PVFV const function
)
*/
// --- String functions ---

// --- Wide string functions ---

// --- Character classification ---

// --- Memory functions ---

// --- I/O stubs ---

fn crt_fopen(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let mode_ptr = emu.regs().rdx;
    let path = emu.maps.read_string(path_ptr);
    let mode = emu.maps.read_string(mode_ptr);
    let handle = helper::handler_create(&path);
    log_red!(emu, "wincrt!fopen '{}' '{}' => 0x{:x}", path, mode, handle);
    emu.regs_mut().rax = handle;
}

fn crt_fclose(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    let name = helper::handler_get_uri(fp);
    helper::handler_close(fp);
    log_red!(emu, "wincrt!fclose 0x{:x} '{}'", fp, name);
    emu.regs_mut().rax = 0;
}

fn crt_fread(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let fp = emu.regs().r9;
    let name = helper::handler_get_uri(fp);
    log_red!(
        emu,
        "wincrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} '{}'",
        buf,
        size,
        count,
        fp,
        name
    );
    helper::handler_set_eof(fp);
    emu.regs_mut().rax = 0;
}

fn crt_fwrite(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let fp = emu.regs().r9;
    let name = helper::handler_get_uri(fp);
    let total = size * count;
    log_red!(
        emu,
        "wincrt!fwrite buf:0x{:x} size:{} count:{} fp:0x{:x} '{}' total:{}",
        buf,
        size,
        count,
        fp,
        name,
        total
    );
    emu.regs_mut().rax = count;
}

fn crt_fflush(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    log_red!(emu, "wincrt!fflush 0x{:x}", fp);
    emu.regs_mut().rax = 0;
}

fn crt_fseek(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    let offset = emu.regs().rdx as i64;
    let origin = emu.regs().r8 as u32;
    let new_pos = helper::handler_seek(fp, offset, origin);
    log_red!(
        emu,
        "wincrt!fseek fp:0x{:x} offset:{} origin:{} => pos:{}",
        fp,
        offset,
        origin,
        new_pos
    );
    emu.regs_mut().rax = if new_pos >= 0 { 0 } else { u64::MAX };
}

fn crt_ftell(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    let pos = helper::handler_tell(fp);
    log_red!(emu, "wincrt!ftell 0x{:x} => {}", fp, pos);
    emu.regs_mut().rax = pos as u64;
}

fn crt_feof(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    let is_eof = helper::handler_eof(fp);
    log_red!(emu, "wincrt!feof 0x{:x} => {}", fp, is_eof as u32);
    emu.regs_mut().rax = is_eof as u64;
}

// --- Process/Environment ---

fn crt_errno(emu: &mut emu::Emu) {
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}

fn crt_errno_stub(emu: &mut emu::Emu, api: &str) {
    if api == "_set_errno" {
        let value = emu.regs().rcx as u32;
        log_red!(emu, "wincrt!_set_errno {}", value);
        emu.regs_mut().rax = 0;
    } else {
        let out_ptr = emu.regs().rcx;
        if out_ptr != 0 {
            emu.maps.write_dword(out_ptr, 0);
        }
        log_red!(emu, "wincrt!_get_errno => 0");
        emu.regs_mut().rax = 0;
    }
}

// --- Math ---

// --- Exception handling ---

// --- Misc ---
