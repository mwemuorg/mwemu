use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::api::windows::libraries::msvcrt::strings::{
    dispatch_bounded, dispatch_character, dispatch_float, dispatch_integer, dispatch_text,
    dispatch_tokens, dispatch_units, dispatch_wide_search, string_operation,
};
use crate::arch::Arch;
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::threading::context::CrtThreadState;
use crate::winapi::common::memory::read_array;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::common::{
    buffers, crt_arguments, crt_callbacks, crt_environment, crt_signals, seh4,
};
use crate::winapi::common::{threads, trace};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crt::*;
use crate::windows::constants::errors::ERROR_NOT_ENOUGH_MEMORY;

const X87_PRECISION_SHIFT: u16 = 8;
const X87_PRECISION_MASK: u16 = 0b11 << X87_PRECISION_SHIFT;

const DEFAULT_RANDOM_SEED: u32 = 1;
const RANDOM_MULTIPLIER: u32 = 214013;
const RANDOM_INCREMENT: u32 = 2531011;
const RANDOM_SHIFT: u32 = 16;

pub(crate) fn malloc(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let size = api_try!(emu, call.arg(emu, 0));
    let value = buffers::allocate(emu, size, 16, 0, BufferKind::Crt).unwrap_or(0);
    call.set_return(emu, value);
}

pub(crate) fn calloc(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let count = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let value = count
        .checked_mul(size)
        .and_then(|length| {
            let address = buffers::allocate(emu, length, 16, 0, BufferKind::Crt)?;
            if !emu.maps.fill_range(address, 0, length as usize) {
                buffers::free(emu, address, BufferKind::Crt);
                return None;
            }
            Some(address)
        })
        .unwrap_or(0);
    call.set_return(emu, value);
}

pub(crate) fn free(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    buffers::free(emu, address, BufferKind::Crt);
}

pub(crate) fn realloc(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let value = buffers::reallocate(emu, address, size, BufferKind::Crt).unwrap_or(0);
    call.set_return(emu, value);
}

pub(crate) fn aligned_malloc(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let size = api_try!(emu, call.arg(emu, 0));
    let alignment = api_try!(emu, call.arg(emu, 1));
    let value = buffers::allocate(emu, size, alignment, 0, BufferKind::Aligned).unwrap_or(0);
    call.set_return(emu, value);
}

pub(crate) fn aligned_free(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    buffers::free(emu, address, BufferKind::Aligned);
}

pub(crate) fn msize(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let address = api_try!(emu, call.arg(emu, 0));
    let value = buffers::size(emu, address, BufferKind::Crt).unwrap_or(u64::MAX);
    call.set_return(emu, value);
}

