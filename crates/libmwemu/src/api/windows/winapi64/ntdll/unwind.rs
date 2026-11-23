use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::exception::unwind64;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "RtlUnwind" | "RtlUnwindEx" => unwind64::unwind(emu, "ntdll", api),
        "RtlVirtualUnwind" => unwind64::virtual_unwind(emu, "ntdll"),
        "__C_specific_handler" => unwind64::c_specific_handler(emu, "ntdll"),
        "RtlDispatchException" => unwind64::dispatch_exception_api(emu, "ntdll"),
        "RtlRaiseException" => unwind64::raise_exception_api(emu, "ntdll"),
    },
        handled(()) => true,
        fallback => false,
    )
}
