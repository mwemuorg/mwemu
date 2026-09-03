use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
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
    if let Some(heap) = emu.heap_arenas.first() {
        if let Some(size) = heap.allocation_size(addr) {
            return AllocKind::Arena { size };
        }
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
/// Honors cfg.heap_free_soft.
fn release(emu: &mut emu::Emu, addr: u64) {
    if emu.cfg.heap_free_soft {
        return;
    }
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
        "abort" => abort(emu),
        "signal" => signal(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented CRT API {}", api);
            }
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
    let p = allocate_memory(emu, 4).expect("wincrt!__p__commode alloc failed");
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}

fn __p__fmode(emu: &mut emu::Emu) {
    // int * __p__fmode(void)
    let p = allocate_memory(emu, 4).expect("wincrt!__p__fmode alloc failed");
    let _ = emu.maps.write_dword(p, 0);
    emu.regs_mut().rax = p;
}

fn __p__environ(emu: &mut emu::Emu) {
    // char *** __p__environ(void)
    // Return a pointer to a NULL-terminated environment pointer list (empty env).
    let envp = allocate_memory(emu, 8).expect("wincrt!__p__environ alloc failed");
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
    let base = allocate_memory(emu, total).expect("wincrt!calloc out of memory");
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
    loop {
        if let Some(b) = emu.maps.read_byte(s + n) {
            if b == 0 {
                break;
            }
            n += 1;
        } else {
            break;
        }
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
    let argv_array_addr = allocate_memory(emu, 16) // 2 * sizeof(pointer) on x64
        .expect("wincrt!__p___argv cannot allocate argv array");

    // Allocate space for program name string (using a dummy name)
    let prog_name = "program.exe\0";
    let prog_name_addr = allocate_memory(emu, prog_name.len() as u64)
        .expect("wincrt!__p___argv cannot allocate program name");

    // Write program name string
    emu.maps.write_string(prog_name_addr, prog_name);

    // Write argv array:
    // argv[0] = pointer to program name
    emu.maps.write_qword(argv_array_addr, prog_name_addr);
    // argv[1] = null terminator
    emu.maps.write_qword(argv_array_addr + 8, 0);

    // Allocate space for pointer to argv array
    let p_argv_addr = allocate_memory(emu, 8) // sizeof(pointer) on x64
        .expect("wincrt!__p___argv cannot allocate p_argv");

    // Write pointer to argv array
    emu.maps.write_qword(p_argv_addr, argv_array_addr);

    // Return pointer to argv
    emu.regs_mut().rax = p_argv_addr;
}

// int* CDECL __p___argc(void) { return &MSVCRT___argc; }
fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs().rcx;

    log_red!(emu, "wincrt!__p___argc argc: 0x{:x}", argc);

    let argc_addr = allocate_memory(emu, 4).expect("wincrt!__p___argc cannot allocate");
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
        if c == '%' {
            if let Some(next) = chars.next() {
                if next != '%' {
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
        }
    }
    specs
}

fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let options = emu.regs().rcx; // _In_ options
    let file = emu.regs().rdx; // _In_ FILE*
    let format = emu.regs().r8; // _In_ format string ptr
    let locale = emu.regs().r9; // _In_opt_ locale
    let va_list = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wincrt!__stdio_common_vfprintf cannot read_qword va_list");

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
                let arg = emu
                    .maps
                    .read_qword(current_ptr)
                    .expect("wincrt!__stdio_common_vfprintf cannot read_qword arg");
                current_ptr += 8; // Move to next arg
                log::trace!("arg: {:016x}", arg);
            }
            "str" => {
                let str_ptr = emu
                    .maps
                    .read_qword(current_ptr)
                    .expect("wincrt!__stdio_common_vfprintf cannot read_qword str_ptr");
                let string = emu.maps.read_string(str_ptr);
                current_ptr += 8;
                log::trace!("string: {}", string);
            }
            _ => {
                unimplemented!(
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
            let base = allocate_memory(emu, size).expect("msvcrt!malloc out of memory");

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

    let new_addr = allocate_memory(emu, size).expect("msvcrt!realloc out of memory");

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

    let base = allocate_memory(emu, size).expect("msvcrt!malloc out of memory");

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
    // TODO: Implement this
    emu.regs_mut().rax = 0;
}
