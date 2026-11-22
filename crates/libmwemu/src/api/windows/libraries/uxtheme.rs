use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::windows::constants::controls::*;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "IsAppThemed" => IsAppThemed(emu),
        "IsThemeActive" => IsThemeActive(emu),
        "GetThemeAppProperties" => GetThemeAppProperties(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn IsAppThemed(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "uxtheme!IsAppThemed");
    call.set_return(emu, 1);
}

fn IsThemeActive(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "uxtheme!IsThemeActive");
    call.set_return(emu, 1);
}

fn GetThemeAppProperties(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    // Preserve the existing fixed profile; theme mutation/rendering is not modeled.
    log_red!(emu, "uxtheme!GetThemeAppProperties");
    call.set_return(emu, STAP_ALLOW_NONCLIENT as u64);
}
