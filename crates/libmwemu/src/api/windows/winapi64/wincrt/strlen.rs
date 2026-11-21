use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn strlen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s = api_try!(emu, call.arg(emu, 0));
    let mut n: u64 = 0;
    while let Some(b) = emu.maps.read_byte(s + n) {
        if b == 0 {
            break;
        }
        n += 1;
        if n > 0x10_0000 {
            break;
        }
    }
    call.set_return(emu, n);
}
