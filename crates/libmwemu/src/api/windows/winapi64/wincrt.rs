use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;

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

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    gateway_by_name(api, emu)
}

pub fn gateway_by_name(api: &str, emu: &mut emu::Emu) -> String {
    match api {
        "_initialize_onexit_table" => _initialize_onexit_table(emu),
        "_register_onexit_function" => _register_onexit_function(emu),
        "_get_initial_narrow_environment" => _get_initial_narrow_environment(emu),
        "_initialize_narrow_environment" => _initialize_narrow_environment(emu),
        "_configure_narrow_argv" => _configure_narrow_argv(emu),
        "_set_invalid_parameter_handler" => set_invalid_parameter_handler(emu),
        "_set_app_type" => _set_app_type(emu),
        "malloc" => malloc(emu),
        "calloc" => calloc(emu),
        "free" => free(emu),
        "realloc" => realloc(emu),
        "_crt_atexit" => _crt_atexit(emu),
        "__p___argv" => __p___argv(emu),
        "__p___argc" => __p___argc(emu),
        "__p__environ" => __p__environ(emu),
        "__acrt_iob_func" => __acrt_iob_func(emu),
        "__p__commode" => __p__commode(emu),
        "__p__fmode" => __p__fmode(emu),
        "_set_new_mode" => _set_new_mode(emu),
        "setvbuf" => setvbuf(emu),
        "__stdio_common_vfprintf" => __stdio_common_vfprintf(emu),
        "puts" => puts(emu),
        "strlen" => strlen(emu),
        "strncmp" => strncmp(emu),
        "memcpy" => memcpy(emu),
        "memset" => crt_memset(emu),
        "memmove" => crt_memmove(emu),
        "memcmp" => crt_memcmp(emu),
        "memchr" => crt_memchr(emu),
        "abort" => abort(emu),
        "signal" => signal(emu),
        "strcmp" => crt_strcmp(emu),
        "strncpy" => crt_strncpy(emu),
        "strcpy" => crt_strcpy(emu),
        "strcat" => crt_strcat(emu),
        "strstr" => crt_strstr(emu),
        "strchr" => crt_strchr(emu),
        "strrchr" => crt_strrchr(emu),
        "_stricmp" | "_strcmpi" => crt_stricmp(emu),
        "_strnicmp" => crt_strnicmp(emu),
        "strtol" => crt_strtol(emu),
        "strtoul" => crt_strtoul(emu),
        "atoi" => crt_atoi(emu),
        "atol" => crt_atoi(emu),
        "_itoa" => crt_itoa(emu),
        "wcslen" => crt_wcslen(emu),
        "wcscpy" => crt_wcscpy(emu),
        "wcscat" => crt_wcscat(emu),
        "wcscmp" => crt_wcscmp(emu),
        "_wcsicmp" => crt_wcsicmp(emu),
        "wcsncpy" => crt_wcsncpy(emu),
        "_wtoi" => crt_wtoi(emu),
        "towupper" => crt_towupper(emu),
        "towlower" => crt_towlower(emu),
        "toupper" => crt_toupper(emu),
        "tolower" => crt_tolower(emu),
        "isalpha" => crt_isalpha(emu),
        "isdigit" => crt_isdigit(emu),
        "isspace" => crt_isspace(emu),
        "isalnum" => crt_isalnum(emu),
        "isupper" => crt_isupper(emu),
        "islower" => crt_islower(emu),
        "_msize" => crt_msize(emu),
        "_aligned_malloc" => crt_aligned_malloc(emu),
        "_aligned_free" => crt_aligned_free(emu),
        "fopen" | "_wfopen" => crt_fopen(emu),
        "fclose" => crt_fclose(emu),
        "fread" => crt_fread(emu),
        "fwrite" => crt_fwrite(emu),
        "fflush" => crt_fflush(emu),
        "fseek" => crt_fseek(emu),
        "ftell" => crt_ftell(emu),
        "feof" => crt_feof(emu),
        "fprintf"
        | "sprintf"
        | "snprintf"
        | "sscanf"
        | "_snprintf"
        | "_sprintf_l"
        | "__stdio_common_vsprintf"
        | "__stdio_common_vsscanf"
        | "__stdio_common_vsprintf_s" => crt_format_stub(emu, api),
        "printf" | "vprintf" | "vfprintf" | "vsprintf" | "vsnprintf" | "_vsnprintf" | "wprintf"
        | "vwprintf" => crt_print_stub(emu, api),
        "exit" | "_exit" | "_cexit" => crt_exit(emu, api),
        "getenv" | "_wgetenv" => crt_getenv(emu, api),
        "_errno" => crt_errno(emu),
        "_set_errno" | "_get_errno" => crt_errno_stub(emu, api),
        "__getmainargs" => crt_getmainargs(emu),
        "__wgetmainargs" => crt_getmainargs(emu),
        "_initterm" => crt_initterm(emu),
        "_initterm_e" => crt_initterm_e(emu),
        "abs" | "labs" => crt_abs(emu),
        "rand" => crt_rand(emu),
        "srand" => crt_srand(emu),
        "_ftol" | "_ftol2" | "_ftol2_sse" => crt_ftol(emu),
        "_except_handler3" | "_except_handler4" | "__CxxFrameHandler" | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => crt_except_handler(emu, api),
        "_CxxThrowException" => crt_cxx_throw(emu),
        "_XcptFilter" | "__CppXcptFilter" => crt_xcpt_filter(emu, api),
        "_controlfp" => crt_controlfp(emu),
        "_controlfp_s" => crt_controlfp_s(emu),
        "__dllonexit" => crt_dllonexit(emu),
        "_onexit" => crt_onexit(emu),
        "clock" => crt_clock(emu),
        "time" | "_time64" | "_time32" => crt_time(emu),
        "_lock" | "_unlock" => crt_lock_noop(emu, api),
        "_beginthread" => crt_beginthread(emu),
        "_beginthreadex" => crt_beginthreadex(emu),
        "_endthread" | "_endthreadex" => crt_endthread(emu, api),
        "strtok" | "strtok_s" => crt_strtok(emu),
        "_iob" | "__iob_func" => crt_iob(emu),
        "localtime" | "_localtime64" => crt_localtime(emu),
        "strftime" | "wcsftime" => crt_strftime(emu),
        "__setusermatherr"
        | "_set_error_mode"
        | "__set_app_type"
        | "_configthreadlocale"
        | "_set_fmode"
        | "__lconv_init"
        | "_amsg_exit"
        | "__setlc_active"
        | "__unguarded_readlc_active"
        | "_set_new_handler"
        | "_set_thread_local_invalid_parameter_handler" => {
            log_red!(emu, "wincrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        "_acmdln" | "__p__acmdln" | "_wcmdln" | "__p__wcmdln" => crt_acmdln(emu, api),
        "??2@YAPEAX_K@Z"
        | "??3@YAXPEAX@Z"
        | "??_V@YAXPEAX@Z"
        | "??0exception@@QEAA@AEBV0@@Z"
        | "??1type_info@@UEAA@XZ" => {
            log_red!(emu, "wincrt!{}", api);
            emu.regs_mut().rax = 0;
        }
        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented CRT API {} at 0x{:x}",
                    api,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented CRT API {}", api);
            log::warn!(
                "calling unimplemented CRT API {} at 0x{:x}",
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn _set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs().rcx;
    log_red!(emu, "wincrt!_set_app_type app_type: 0x{:x}", app_type);
    emu.regs_mut().rax = 0;
}

fn _initialize_narrow_environment(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_initialize_narrow_environment");
    emu.regs_mut().rax = 0;
}

fn _configure_narrow_argv(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;
    log_red!(emu, "wincrt!_configure_narrow_argv mode: 0x{:x}", mode);
    emu.regs_mut().rax = 0;
}

fn setvbuf(emu: &mut emu::Emu) {
    // int setvbuf(FILE *stream, char *buf, int mode, size_t size): configures
    // stream buffering. We don't do real buffered I/O, so accept it and report
    // success (0).
    let stream = emu.regs().rcx;
    let mode = emu.regs().r8;
    log_red!(
        emu,
        "wincrt!setvbuf stream: 0x{:x} mode: 0x{:x}",
        stream,
        mode
    );
    emu.regs_mut().rax = 0;
}

fn _set_new_mode(emu: &mut emu::Emu) {
    // int _set_new_mode(int newmode): selects whether malloc failures call the
    // new-handler. Returns the previous mode. We keep no CRT state, so report the
    // default (0). Pure init-time bookkeeping — a safe no-op for emulation.
    let newmode = emu.regs().rcx;
    log_red!(emu, "wincrt!_set_new_mode newmode: 0x{:x}", newmode);
    emu.regs_mut().rax = 0;
}

fn __p__commode(emu: &mut emu::Emu) {
    // int * __p__commode(void)
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__commode alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}

fn __p__fmode(emu: &mut emu::Emu) {
    // int * __p__fmode(void)
    let p = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__fmode alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}

fn __p__environ(emu: &mut emu::Emu) {
    // char *** __p__environ(void)
    // Return a pointer to a NULL-terminated environment pointer list (empty env).
    let envp = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p__environ alloc failed");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let _ = emu.maps.write_qword(envp, 0);
    emu.regs_mut().rax = envp;
}

fn calloc(emu: &mut emu::Emu) {
    let nmemb = emu.regs().rcx;
    let size = emu.regs().rdx;
    let total = nmemb.saturating_mul(size);
    if total == 0 {
        emu.regs_mut().rax = 0;
        return;
    }
    let base = match allocate_memory(emu, total) {
        Some(a) => a,
        None => {
            log::error!("wincrt!calloc out of memory (size={})", total);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    for i in 0..total {
        let _ = emu.maps.write_byte(base + i, 0);
    }
    log_red!(
        emu,
        "wincrt!calloc nmemb:{} size:{} =0x{:x}",
        nmemb,
        size,
        base
    );
    emu.regs_mut().rax = base;
}

fn free(emu: &mut emu::Emu) {
    let p = emu.regs().rcx;
    log_red!(emu, "wincrt!free 0x{:x}", p);
    release(emu, p);
    emu.regs_mut().rax = 0;
}

fn puts(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let msg = emu.maps.read_string(s);
    log_red!(emu, "wincrt!puts '{}'", msg);
    emu.regs_mut().rax = 0;
}

fn strlen(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let mut n: u64 = 0;
    while let Some(b) = emu.maps.read_byte(s + n) {
        if b == 0 {
            break;
        }
        n += 1;
        if n > 0x10_0000 {
            break;
        }
    }
    emu.regs_mut().rax = n;
}

fn strncmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8;
    let mut i: u64 = 0;
    let mut res: i64 = 0;
    while i < n {
        let b1 = emu.maps.read_byte(s1 + i).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i).unwrap_or(0);
        if b1 != b2 {
            res = (b1 as i64) - (b2 as i64);
            break;
        }
        if b1 == 0 {
            break;
        }
        i += 1;
    }
    emu.regs_mut().rax = res as u64;
}

fn crt_memset(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let val = emu.regs().rdx as u8;
    let count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "wincrt!memset dst:0x{:x} val:{} count:{}",
        dst,
        val,
        count
    );
    emu.maps.memset(dst, val, count);
    emu.regs_mut().rax = dst;
}

fn memcpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let sz = n.min(usize::MAX as u64) as usize;
    if let Some(bytes) = emu.maps.try_read_bytes(src, sz).map(|b| b.to_vec()) {
        let _ = emu.maps.write_bytes(dst, &bytes);
    }
    emu.regs_mut().rax = dst;
}

fn abort(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!abort");
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    emu.regs_mut().rax = 0;
}

fn signal(emu: &mut emu::Emu) {
    let sig = emu.regs().rcx;
    let handler = emu.regs().rdx;
    log_red!(emu, "wincrt!signal sig:{} handler:0x{:x}", sig, handler);
    emu.regs_mut().rax = 0;
}

fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;

    /*
    http://sandbox.hlt.bme.hu/~gaebor/STLdoc/VS2017/corecrt__startup_8h_source.html
    133 typedef struct _onexit_table_t
    134 {
    135     _PVFV* _first;
    136     _PVFV* _last;
    137     _PVFV* _end;
    138 } _onexit_table_t;
    139
     */

    log_red!(emu, "wincrt!_initialize_onexit_table");

    emu.regs_mut().rax = 0;
}

fn _register_onexit_function(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let callback = emu.regs().rdx;

    /*
    http://sandbox.hlt.bme.hu/~gaebor/STLdoc/VS2017/corecrt__startup_8h_source.html
    133 typedef struct _onexit_table_t
    134 {
    135     _PVFV* _first;
    136     _PVFV* _last;
    137     _PVFV* _end;
    138 } _onexit_table_t;
    139
     */

    log_red!(
        emu,
        "wincrt!_initialize_onexit_function callback: 0x{:x}",
        callback
    );

    emu.regs_mut().rax = 0;
}

/*
extern "C" char** __cdecl _get_initial_narrow_environment()
{
    return common_get_initial_environment<char>();
}
*/
fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let env = emu.regs().rcx;

    log_red!(
        emu,
        "wincrt!_get_initial_narrow_environment env: 0x{:x}",
        env
    );

    // TODO: Implement this
    emu.regs_mut().rax = 0;
}

