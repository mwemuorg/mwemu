use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::heap;
use crate::winapi::policy::api_try;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GdipAlloc" => GdipAlloc(emu),
        "GdipFree" => GdipFree(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn GdipAlloc(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let address = heap::heap_allocate(emu, 0, size).unwrap_or(0);
    log_red!(emu, "gdiplus!GdipAlloc size={size} => {address:#x}");
    call.set_return(emu, address);
    call.cleanup_args(emu, 1);
}

fn GdipFree(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdiplus!GdipFree address={address:#x}");
    if address != 0 && !heap::heap_free(emu, 0, address) {
        log::warn!("gdiplus!GdipFree: unknown allocation {address:#x}");
    }
    call.cleanup_args(emu, 1);
}
