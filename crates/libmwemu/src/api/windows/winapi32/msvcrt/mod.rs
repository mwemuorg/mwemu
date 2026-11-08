use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::kernel32;
//use crate::endpoint;

mod __setusermatherr;
pub use __setusermatherr::__setusermatherr;
mod _controlfp;
pub use _controlfp::_controlfp;
mod __getmainargs;
pub use __getmainargs::__getmainargs;
mod _xcpt_filter;
pub use _xcpt_filter::_XcptFilter;
mod __dllonexit;
pub use __dllonexit::__dllonexit;
mod _except_handler3;
pub use _except_handler3::_except_handler3;
mod _eh_prolog;
pub use _eh_prolog::_EH_prolog;
mod _cxx_throw_exception;
pub use _cxx_throw_exception::_CxxThrowException;
mod _ftol;
pub use _ftol::_ftol;
mod time;
pub use time::time;
mod localtime;
pub use localtime::localtime;
mod strftime;
pub use strftime::strftime;
mod srand;
pub use srand::srand;
mod rand;
pub use rand::rand;
mod atoi;
pub use atoi::atoi;
mod _itoa;
pub use _itoa::_itoa;
mod getenv;
pub use getenv::getenv;
mod printf;
pub use printf::printf;
mod sprintf;
pub use sprintf::sprintf;
mod swprintf;
pub use swprintf::swprintf;
mod strncmp;
pub use strncmp::strncmp;
mod toupper;
pub use toupper::toupper;
mod tolower;
pub use tolower::tolower;
mod wcslen;
pub use wcslen::wcslen;
mod wcscpy;
pub use wcscpy::wcscpy;
mod wcscat;
pub use wcscat::wcscat;
mod wcscmp;
pub use wcscmp::wcscmp;
mod freopen;
pub use freopen::freopen;
mod _wrename;
pub use _wrename::_wrename;
mod _wsystem;
pub use _wsystem::_wsystem;
mod _iob;
pub use _iob::_iob;
mod __2_yapaxi_z;
pub use __2_yapaxi_z::__2_YAPAXI_Z;
mod __3_yaxpax_z;
pub use __3_yaxpax_z::__3_YAXPAX_Z;
mod __1type_info__uae_xz;
pub use __1type_info__uae_xz::__1type_info__UAE_XZ;
mod __0exception__qae_abv0__z;
pub use __0exception__qae_abv0__z::__0exception__QAE_ABV0__Z;
mod strcmp;
pub use strcmp::strcmp;
mod strcpy;
pub use strcpy::strcpy;
mod strncpy;
pub use strncpy::strncpy;
mod strcat;
pub use strcat::strcat;
mod strncat;
pub use strncat::strncat;
mod strstr;
pub use strstr::strstr;
mod strchr;
pub use strchr::strchr;
mod strrchr;
pub use strrchr::strrchr;
mod _stricmp;
pub use _stricmp::_stricmp;
mod _strnicmp;
pub use _strnicmp::_strnicmp;
mod strlen;
pub use strlen::strlen;
mod strtol;
pub use strtol::strtol;
mod strtoul;
pub use strtoul::strtoul;
mod atol;
pub use atol::atol;
mod _snprintf;
pub use _snprintf::_snprintf;
mod sscanf;
pub use sscanf::sscanf;
mod wcsncpy;
pub use wcsncpy::wcsncpy;
mod wcsncat;
pub use wcsncat::wcsncat;
mod _wcsicmp;
pub use _wcsicmp::_wcsicmp;
mod _wcsnicmp;
pub use _wcsnicmp::_wcsnicmp;
mod _wtoi;
pub use _wtoi::_wtoi;
mod _wtol;
pub use _wtol::_wtol;
mod towupper;
pub use towupper::towupper;
mod towlower;
pub use towlower::towlower;
mod mbstowcs;
pub use mbstowcs::mbstowcs;
mod wcstombs;
pub use wcstombs::wcstombs;
mod isalpha;
pub use isalpha::isalpha;
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
mod _cexit;
pub use _cexit::_cexit;
mod _beginthread;
pub use _beginthread::_beginthread;
mod _beginthreadex;
pub use _beginthreadex::_beginthreadex;
mod _endthread;
pub use _endthread::_endthread;
mod system;
pub use system::system;
mod _putenv;
pub use _putenv::_putenv;
mod __wgetmainargs;
pub use __wgetmainargs::__wgetmainargs;
mod _wcmdln;
pub use _wcmdln::_wcmdln;
mod _except_handler4;
pub use _except_handler4::_except_handler4;
mod _set_se_translator;
pub use _set_se_translator::_set_se_translator;
mod _eh_prolog3;
pub use _eh_prolog3::_EH_prolog3;
mod _ftol2;
pub use _ftol2::_ftol2;
mod abs;
pub use abs::abs;
mod ceil;
pub use ceil::ceil;
mod floor;
pub use floor::floor;
mod sqrt;
pub use sqrt::sqrt;
mod pow;
pub use pow::pow;
mod log_api;
pub use log_api::log;
mod log10;
pub use log10::log10;
mod exp;
pub use exp::exp;
mod sin;
pub use sin::sin;
mod cos;
pub use cos::cos;
mod tan;
pub use tan::tan;
mod atan;
pub use atan::atan;
mod atan2;
pub use atan2::atan2;
mod fabs;
pub use fabs::fabs;
mod fmod;
pub use fmod::fmod;
mod clock;
pub use clock::clock;
mod _time64;
pub use _time64::_time64;
mod _controlfp_s;
pub use _controlfp_s::_controlfp_s;
mod _unlock;
pub use _unlock::_unlock;
mod _configthreadlocale;
pub use _configthreadlocale::_configthreadlocale;
mod _amsg_exit;
pub use _amsg_exit::_amsg_exit;
mod _crt_atexit;
pub use _crt_atexit::_crt_atexit;
mod _initialize_onexit_table;
pub use _initialize_onexit_table::_initialize_onexit_table;
mod __cpp_xcpt_filter;
pub use __cpp_xcpt_filter::__CppXcptFilter;
mod __acrt_iob_func;
pub use __acrt_iob_func::__acrt_iob_func;
mod __iob_func;
pub use __iob_func::__iob_func;
mod __p__environ;
pub use __p__environ::__p__environ;
mod __stdio_common_vfprintf;
pub use __stdio_common_vfprintf::__stdio_common_vfprintf;
mod __stdio_common_vsprintf_s;
pub use __stdio_common_vsprintf_s::__stdio_common_vsprintf_s;
mod _configure_narrow_argv;
pub use _configure_narrow_argv::_configure_narrow_argv;
mod _set_errno;
pub use _set_errno::_set_errno;
mod _get_errno;
pub use _get_errno::_get_errno;
mod _get_initial_narrow_environment;
pub use _get_initial_narrow_environment::_get_initial_narrow_environment;
mod _initialize_narrow_environment;
pub use _initialize_narrow_environment::_initialize_narrow_environment;
mod _localtime64;
pub use _localtime64::_localtime64;
mod _set_app_type;
pub use _set_app_type::_set_app_type;
mod _set_invalid_parameter_handler;
pub use _set_invalid_parameter_handler::_set_invalid_parameter_handler;
mod _set_thread_local_invalid_parameter_handler;
pub use _set_thread_local_invalid_parameter_handler::_set_thread_local_invalid_parameter_handler;
mod _strcmpi;
pub use _strcmpi::_strcmpi;
mod _time32;
pub use _time32::_time32;
mod _wfopen;
pub use _wfopen::_wfopen;
mod abort;
pub use abort::abort;
mod ptr;
pub use ptr::ptr;
mod setvbuf;
pub use setvbuf::setvbuf;
mod signal;
pub use signal::signal;
mod str;
pub use str::str;
mod strtok_s;
pub use strtok_s::strtok_s;
mod vwprintf;
pub use vwprintf::vwprintf;
mod wcsftime;
pub use wcsftime::wcsftime;
mod __lconv_init;
pub use __lconv_init::__lconv_init;
mod __setlc_active;
pub use __setlc_active::__setlc_active;
mod _set_error_mode;
pub use _set_error_mode::_set_error_mode;
mod _set_fmode;
pub use _set_fmode::_set_fmode;
mod __stdio_common_vsprintf;
pub use __stdio_common_vsprintf::__stdio_common_vsprintf;
mod __stdio_common_vsscanf;
pub use __stdio_common_vsscanf::__stdio_common_vsscanf;
mod _sprintf_l;
pub use _sprintf_l::_sprintf_l;
mod wprintf;
pub use wprintf::wprintf;
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

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    if size < LARGE_ALLOC_THRESHOLD {
        let heap_manage = emu.heap_mut();
        return heap_manage.allocate(size as usize);
    }

    let allocation = emu.maps.alloc(size)?;
    emu.maps
        .create_map(
            &format!("alloc_{:x}", allocation),
            allocation,
            size,
            Permission::READ_WRITE,
        )
        .ok()?;
    Some(allocation)
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
fn release(emu: &mut emu::Emu, addr: u64) {
    match classify(emu, addr) {
        AllocKind::Arena { .. } => {
            if let Some(heap) = emu.heap_arenas.first_mut() {
                heap.free(addr);
            }
        }
        AllocKind::Map { base, .. } => emu.maps.dealloc(base),
        AllocKind::Invalid => {}
    }
}

