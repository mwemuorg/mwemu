use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _wcstombs_l(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let locale = api_try!(emu, call.arg(emu, 3));
    let s = emu.maps.read_wide_string(src);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "wincrt!_wcstombs_l '{}' n:{}", s, n);
    call.set_return(emu, s.len() as u64);
}
