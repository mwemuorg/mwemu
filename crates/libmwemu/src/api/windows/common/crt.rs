use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::trace;
use crate::winapi::common::{buffers, memory::Call};
use crate::winapi::helper::WinApiAbi;

pub(crate) fn malloc(emu: &mut Emu) {
    let size = Call::Api.arg(emu, 0);
    let value = buffers::allocate(emu, size, 16, 0, BufferKind::Crt).unwrap_or(0);
    WinApiAbi::set_ret(emu, value);
}

pub(crate) fn calloc(emu: &mut Emu) {
    let count = Call::Api.arg(emu, 0);
    let size = Call::Api.arg(emu, 1);
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
    WinApiAbi::set_ret(emu, value);
}

pub(crate) fn free(emu: &mut Emu) {
    let address = Call::Api.arg(emu, 0);
    buffers::free(emu, address, BufferKind::Crt);
}

pub(crate) fn realloc(emu: &mut Emu) {
    let address = Call::Api.arg(emu, 0);
    let size = Call::Api.arg(emu, 1);
    let value = buffers::reallocate(emu, address, size, BufferKind::Crt).unwrap_or(0);
    WinApiAbi::set_ret(emu, value);
}

pub(crate) fn aligned_malloc(emu: &mut Emu) {
    let size = Call::Api.arg(emu, 0);
    let alignment = Call::Api.arg(emu, 1);
    let value = buffers::allocate(emu, size, alignment, 0, BufferKind::Aligned).unwrap_or(0);
    WinApiAbi::set_ret(emu, value);
}

pub(crate) fn aligned_free(emu: &mut Emu) {
    let address = Call::Api.arg(emu, 0);
    buffers::free(emu, address, BufferKind::Aligned);
}

pub(crate) fn msize(emu: &mut Emu) {
    let address = Call::Api.arg(emu, 0);
    let value = buffers::size(emu, address, BufferKind::Crt).unwrap_or(u64::MAX);
    WinApiAbi::set_ret(emu, value);
}
/// cdecl exports leave x86 arguments for the caller to remove.
pub(crate) fn copy(emu: &mut Emu) {
    let dest = Call::Api.arg(emu, 0);
    let src = Call::Api.arg(emu, 1);
    let size = Call::Api.arg(emu, 2);
    if emu.maps.copy_range(dest, src, size as usize) {
        WinApiAbi::set_ret(emu, dest);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn fill(emu: &mut Emu) {
    let dest = Call::Api.arg(emu, 0);
    let value = Call::Api.arg(emu, 1) as u8;
    let size = Call::Api.arg(emu, 2);
    if emu.maps.fill_range(dest, value, size as usize) {
        WinApiAbi::set_ret(emu, dest);
    } else {
        emu.exception(ExceptionType::BadAddressDereferencing);
    }
}

pub(crate) fn task_alloc(emu: &mut Emu) {
    trace::api(emu, "CoTaskMemAlloc", "size", true, task_alloc_impl);
}

fn task_alloc_impl(emu: &mut Emu) {
    let size = Call::Api.arg(emu, 0);
    let value = buffers::allocate(emu, size, 16, 0, BufferKind::Task).unwrap_or(0);
    Call::Api.finish(emu, value, 1);
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
    let ptr = Call::Api.arg(emu, 0);
    let size = Call::Api.arg(emu, 1);
    let value = buffers::reallocate(emu, ptr, size, BufferKind::Task).unwrap_or(0);
    Call::Api.finish(emu, value, 2);
}

pub(crate) fn task_free(emu: &mut Emu) {
    trace::api(emu, "CoTaskMemFree", "address", false, task_free_impl);
}

fn task_free_impl(emu: &mut Emu) {
    let ptr = Call::Api.arg(emu, 0);
    buffers::free(emu, ptr, BufferKind::Task);
    Call::Api.finish(emu, 0, 1);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
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
    trace::api(emu, api, parameters, returns_value, handler);
    true
}