// char*** CDECL __p___argv(void) { return &MSVCRT___argv; }
fn __p___argv(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!__p___argv");

    // First, allocate space for argv array (pointer array)
    // We'll allocate space for 2 pointers - one for program name and null terminator
    let argv_array_addr = match allocate_memory(emu, 16) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate argv array");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Allocate space for program name string (using a dummy name)
    let prog_name = "program.exe\0";
    let prog_name_addr = match allocate_memory(emu, prog_name.len() as u64) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate program name");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Write program name string
    emu.maps.write_string(prog_name_addr, prog_name);

    // Write argv array:
    // argv[0] = pointer to program name
    emu.maps.write_qword(argv_array_addr, prog_name_addr);
    // argv[1] = null terminator
    emu.maps.write_qword(argv_array_addr + 8, 0);

    // Allocate space for pointer to argv array
    let p_argv_addr = match allocate_memory(emu, 8) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argv cannot allocate p_argv");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    // Write pointer to argv array
    emu.maps.write_qword(p_argv_addr, argv_array_addr);

    // Return pointer to argv
    emu.regs_mut().rax = p_argv_addr;
}

// int* CDECL __p___argc(void) { return &MSVCRT___argc; }
fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs().rcx;

    log_red!(emu, "wincrt!__p___argc argc: 0x{:x}", argc);

    let argc_addr = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argc cannot allocate");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    emu.maps.write_dword(argc_addr, 1);
    emu.regs_mut().rax = argc_addr;
}

