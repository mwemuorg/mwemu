use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strcat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let dst_s = emu.maps.read_string(dst);
    let src_s = emu.maps.read_string(src);
    emu.maps.write_string(dst + dst_s.len() as u64, &src_s);
    log_red!(emu, "wincrt!strcat '{}' + '{}'", dst_s, src_s);
    call.set_return(emu, dst);
}
