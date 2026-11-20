use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::controls::*;

const INITCOMMONCONTROLSEX_SIZE: u32 = 2 * size_of::<u32>() as u32;
const SUPPORTED_CLASSES: u32 = ICC_WIN95_CLASSES
    | ICC_DATE_CLASSES
    | ICC_USEREX_CLASSES
    | ICC_COOL_CLASSES
    | ICC_INTERNET_CLASSES
    | ICC_PAGESCROLLER_CLASS
    | ICC_NATIVEFNTCTL_CLASS
    | ICC_STANDARD_CLASSES
    | ICC_LINK_CLASS;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "InitCommonControls" => InitCommonControls(emu),
        "InitCommonControlsEx" => InitCommonControlsEx(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

fn InitCommonControls(emu: &mut Emu) {
    // The bounded UI model makes its control classes available without registration.
    log_red!(emu, "comctl32!InitCommonControls");
}

fn InitCommonControlsEx(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let size = emu.maps.read_dword(input);
    let flags = input
        .checked_add(size_of::<u32>() as u64)
        .and_then(|address| emu.maps.read_dword(address));
    let valid = size == Some(INITCOMMONCONTROLSEX_SIZE)
        && flags.is_some_and(|flags| flags & !SUPPORTED_CLASSES == 0);
    log_red!(
        emu,
        "comctl32!InitCommonControlsEx input={input:#x} flags={flags:?} => {valid}"
    );
    call.set_return(emu, valid as u64);
    call.cleanup_args(emu, 1);
}