/*
FILE * CDECL __acrt_iob_func(int index)
{
    return &__iob_func()[index];
}
*/

fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;

    log_red!(emu, "wincrt!__acrt_iob_func index: 0x{:x}", index);

    // TODO: Implement this
    emu.regs_mut().rax = 0;
}

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

fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let options = emu.regs().rcx; // _In_ options
    let file = emu.regs().rdx; // _In_ FILE*
    let format = emu.regs().r8; // _In_ format string ptr
    let locale = emu.regs().r9; // _In_opt_ locale
    let va_list = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    // Just try to read the format string
    let fmt_str = emu.maps.read_string(format);
    let specs = parse_format_specifiers(&fmt_str);

    log_red!(
        emu,
        "wincrt!__stdio_common_vfprintf options: 0x{:x} file: 0x{:x} format: '{}' locale: 0x{:x} va_list: 0x{:x}",
        options,
        file,
        fmt_str,
        locale,
        va_list
    );

    let mut current_ptr = va_list;
    for spec in specs {
        match spec {
            "int" | "hex" | "ptr" => {
                let arg = emu.maps.read_qword(current_ptr).unwrap_or(0);
                current_ptr += 8; // Move to next arg
                log::trace!("arg: {:016x}", arg);
            }
            "str" => {
                let str_ptr = emu.maps.read_qword(current_ptr).unwrap_or(0);
                let string = emu.maps.read_string(str_ptr);
                current_ptr += 8;
                log::trace!("string: {}", string);
            }
            _ => {
                log::warn!(
                    "wincrt!__stdio_common_vfprintf unknown format character: {}",
                    spec
                );
            }
        }
    }

    // Return success (1) - this is super basic
    emu.regs_mut().rax = 1;
}

