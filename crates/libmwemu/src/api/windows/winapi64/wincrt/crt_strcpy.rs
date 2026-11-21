use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strcpy(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_string(src);
    emu.maps.write_string(dst, &s);
    log_red!(emu, "wincrt!strcpy 0x{:x} '{}'", dst, s);
    call.set_return(emu, dst);
}
