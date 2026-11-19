use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::trace;
use crate::winapi::policy::api_try;

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
        "memcpy" | "memmove" => (copy, "destination source size", true),
        "memset" => (fill, "destination value size", true),
        _ => return false,
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
pub(crate) fn float_to_integer(emu: &mut Emu, api: &str) {
    let call = WinApi(Cdecl);
    let value = emu.fpu_mut().pop_f64();
    let limit = -(i64::MIN as f64);
    let result = if value.is_finite() && value >= -limit && value < limit {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    log_red!(emu, "msvcrt!{} {} => {}", api, value, result);
    call.set_return_u64(emu, result as u64);
}