pub fn realloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;

    if addr == 0 {
        if size == 0 {
            emu.regs_mut().rax = 0;
            return;
        } else {
            let base = allocate_memory(emu, size).unwrap_or(0);

            log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", addr, size, base);

            emu.regs_mut().rax = base;
            return;
        }
    }

    if size == 0 {
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", addr, size);

        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
        return;
    }

    let prev_size = match classify(emu, addr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size,
        AllocKind::Invalid => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let new_addr = allocate_memory(emu, size).unwrap_or(0);

    emu.maps
        .memcpy(new_addr, addr, std::cmp::min(prev_size, size as usize));
    release(emu, addr);

    log_red!(
        emu,
        "msvcrt!realloc 0x{:x} {} =0x{:x}",
        addr,
        size,
        new_addr
    );

    emu.regs_mut().rax = new_addr;
}

fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_set_invalid_parameter_handler");
    emu.regs_mut().rax = 0;
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx; // In malloc, size is the only parameter

    if size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let base = allocate_memory(emu, size).unwrap_or(0);

    log_red!(emu, "msvcrt!malloc {} =0x{:x}", size, base);

    emu.regs_mut().rax = base;
}

/*
int _crt_atexit(
    _PVFV const function
)
*/
fn _crt_atexit(emu: &mut emu::Emu) {
    let function = emu.regs().rcx;
    log_red!(emu, "wincrt!_crt_atexit function: 0x{:x}", function);
    emu.regs_mut().rax = 0;
}

// --- String functions ---

fn crt_strcmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = emu.maps.read_string(s1);
    let str2 = emu.maps.read_string(s2);
    let res = str1.cmp(&str2) as i32;
    log_red!(emu, "wincrt!strcmp '{}' '{}' => {}", str1, str2, res);
    emu.regs_mut().rax = res as u64;
}