// msvcrt is an exception and these functions dont have to compensate the stack.

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "_initterm_e" => _initterm_e(emu),
        "_initterm" => _initterm(emu),
        "StrCmpCA" => StrCmpCA(emu),
        "fopen" => fopen(emu),
        "fwrite" => fwrite(emu),
        "fflush" => fflush(emu),
        "fclose" => fclose(emu),
        "__p___argv" => __p___argv(emu),
        "__p___argc" => __p___argc(emu),
        "malloc" => malloc(emu),
        "_onexit" => _onexit(emu),
        "_lock" => _lock(emu),
        "free" => free(emu),
        "realloc" => realloc(emu),
        "strtok" => strtok(emu),
        "exit" | "_exit" => exit(emu),
        "__set_app_type" => __set_app_type(emu),
        "_errno" => _errno(emu),
        "__p__fmode" => __p__global_int(emu, "__p__fmode"),
        "__p__commode" => __p__global_int(emu, "__p__commode"),
        "_adjust_fdiv" => __p__global_int(emu, "_adjust_fdiv"),
        "__setusermatherr" => __setusermatherr(emu),
        "_controlfp" => _controlfp(emu),
        "__getmainargs" => __getmainargs(emu),
        "_acmdln" | "__p__acmdln" => __p__acmdln(emu),
        "_XcptFilter" => _XcptFilter(emu),
        "__dllonexit" => __dllonexit(emu),
        "_except_handler3" | "__CxxFrameHandler" => _except_handler3(emu, api),
        "_EH_prolog" => _EH_prolog(emu),
        "_CxxThrowException" => _CxxThrowException(emu),
        "_ftol" => _ftol(emu),
        "time" => time(emu),
        "localtime" => localtime(emu),
        "strftime" => strftime(emu),
        "srand" => srand(emu),
        "rand" => rand(emu),
        "atoi" => atoi(emu),
        "_itoa" | "_itow" => _itoa(emu, api),
        "getenv" | "_wgetenv" => getenv(emu, api),
        "printf" => printf(emu),
        "sprintf" => sprintf(emu),
        "swprintf" => swprintf(emu),
        "strncmp" => strncmp(emu),
        "toupper" => toupper(emu),
        "tolower" => tolower(emu),
        "wcslen" => wcslen(emu),
        "wcscpy" => wcscpy(emu),
        "wcscat" => wcscat(emu),
        "wcscmp" => wcscmp(emu),
        "freopen" => freopen(emu),
        "_wrename" => _wrename(emu),
        "_wsystem" => _wsystem(emu),
        "_iob" => _iob(emu),
        "??2@YAPAXI@Z" => __2_YAPAXI_Z(emu, addr),
        "??3@YAXPAX@Z" => __3_YAXPAX_Z(emu),
        "??1type_info@@UAE@XZ" => __1type_info__UAE_XZ(emu),
        "??0exception@@QAE@ABV0@@Z" => __0exception__QAE_ABV0__Z(emu),

        // --- String functions ---
        "strcmp" => strcmp(emu),
        "strcpy" => strcpy(emu),
        "strncpy" => strncpy(emu),
        "strcat" => strcat(emu),
        "strncat" => strncat(emu),
        "strstr" => strstr(emu),
        "strchr" => strchr(emu),
        "strrchr" => strrchr(emu),
        "_stricmp" => _stricmp(emu),
        "_strnicmp" => _strnicmp(emu),
        "strlen" => strlen(emu),
        "strtol" => strtol(emu),
        "strtoul" => strtoul(emu),
        "atol" => atol(emu),
        "_snprintf" | "snprintf" | "sprintf_s" | "_snprintf_s" | "_sprintf_p" => {
            _snprintf(emu, api)
        }
        "sscanf" | "_sscanf_s_l" | "sscanf_s" => sscanf(emu, api),
        "wcsncpy" => wcsncpy(emu),
        "wcsncat" => wcsncat(emu),
        "_wcsicmp" => _wcsicmp(emu),
        "_wcsnicmp" => _wcsnicmp(emu),
        "_wtoi" => _wtoi(emu),
        "_wtol" => _wtol(emu),
        "towupper" => towupper(emu),
        "towlower" => towlower(emu),
        "mbstowcs" | "_mbstowcs_l" => mbstowcs(emu, api),
        "wcstombs" | "_wcstombs_l" => wcstombs(emu, api),
        "isalpha" | "isdigit" | "isspace" | "isalnum" | "ispunct" | "isupper" | "islower"
        | "isxdigit" => isalpha(emu, api),

        // --- Memory functions ---
        "calloc" => calloc(emu),
        "memset" => memset(emu),
        "memcpy" => memcpy(emu),
        "memmove" => memmove(emu),
        "memcmp" => memcmp(emu),
        "memchr" => memchr(emu),
        "_msize" => _msize(emu),
        "_aligned_malloc" => _aligned_malloc(emu),
        "_aligned_free" => _aligned_free(emu, addr),

        // --- I/O functions ---
        "fread" => fread(emu),
        "fseek" => fseek(emu),
        "ftell" => ftell(emu),
        "feof" => feof(emu),
        "fgets" => fgets(emu),
        "fprintf" => fprintf(emu),
        "vprintf" => vprintf(emu),
        "vsprintf" | "_vsnprintf" | "vsnprintf" => vsprintf(emu, api),
        "vfprintf" => vfprintf(emu),
        "fputs" => fputs(emu),
        "puts" => puts(emu),
        "putchar" | "putc" | "_putch" => putchar(emu, api),
        "getc" | "fgetc" | "_fgetchar" | "getchar" => getc(emu, api),
        "_fileno" => _fileno(emu),
        "_setmode" => _setmode(emu),
        "_open" | "_wopen" => _open(emu, api),
        "_close" => _close(emu),
        "_read" => _read(emu),
        "_write" => _write(emu),

        // --- Process/Environment ---
        "_cexit" | "_c_exit" => _cexit(emu, api),
        "_beginthread" => _beginthread(emu),
        "_beginthreadex" => _beginthreadex(emu),
        "_endthread" | "_endthreadex" => _endthread(emu, api),
        "system" => system(emu),
        "_putenv" | "_wputenv" | "_putenv_s" => _putenv(emu, api),
        "__wgetmainargs" => __wgetmainargs(emu),
        "_wcmdln" | "__p__wcmdln" => _wcmdln(emu, addr, api),

        // --- Exception handling ---
        "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => _except_handler4(emu, api),
        "_set_se_translator" | "_set_new_handler" | "_set_new_mode" | "set_terminate"
        | "set_unexpected" => _set_se_translator(emu, api),
        "_EH_prolog3" | "_EH_prolog3_catch" | "_EH_prolog3_GS" => _EH_prolog3(emu, api),

        // --- Math functions ---
        "_ftol2" | "_ftol2_sse" => _ftol2(emu, api),
        "abs" | "labs" => abs(emu, api),
        "ceil" => ceil(emu),
        "floor" => floor(emu),
        "sqrt" => sqrt(emu),
        "pow" => pow(emu),
        "log" => log(emu),
        "log10" => log10(emu),
        "exp" => exp(emu),
        "sin" => sin(emu),
        "cos" => cos(emu),
        "tan" => tan(emu),
        "atan" => atan(emu),
        "atan2" => atan2(emu),
        "fabs" => fabs(emu),
        "fmod" => fmod(emu),

        // --- Misc ---
        "clock" => clock(emu),
        "_time64" => _time64(emu),
        "_controlfp_s" => _controlfp_s(emu),
        "_unlock" => _unlock(emu),
        "_configthreadlocale" => _configthreadlocale(emu),
        "_amsg_exit" => _amsg_exit(emu),
        "_crt_atexit" | "_register_onexit_function" => _crt_atexit(emu, api),
        "_initialize_onexit_table" => _initialize_onexit_table(emu),
        "__CppXcptFilter" => __CppXcptFilter(emu),
        "__acrt_iob_func" => __acrt_iob_func(emu),
        "__iob_func" => __iob_func(emu),
        "__p__environ" => __p__environ(emu),
        "__stdio_common_vfprintf" => __stdio_common_vfprintf(emu),
        "__stdio_common_vsprintf_s" => __stdio_common_vsprintf_s(emu),
        "_configure_narrow_argv" => _configure_narrow_argv(emu),
        "_set_errno" => _set_errno(emu),
        "_get_errno" => _get_errno(emu),
        "_get_initial_narrow_environment" => _get_initial_narrow_environment(emu),
        "_initialize_narrow_environment" => _initialize_narrow_environment(emu),
        "_localtime64" => _localtime64(emu),
        "_set_app_type" => _set_app_type(emu),
        "_set_invalid_parameter_handler" => _set_invalid_parameter_handler(emu),
        "_set_thread_local_invalid_parameter_handler" => {
            _set_thread_local_invalid_parameter_handler(emu)
        }
        "_strcmpi" => _strcmpi(emu),
        "_time32" => _time32(emu),
        "_wfopen" => _wfopen(emu),
        "abort" => abort(emu),
        "ptr" => ptr(emu),
        "setvbuf" => setvbuf(emu),
        "signal" => signal(emu),
        "str" => str(emu),
        "strtok_s" => strtok_s(emu),
        "vwprintf" => vwprintf(emu),
        "wcsftime" => wcsftime(emu),
        "__lconv_init" => __lconv_init(emu, addr),
        "__setlc_active" | "__unguarded_readlc_active" => __setlc_active(emu, addr, api),
        "_set_error_mode" => _set_error_mode(emu),
        "_set_fmode" => _set_fmode(emu),
        "__stdio_common_vsprintf" => __stdio_common_vsprintf(emu),
        "__stdio_common_vsscanf" => __stdio_common_vsscanf(emu),
        "_sprintf_l" => _sprintf_l(emu),
        "wprintf" => wprintf(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
