use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn strncmp(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let s1 = api_try!(emu, call.arg(emu, 0));
    let s2 = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2));
    let mut i: u64 = 0;
    let mut res: i64 = 0;
    while i < n {
        let b1 = emu.maps.read_byte(s1 + i).unwrap_or(0);
        let b2 = emu.maps.read_byte(s2 + i).unwrap_or(0);
        if b1 != b2 {
            res = (b1 as i64) - (b2 as i64);
            break;
        }
        if b1 == 0 {
            break;
        }
        i += 1;
    }
    call.set_return(emu, res as u64);
}