fn crt_strcpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let s = emu.maps.read_string(src);
    emu.maps.write_string(dst, &s);
    log_red!(emu, "wincrt!strcpy 0x{:x} '{}'", dst, s);
    emu.regs_mut().rax = dst;
}

fn crt_strncpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let s = emu.maps.read_string(src);
    let copy_len = s.len().min(n as usize);
    for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
        let _ = emu.maps.write_byte(dst + i as u64, *b);
    }
    for i in copy_len..n as usize {
        let _ = emu.maps.write_byte(dst + i as u64, 0);
    }
    log_red!(emu, "wincrt!strncpy 0x{:x} '{}' {}", dst, s, n);
    emu.regs_mut().rax = dst;
}

fn crt_strcat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let dst_s = emu.maps.read_string(dst);
    let src_s = emu.maps.read_string(src);
    emu.maps.write_string(dst + dst_s.len() as u64, &src_s);
    log_red!(emu, "wincrt!strcat '{}' + '{}'", dst_s, src_s);
    emu.regs_mut().rax = dst;
}

fn crt_strstr(emu: &mut emu::Emu) {
    let haystack = emu.regs().rcx;
    let needle = emu.regs().rdx;
    let h = emu.maps.read_string(haystack);
    let n = emu.maps.read_string(needle);
    let result = match h.find(&n) {
        Some(pos) => haystack + pos as u64,
        None => 0,
    };
    log_red!(emu, "wincrt!strstr '{}' '{}' => 0x{:x}", h, n, result);
    emu.regs_mut().rax = result;
}

fn crt_strchr(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let c = emu.regs().rdx as u8;
    let str_val = emu.maps.read_string(s);
    let result = match str_val.find(c as char) {
        Some(pos) => s + pos as u64,
        None => 0,
    };
    emu.regs_mut().rax = result;
}

fn crt_strrchr(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let c = emu.regs().rdx as u8;
    let str_val = emu.maps.read_string(s);
    let result = match str_val.rfind(c as char) {
        Some(pos) => s + pos as u64,
        None => 0,
    };
    emu.regs_mut().rax = result;
}

fn crt_stricmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = emu.maps.read_string(s1).to_ascii_lowercase();
    let str2 = emu.maps.read_string(s2).to_ascii_lowercase();
    let res = str1.cmp(&str2) as i32;
    log_red!(emu, "wincrt!_stricmp '{}' '{}' => {}", str1, str2, res);
    emu.regs_mut().rax = res as u64;
}

fn crt_strnicmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let str1 = emu.maps.read_string(s1).to_ascii_lowercase();
    let str2 = emu.maps.read_string(s2).to_ascii_lowercase();
    let a = &str1[..str1.len().min(n)];
    let b = &str2[..str2.len().min(n)];
    let res = a.cmp(b) as i32;
    emu.regs_mut().rax = res as u64;
}

fn crt_strtol(emu: &mut emu::Emu) {
    let nptr = emu.regs().rcx;
    let endptr = emu.regs().rdx;
    let base = emu.regs().r8 as u32;
    let s = emu.maps.read_string(nptr);
    let trimmed = s.trim();
    let val = i64::from_str_radix(
        trimmed.trim_start_matches("0x").trim_start_matches("0X"),
        if base == 0 {
            if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
                16
            } else {
                10
            }
        } else {
            base
        },
    )
    .unwrap_or(0);
    if endptr != 0 {
        emu.maps.write_qword(endptr, nptr + s.len() as u64);
    }
    log_red!(emu, "wincrt!strtol '{}' base:{} => {}", s, base, val);
    emu.regs_mut().rax = val as u64;
}

fn crt_strtoul(emu: &mut emu::Emu) {
    let nptr = emu.regs().rcx;
    let endptr = emu.regs().rdx;
    let base = emu.regs().r8 as u32;
    let s = emu.maps.read_string(nptr);
    let trimmed = s.trim();
    let val = u64::from_str_radix(
        trimmed.trim_start_matches("0x").trim_start_matches("0X"),
        if base == 0 {
            if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
                16
            } else {
                10
            }
        } else {
            base
        },
    )
    .unwrap_or(0);
    if endptr != 0 {
        emu.maps.write_qword(endptr, nptr + s.len() as u64);
    }
    log_red!(emu, "wincrt!strtoul '{}' base:{} => {}", s, base, val);
    emu.regs_mut().rax = val;
}

fn crt_atoi(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let s = emu.maps.read_string(ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "wincrt!atoi '{}' => {}", s, val);
    emu.regs_mut().rax = val as u64;
}