/// cdecl exports leave x86 arguments for the caller to remove.
pub(crate) fn copy(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let dest = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    if emu.maps.copy_range(dest, src, size as usize) {
        call.set_return(emu, dest);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn fill(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let dest = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u8(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    if emu.maps.fill_range(dest, value, size as usize) {
        call.set_return(emu, dest);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn task_alloc(emu: &mut Emu) {
    trace::api(emu, "CoTaskMemAlloc", "size", true, task_alloc_impl);
}

fn task_alloc_impl(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let value = buffers::allocate(emu, size, 16, 0, BufferKind::Task).unwrap_or(0);
    call.set_return(emu, value);
    call.cleanup_args(emu, 1);
}

pub(crate) fn task_realloc(emu: &mut Emu) {
    trace::api(
        emu,
        "CoTaskMemRealloc",
        "address size",
        true,
        task_realloc_impl,
    );
}

fn task_realloc_impl(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let value = buffers::reallocate(emu, ptr, size, BufferKind::Task).unwrap_or(0);
    call.set_return(emu, value);
    call.cleanup_args(emu, 2);
}

pub(crate) fn task_free(emu: &mut Emu) {
    trace::api(emu, "CoTaskMemFree", "address", false, task_free_impl);
}

fn task_free_impl(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    buffers::free(emu, ptr, BufferKind::Task);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch_memory(api: &str, emu: &mut Emu) -> bool {
    let (handler, parameters): (fn(&mut Emu), &str) = match api {
        "memcpy" | "memmove" => (copy, "destination source size"),
        "memset" => (fill, "destination value size"),
        "strrchr" => (
            |emu| string_operation(emu, "strrchr", 2),
            "string character",
        ),
        _ => return false,
    };
    trace::invoke(emu, WinApi(Cdecl), api, parameters, true, handler);
    true
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let call = WinApi(Cdecl);
    let (handler, parameters, returns_value): (fn(&mut Emu), &str, bool) = match api {
        "malloc" => (malloc, "size", true),
        "calloc" => (calloc, "count size", true),
        "realloc" => (realloc, "address size", true),
        "free" => (free, "address", false),
        "_aligned_malloc" => (aligned_malloc, "size alignment", true),
        "_aligned_free" => (aligned_free, "address", false),
        "_msize" => (msize, "address", true),
        _ => return dispatch_memory(api, emu),
    };
    trace::invoke(emu, call, api, parameters, returns_value, handler);
    true
}

/// Normal cdecl math calls read stack arguments on x86 and XMM arguments on x64.
pub(crate) fn math_unary(emu: &mut Emu, api: &str, operation: fn(f64) -> f64) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_f64(emu, 0, 0));
    let result = operation(value);
    log_red!(emu, "msvcrt!{} {} => {}", api, value, result);
    call.set_return_f64(emu, result);
}

pub(crate) fn math_binary(emu: &mut Emu, api: &str, operation: fn(f64, f64) -> f64) {
    let call = WinApi(Cdecl);
    let left = api_try!(emu, call.arg_f64(emu, 0, 0));
    let right = api_try!(emu, call.arg_f64(emu, 1, 2));
    let result = operation(left, right);
    log_red!(emu, "msvcrt!{} {} {} => {}", api, left, right, result);
    call.set_return_f64(emu, result);
}

/// Compiler _ftol helpers consume ST0 and return a signed integer in EDX:EAX.
pub(crate) fn float_to_integer(emu: &mut Emu, api: &str, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let value = emu.fpu_mut().pop_f64();
    let limit = -(i64::MIN as f64);
    let result = if value.is_finite() && value >= -limit && value < limit {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    log_red!(emu, "{}!{api} value={value} => {result}", runtime.library());
    call.set_return_u64(emu, result as u64);
}

#[derive(Clone, Copy)]
pub(crate) enum Runtime {
    Msvcrt,
    Ucrt,
}

impl Runtime {
    fn state(self, emu: &mut Emu) -> &mut CrtThreadState {
        let thread = &mut emu.threads[emu.current_thread_id].windows;
        match self {
            Self::Msvcrt => &mut thread.msvcrt,
            Self::Ucrt => &mut thread.ucrt,
        }
    }

    pub(crate) fn library(self) -> &'static str {
        match self {
            Self::Msvcrt => "msvcrt",
            Self::Ucrt => "ucrtbase",
        }
    }
}

pub(crate) fn is_universal_library(library: &str) -> bool {
    library == "ucrtbase" || library == "ucrtbase.dll" || library.starts_with("api-ms-win-crt-")
}

pub(crate) fn storage(emu: &mut Emu, name: &str, width: u64) -> Option<u64> {
    if let Some(map) = emu.maps.get_map_by_name(name) {
        return Some(map.get_base());
    }
    let base = emu.maps.alloc(width)?;
    emu.maps
        .create_map(name, base, width, Permission::READ_WRITE)
        .ok()?;
    Some(base)
}

fn configure_thread_locale(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let mode = api_try!(emu, call.arg_i32(emu, 0));
    if mode == -1 {
        // UCRT uses this private startup mode to change the process-wide locale policy.
        policy::unsupported_with_abi(
            emu,
            runtime.library(),
            "_configthreadlocale",
            emu.pc(),
            call,
        );
        return;
    }
    if !matches!(
        mode,
        0 | _ENABLE_PER_THREAD_LOCALE | _DISABLE_PER_THREAD_LOCALE
    ) {
        if record_errno(emu, runtime, EINVAL) {
            call.set_return(emu, u32::MAX.into());
        }
        return;
    }
    let name = format!(
        "{}_thread_locale_{}",
        runtime.library(),
        emu.current_thread().id
    );
    let Some(cell) = storage(emu, &name, size_of::<u32>() as u64) else {
        if record_errno(emu, runtime, ENOMEM) {
            call.set_return(emu, u32::MAX.into());
        }
        return;
    };
    let Some(enabled) = emu.maps.read_dword(cell) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if mode != 0
        && !emu
            .maps
            .write_dword(cell, u32::from(mode == _ENABLE_PER_THREAD_LOCALE))
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let previous = if enabled != 0 {
        _ENABLE_PER_THREAD_LOCALE
    } else {
        _DISABLE_PER_THREAD_LOCALE
    };
    call.set_return(emu, previous as u64);
}

pub(crate) fn errno_address(emu: &mut Emu, runtime: Runtime) -> Option<u64> {
    let address = runtime.state(emu).errno_address;
    if address != 0 {
        return Some(address);
    }
    let address = buffers::allocate(
        emu,
        size_of::<i32>() as u64,
        size_of::<i32>() as u64,
        0,
        BufferKind::Crt,
    )?;
    if !emu.maps.write_bytes(address, &0i32.to_le_bytes()) {
        buffers::free(emu, address, BufferKind::Crt);
        return None;
    }
    runtime.state(emu).errno_address = address;
    Some(address)
}

pub(crate) fn record_errno(emu: &mut Emu, runtime: Runtime, value: i32) -> bool {
    if errno_address(emu, runtime)
        .is_some_and(|address| emu.maps.write_bytes(address, &value.to_le_bytes()))
    {
        true
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        false
    }
}

fn errno(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let address = errno_address(emu, runtime).unwrap_or(0);
    log_red!(emu, "{}!_errno => {address:#x}", runtime.library());
    call.set_return(emu, address);
}

fn set_errno(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_i32(emu, 0));
    let Some(address) = errno_address(emu, runtime) else {
        call.set_return(emu, ENOMEM as u64);
        return;
    };
    if !emu.maps.write_bytes(address, &value.to_le_bytes()) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    log_red!(emu, "{}!_set_errno value={value}", runtime.library());
    call.set_return(emu, 0);
}

fn get_errno(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let output = api_try!(emu, call.arg(emu, 0));
    let Some(address) = errno_address(emu, runtime) else {
        call.set_return(emu, ENOMEM as u64);
        return;
    };
    // Invalid-parameter callbacks are not modeled; use the documented continuation result.
    let result = if output == 0 {
        if !emu.maps.write_bytes(address, &EINVAL.to_le_bytes()) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        EINVAL
    } else {
        let Some(bytes) = read_array::<{ size_of::<i32>() }>(emu, address) else {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        };
        if !emu.maps.write_bytes(output, &bytes) {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        0
    };
    log_red!(
        emu,
        "{}!_get_errno output={output:#x} => {result}",
        runtime.library()
    );
    call.set_return(emu, result as u64);
}

fn rand(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let state = runtime.state(emu);
    let seed = state
        .random_seed
        .unwrap_or(DEFAULT_RANDOM_SEED)
        .wrapping_mul(RANDOM_MULTIPLIER)
        .wrapping_add(RANDOM_INCREMENT);
    state.random_seed = Some(seed);
    let value = (seed >> RANDOM_SHIFT) & RAND_MAX;
    log_red!(emu, "{}!rand => {value}", runtime.library());
    call.set_return(emu, value as u64);
}

fn srand(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let seed = api_try!(emu, call.arg_u32(emu, 0));
    runtime.state(emu).random_seed = Some(seed);
    log_red!(emu, "{}!srand seed={seed}", runtime.library());
}

fn controlfp_s(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let [output, value, mask] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{}!_controlfp_s current={output:#x} value={value:#x} mask={mask:#x}",
        runtime.library()
    );
    if emu.cfg.arch == Arch::X86 && output == 0 && mask as u32 == _MCW_PC {
        let precision = match (value as u32) & _MCW_PC {
            _PC_24 => Some(0),
            _PC_53 => Some(2),
            _PC_64 => Some(3),
            _ => None,
        };
        if let Some(precision) = precision {
            let word = emu.fpu().get_ctrl();
            emu.fpu_mut()
                .set_ctrl((word & !X87_PRECISION_MASK) | (precision << X87_PRECISION_SHIFT));
            call.set_return(emu, 0);
            return;
        }
    }
    policy::unsupported_with_abi(emu, runtime.library(), "_controlfp_s", emu.pc(), call);
}

pub(crate) fn dispatch_runtime(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    if matches!(runtime, Runtime::Ucrt) && crt_signals::dispatch(emu, api) {
        return true;
    }
    if seh4::dispatch(emu, runtime.library(), api) {
        return true;
    }
    let handler = match api {
        "_ftol" if emu.cfg.arch == Arch::X86 => {
            float_to_integer(emu, api, runtime);
            return true;
        }
        "_configthreadlocale" => configure_thread_locale,
        "_beginthread" | "_beginthreadex" => {
            begin_thread(emu, runtime, api.ends_with("ex"));
            return true;
        }
        "_endthread" | "_endthreadex" => {
            end_thread(emu, api.ends_with("ex"));
            return true;
        }
        "_set_app_type" | "__set_app_type" => {
            set_app_type(emu, runtime);
            return true;
        }
        "__p__commode"
        | "__p__fmode"
        | "_set_fmode"
        | "_set_new_mode"
        | "_query_new_mode"
        | "_get_invalid_parameter_handler"
        | "_set_invalid_parameter_handler"
        | "_set_thread_local_invalid_parameter_handler"
        | "_get_thread_local_invalid_parameter_handler" => {
            setting(emu, api, runtime);
            return true;
        }
        "_CxxThrowException" => {
            policy::unsupported_with_abi(emu, runtime.library(), api, emu.pc(), WinApi(Stdcall));
            return true;
        }
        "_iob"
        | "_acmdln"
        | "_wcmdln"
        | "__setlc_active"
        | "__unguarded_readlc_active"
        | "_adjust_fdiv"
        | "__lconv_init"
        | "_lock"
        | "_unlock"
        | "ptr"
        | "_EH_prolog"
        | "_EH_prolog3"
        | "_EH_prolog3_catch"
        | "_EH_prolog3_GS"
        | "_except_handler3"
        | "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4"
        | "_XcptFilter"
        | "__CppXcptFilter"
        | "_set_error_mode"
        | "_set_se_translator"
        | "__setusermatherr"
        | "_set_new_handler"
        | "set_terminate"
        | "set_unexpected"
        | "signal"
        | "_controlfp"
        | "swprintf"
        | "sscanf"
        | "sscanf_s"
        | "_sscanf_s_l"
        | "_snprintf_s"
        | "sprintf_s"
        | "_sprintf_p"
        | "vwprintf"
        | "wprintf"
        | "__stdio_common_vsprintf"
        | "__stdio_common_vsprintf_s"
        | "__stdio_common_vsscanf"
        | "_sprintf_l"
        | "setvbuf"
        | "_setmode"
        | "wcsftime"
        | "localtime"
        | "_localtime64"
        | "_wsystem" => {
            policy::unsupported_with_abi(emu, runtime.library(), api, emu.pc(), WinApi(Cdecl));
            return true;
        }
        "_controlfp_s" => controlfp_s,
        "_errno" => errno,
        "_get_errno" => get_errno,
        "_set_errno" => set_errno,
        "rand" => rand,
        "srand" => srand,
        _ => {
            return crt_callbacks::dispatch(emu, runtime, api)
                || crt_arguments::dispatch(api, emu, runtime)
                || crt_environment::dispatch(api, emu, runtime)
                || (matches!(runtime, Runtime::Ucrt)
                    && (dispatch_tokens(api, emu, runtime)
                        || dispatch_units(api, emu, runtime)
                        || dispatch_integer(api, emu, runtime)))
                || (matches!(api, "wcstol" | "wcstoul") && dispatch_integer(api, emu, runtime))
                || dispatch_text(api, emu, runtime)
                || dispatch_bounded(api, emu, runtime)
                || dispatch_float(api, emu, runtime)
                || dispatch_character(api, runtime.library(), emu)
                || dispatch_wide_search(api, runtime.library(), emu);
        }
    };
    handler(emu, runtime);
    true
}

fn begin_thread(emu: &mut Emu, runtime: Runtime, extended: bool) {
    let call = WinApi(Cdecl);
    let (start, stack, param, flags, id) = if extended {
        (
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg_u32(emu, 1)),
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg_u32(emu, 4)),
            api_try!(emu, call.arg(emu, 5)),
        )
    } else {
        (
            api_try!(emu, call.arg(emu, 0)),
            api_try!(emu, call.arg_u32(emu, 1)),
            api_try!(emu, call.arg(emu, 2)),
            0,
            0,
        )
    };
    let result = if start == 0 {
        Err(EINVAL)
    } else {
        threads::create(emu, [stack.into(), start, param, flags.into(), id]).map_err(|error| {
            if error == ERROR_NOT_ENOUGH_MEMORY {
                EAGAIN
            } else {
                EINVAL
            }
        })
    };
    match result {
        Ok(handle) => call.set_return(emu, handle),
        Err(error) => {
            if record_errno(emu, runtime, error) {
                call.set_return(emu, if extended { 0 } else { u64::MAX });
            }
        }
    }
}

fn end_thread(emu: &mut Emu, extended: bool) {
    let call = WinApi(Cdecl);
    let code = if extended {
        api_try!(emu, call.arg_u32(emu, 0))
    } else {
        0
    };
    threads::finish_thread(emu, code);
}

fn set_app_type(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_i32(emu, 0));
    if let Some(cell) = storage(
        emu,
        match runtime {
            Runtime::Msvcrt => "msvcrt_app_type",
            Runtime::Ucrt => "ucrt_app_type",
        },
        4,
    ) {
        emu.maps.write_dword(cell, value as u32);
    }
}

fn setting(emu: &mut Emu, api: &str, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let is_handler = api.ends_with("parameter_handler");
    let prefix = match runtime {
        Runtime::Ucrt => "ucrt",
        Runtime::Msvcrt => "msvcrt",
    };
    let key = if api.contains("thread_local") {
        format!("{prefix}_invalid_parameter_{}", emu.current_thread().id)
    } else if is_handler {
        format!("{prefix}_invalid_parameter")
    } else if api.contains("new_mode") {
        format!("{prefix}_new_mode")
    } else if api.contains("commode") {
        format!("{prefix}_commode")
    } else {
        format!("{prefix}_fmode")
    };
    let width = if is_handler {
        pointer_size(emu)
    } else {
        size_of::<u32>() as u64
    };
    let initialize_fmode = api.contains("fmode") && emu.maps.get_map_by_name(&key).is_none();
    let Some(cell) = storage(emu, &key, width) else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if initialize_fmode && !emu.maps.write_dword(cell, _O_TEXT as u32) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    if api.starts_with("__p__") {
        call.set_return(emu, cell);
        return;
    }
    let value = if is_handler {
        read_pointer(emu, cell)
    } else {
        emu.maps.read_dword(cell).map(u64::from)
    };
    let Some(previous) = value else {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    };
    if api.starts_with("_set") {
        let next = api_try!(emu, call.arg(emu, 0));
        if (api == "_set_fmode" && !matches!(next as i32, _O_BINARY | _O_TEXT | _O_WTEXT))
            || (api == "_set_new_mode" && next as u32 > 1)
        {
            if record_errno(emu, runtime, EINVAL) {
                call.set_return(
                    emu,
                    if api == "_set_fmode" {
                        EINVAL as u64
                    } else {
                        u32::MAX as u64
                    },
                );
            }
            return;
        }
        let ok = if is_handler {
            write_pointer(emu, cell, next)
        } else {
            emu.maps.write_dword(cell, next as u32)
        };
        if !ok {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
    }
    call.set_return(emu, if api == "_set_fmode" { 0 } else { previous });
}
