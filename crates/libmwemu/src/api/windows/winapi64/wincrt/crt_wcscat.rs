use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_wcscat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let dst_ws = emu.maps.read_wide_string(dst);
    let src_ws = emu.maps.read_wide_string(src);
    emu.maps
        .write_wide_string(dst + (dst_ws.len() as u64) * 2, &src_ws);
    log_red!(emu, "wincrt!wcscat '{}' + '{}'", dst_ws, src_ws);
    call.set_return(emu, dst);
}
