use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::common::ntdll;
use crate::emu;
use crate::winapi::policy;
use crate::winapi::policy::api_try;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    if ntdll::dispatch_user_profile(api, emu) || ntdll::dispatch_counted_string(api, "ntdll", emu) {
        return true;
    }
    dispatch_api!(api, {
        "stricmp" => stricmp(emu),
        "strlen" | "strcat" => ntdll::byte_string(emu, api),
        "sscanf" => policy::unsupported_with_abi(emu, "ntdll", api, emu.pc(), WinApi(Cdecl)),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn stricmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let str1ptr = api_try!(emu, call.arg(emu, 0));
    let str2ptr = api_try!(emu, call.arg(emu, 1));
    ntdll::stricmp(emu, str1ptr, str2ptr);

    call.cleanup_args(emu, 2);
}
