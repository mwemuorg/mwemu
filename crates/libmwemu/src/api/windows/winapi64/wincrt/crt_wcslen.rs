use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_wcslen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let ptr = api_try!(emu, call.arg(emu, 0));
    let ws = emu.maps.read_wide_string(ptr);
    log_red!(emu, "wincrt!wcslen '{}' => {}", ws, ws.len());
    call.set_return(emu, ws.len() as u64);
}
