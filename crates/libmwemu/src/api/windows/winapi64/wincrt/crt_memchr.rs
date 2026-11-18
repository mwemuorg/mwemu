use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_memchr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s = api_try!(emu, call.arg(emu, 0));
    let c = api_try!(emu, call.arg_u8(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2));
    let mut result: u64 = 0;
    for i in 0..n {
        if emu.maps.read_byte(s + i).unwrap_or(0) == c {
            result = s + i;
            break;
        }
    }
    call.set_return(emu, result);
}
