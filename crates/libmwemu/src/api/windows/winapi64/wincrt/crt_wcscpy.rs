use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_wcscpy(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let ws = emu.maps.read_wide_string(src);
    emu.maps.write_wide_string(dst, &ws);
    log_red!(emu, "wincrt!wcscpy 0x{:x} '{}'", dst, ws);
    call.set_return(emu, dst);
}