fn crt_itoa(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as i32;
    let buf = emu.regs().rdx;
    let radix = emu.regs().r8 as u32;
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "wincrt!_itoa {} radix:{} => '{}'", val, radix, s);
    emu.regs_mut().rax = buf;
}

fn crt_strtok(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    log_red!(emu, "wincrt!strtok 0x{:x}", s);
    emu.regs_mut().rax = 0;
}

// --- Wide string functions ---

fn crt_wcslen(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let ws = emu.maps.read_wide_string(ptr);
    log_red!(emu, "wincrt!wcslen '{}' => {}", ws, ws.len());
    emu.regs_mut().rax = ws.len() as u64;
}

fn crt_wcscpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let ws = emu.maps.read_wide_string(src);
    emu.maps.write_wide_string(dst, &ws);
    log_red!(emu, "wincrt!wcscpy 0x{:x} '{}'", dst, ws);
    emu.regs_mut().rax = dst;
}

fn crt_wcscat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let dst_ws = emu.maps.read_wide_string(dst);
    let src_ws = emu.maps.read_wide_string(src);
    emu.maps
        .write_wide_string(dst + (dst_ws.len() as u64) * 2, &src_ws);
    log_red!(emu, "wincrt!wcscat '{}' + '{}'", dst_ws, src_ws);
    emu.regs_mut().rax = dst;
}

fn crt_wcscmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let w1 = emu.maps.read_wide_string(s1);
    let w2 = emu.maps.read_wide_string(s2);
    let res = w1.cmp(&w2) as i32;
    log_red!(emu, "wincrt!wcscmp '{}' '{}' => {}", w1, w2, res);
    emu.regs_mut().rax = res as u64;
}

fn crt_wcsicmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let w1 = emu.maps.read_wide_string(s1).to_ascii_lowercase();
    let w2 = emu.maps.read_wide_string(s2).to_ascii_lowercase();
    let res = w1.cmp(&w2) as i32;
    log_red!(emu, "wincrt!_wcsicmp '{}' '{}' => {}", w1, w2, res);
    emu.regs_mut().rax = res as u64;
}

fn crt_wcsncpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let ws = emu.maps.read_wide_string(src);
    let chars: Vec<u16> = ws.encode_utf16().collect();
    let copy_len = chars.len().min(n as usize);
    for i in 0..copy_len {
        emu.maps.write_word(dst + (i as u64) * 2, chars[i]);
    }
    for i in copy_len..n as usize {
        emu.maps.write_word(dst + (i as u64) * 2, 0);
    }
    emu.regs_mut().rax = dst;
}

fn crt_wtoi(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let ws = emu.maps.read_wide_string(ptr);
    let val = ws.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "wincrt!_wtoi '{}' => {}", ws, val);
    emu.regs_mut().rax = val as u64;
}

fn crt_towupper(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u16;
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let upper = ch.to_ascii_uppercase() as u64;
    emu.regs_mut().rax = upper;
}

fn crt_towlower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u16;
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let lower = ch.to_ascii_lowercase() as u64;
    emu.regs_mut().rax = lower;
}

fn crt_toupper(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = (c as char).to_ascii_uppercase() as u64;
}

fn crt_tolower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = (c as char).to_ascii_lowercase() as u64;
}

// --- Character classification ---

fn crt_isalpha(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_alphabetic() {
        1
    } else {
        0
    };
}

fn crt_isdigit(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_digit() { 1 } else { 0 };
}

fn crt_isspace(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_whitespace() {
        1
    } else {
        0
    };
}

fn crt_isalnum(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_alphanumeric() {
        1
    } else {
        0
    };
}

fn crt_isupper(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_uppercase() {
        1
    } else {
        0
    };
}

fn crt_islower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_lowercase() {
        1
    } else {
        0
    };
}

// --- Memory functions ---

fn crt_memmove(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let sz = n.min(0x100_0000) as usize;
    if let Some(bytes) = emu.maps.try_read_bytes(src, sz).map(|b| b.to_vec()) {
        let _ = emu.maps.write_bytes(dst, &bytes);
    }
    emu.regs_mut().rax = dst;
}

fn crt_memcmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8;
    let mut res: i64 = 0;
    for i in 0..n {
        let b1 = emu.maps.read_byte(s1 + i).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i).unwrap_or(0);
        if b1 != b2 {
            res = (b1 as i64) - (b2 as i64);
            break;
        }
    }
    emu.regs_mut().rax = res as u64;
}

