use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::heap::{heap_allocate, heap_free, heap_reallocate};
use crate::winapi::policy::api_try;

pub fn CryptMemAlloc(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg_u32(emu, 0));
    let memory = heap_allocate(emu, 0, size.into()).unwrap_or(0);
    log_red!(emu, "crypt32!CryptMemAlloc size={size} result={memory:#x}");
    call.set_return(emu, memory);
    call.cleanup_args(emu, 1);
}

pub fn CryptMemRealloc(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let memory = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let resized = if memory == 0 {
        heap_allocate(emu, 0, size.into())
    } else {
        heap_reallocate(emu, 0, memory, size.into())
    };
    log_red!(
        emu,
        "crypt32!CryptMemRealloc memory={memory:#x} size={size}"
    );
    call.set_return(emu, resized.unwrap_or(0));
    call.cleanup_args(emu, 2);
}

pub fn CryptMemFree(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let memory = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CryptMemFree memory={memory:#x}");
    if memory != 0 {
        heap_free(emu, 0, memory);
    }
    call.cleanup_args(emu, 1);
}