fn crt_memchr(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let c = emu.regs().rdx as u8;
    let n = emu.regs().r8;
    let mut result: u64 = 0;
    for i in 0..n {
        if emu.maps.read_byte(s + i).unwrap_or(0) == c {
            result = s + i;
            break;
        }
    }
    emu.regs_mut().rax = result;
}

fn crt_msize(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = match classify(emu, ptr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size as u64,
        AllocKind::Invalid => 0xFFFF_FFFF_FFFF_FFFFu64,
    };
    log_red!(emu, "wincrt!_msize 0x{:x} => {}", ptr, size);
    emu.regs_mut().rax = size;
}

fn crt_aligned_malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    let alignment = emu.regs().rdx;
    let base = allocate_memory(emu, size + alignment).unwrap_or(0);
    let aligned = if base == 0 {
        0
    } else {
        (base + alignment - 1) & !(alignment - 1)
    };
    log_red!(
        emu,
        "wincrt!_aligned_malloc size:{} align:{} =0x{:x}",
        size,
        alignment,
        aligned
    );
    emu.regs_mut().rax = aligned;
}

fn crt_aligned_free(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "wincrt!_aligned_free 0x{:x}", ptr);
    release(emu, ptr);
}

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
    log_red!(emu, "wincrt!fclose 0x{:x}", fp);
    emu.regs_mut().rax = 0;
}

fn crt_fread(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let fp = emu.regs().r9;
    log_red!(
        emu,
        "wincrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x}",
        buf,
        size,
        count,
        fp
    );
    emu.regs_mut().rax = 0;
}

fn crt_fwrite(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let fp = emu.regs().r9;
    log_red!(
        emu,
        "wincrt!fwrite buf:0x{:x} size:{} count:{} fp:0x{:x}",
        buf,
        size,
        count,
        fp
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
    let origin = emu.regs().r8;
    log_red!(
        emu,
        "wincrt!fseek fp:0x{:x} offset:{} origin:{}",
        fp,
        offset,
        origin
    );
    emu.regs_mut().rax = 0;
}

fn crt_ftell(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    log_red!(emu, "wincrt!ftell 0x{:x} => 0", fp);
    emu.regs_mut().rax = 0;
}

fn crt_feof(emu: &mut emu::Emu) {
    let fp = emu.regs().rcx;
    log_red!(emu, "wincrt!feof 0x{:x} => 1", fp);
    emu.regs_mut().rax = 1;
}

fn crt_format_stub(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.regs_mut().rax = 0;
}

fn crt_print_stub(emu: &mut emu::Emu, api: &str) {
    let fmt = emu.regs().rcx;
    let s = emu.maps.read_string(fmt);
    log_red!(emu, "wincrt!{} '{}'", api, s);
    emu.regs_mut().rax = 0;
}

// --- Process/Environment ---

fn crt_exit(emu: &mut emu::Emu, api: &str) {
    let code = emu.regs().rcx;
    log_red!(emu, "wincrt!{} code:{}", api, code);
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
    emu.regs_mut().rax = 0;
}

fn crt_getenv(emu: &mut emu::Emu, api: &str) {
    let name_ptr = emu.regs().rcx;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "wincrt!{} '{}' => NULL", api, name);
    emu.regs_mut().rax = 0;
}

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
    log_red!(emu, "wincrt!{}", api);
    emu.regs_mut().rax = 0;
}

fn crt_getmainargs(emu: &mut emu::Emu) {
    let p_argc = emu.regs().rcx;
    let p_argv = emu.regs().rdx;
    let p_env = emu.regs().r8;
    log_red!(emu, "wincrt!__getmainargs");
    if p_argc != 0 {
        emu.maps.write_dword(p_argc, 1);
    }
    if p_argv != 0 {
        emu.maps.write_qword(p_argv, 0);
    }
    if p_env != 0 {
        emu.maps.write_qword(p_env, 0);
    }
    emu.regs_mut().rax = 0;
}

fn crt_initterm(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    let end = emu.regs().rdx;
    log_red!(emu, "wincrt!_initterm 0x{:x} - 0x{:x}", start, end);
    emu.regs_mut().rax = 0;
}

fn crt_initterm_e(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    let end = emu.regs().rdx;
    log_red!(emu, "wincrt!_initterm_e 0x{:x} - 0x{:x}", start, end);
    emu.regs_mut().rax = 0;
}

// --- Math ---

fn crt_abs(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as i64;
    emu.regs_mut().rax = val.unsigned_abs();
}

fn crt_rand(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!rand => 42");
    emu.regs_mut().rax = 42;
}

fn crt_srand(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!srand");
}

fn crt_ftol(emu: &mut emu::Emu) {
    let val = emu.fpu_mut().get_st(0);
    log_red!(emu, "wincrt!_ftol {} => {}", val, val as i64);
    emu.regs_mut().rax = val as i64 as u64;
}

// --- Exception handling ---

fn crt_except_handler(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.regs_mut().rax = 0;
}

fn crt_cxx_throw(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_CxxThrowException");
    emu.regs_mut().rax = 0;
}

fn crt_xcpt_filter(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.regs_mut().rax = 0;
}

// --- Misc ---

fn crt_controlfp(emu: &mut emu::Emu) {
    let new_val = emu.regs().rcx;
    let mask = emu.regs().rdx;
    log_red!(emu, "wincrt!_controlfp 0x{:x} 0x{:x}", new_val, mask);
    emu.regs_mut().rax = 0x0008_001f;
}

fn crt_controlfp_s(emu: &mut emu::Emu) {
    let current = emu.regs().rcx;
    let new_val = emu.regs().rdx;
    let mask = emu.regs().r8;
    if current != 0 {
        emu.maps.write_dword(current, 0x0008_001f);
    }
    log_red!(emu, "wincrt!_controlfp_s 0x{:x} 0x{:x}", new_val, mask);
    emu.regs_mut().rax = 0;
}

fn crt_dllonexit(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    log_red!(emu, "wincrt!__dllonexit 0x{:x}", func);
    emu.regs_mut().rax = func;
}

fn crt_onexit(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    log_red!(emu, "wincrt!_onexit 0x{:x}", func);
    emu.regs_mut().rax = func;
}

fn crt_clock(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!clock => 1000");
    emu.regs_mut().rax = 1000;
}

fn crt_time(emu: &mut emu::Emu) {
    let t_ptr = emu.regs().rcx;
    let now = 0x6718_0000u64;
    if t_ptr != 0 {
        emu.maps.write_qword(t_ptr, now);
    }
    log_red!(emu, "wincrt!time => 0x{:x}", now);
    emu.regs_mut().rax = now;
}

fn crt_lock_noop(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
}

fn crt_beginthread(emu: &mut emu::Emu) {
    let start = emu.regs().rcx;
    let stack_size = emu.regs().rdx;
    let arg = emu.regs().r8;
    log_red!(
        emu,
        "wincrt!_beginthread start:0x{:x} stack:{} arg:0x{:x}",
        start,
        stack_size,
        arg
    );
    emu.regs_mut().rax = 0x1337;
}

fn crt_beginthreadex(emu: &mut emu::Emu) {
    let security = emu.regs().rcx;
    let stack_size = emu.regs().rdx;
    let start = emu.regs().r8;
    let arg = emu.regs().r9;
    log_red!(
        emu,
        "wincrt!_beginthreadex start:0x{:x} arg:0x{:x}",
        start,
        arg
    );
    emu.regs_mut().rax = 0x1337;
}

fn crt_endthread(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "wincrt!{}", api);
    emu.is_running
        .store(0, std::sync::atomic::Ordering::Relaxed);
}

fn crt_iob(emu: &mut emu::Emu) {
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    log_red!(emu, "wincrt!_iob => 0x{:x}", iob_addr);
    emu.regs_mut().rax = iob_addr;
}

fn crt_localtime(emu: &mut emu::Emu) {
    let tm_addr = allocate_memory(emu, 64).unwrap_or(0);
    if tm_addr != 0 {
        emu.maps.write_dword(tm_addr, 0); // tm_sec
        emu.maps.write_dword(tm_addr + 4, 0); // tm_min
        emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
        emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
        emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
        emu.maps.write_dword(tm_addr + 20, 124); // tm_year (2024-1900)
        emu.maps.write_dword(tm_addr + 24, 1); // tm_wday
        emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
        emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
    }
    log_red!(emu, "wincrt!localtime => 0x{:x}", tm_addr);
    emu.regs_mut().rax = tm_addr;
}

fn crt_strftime(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let max_size = emu.regs().rdx;
    let s = "01/01/2024 00:00:00";
    if dst != 0 && max_size > 0 {
        emu.maps.write_string(dst, s);
    }
    log_red!(emu, "wincrt!strftime => '{}'", s);
    emu.regs_mut().rax = s.len() as u64;
}

fn crt_acmdln(emu: &mut emu::Emu, api: &str) {
    let p = match allocate_memory(emu, 64) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    if api.contains("wcmdln") || api.contains("_w") {
        emu.maps.write_wide_string(p, "program.exe");
    } else {
        emu.maps.write_string(p, "program.exe");
    }
    log_red!(emu, "wincrt!{} => 0x{:x}", api, p);
    emu.regs_mut().rax = p;
}
