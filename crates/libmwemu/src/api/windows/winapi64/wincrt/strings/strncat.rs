use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strncat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let existing = emu.maps.read_string(dst);
    let append = emu.maps.read_string(src);
    let truncated: String = append.chars().take(n).collect();
    let result = format!("{}{}", existing, truncated);
    if dst != 0 {
        emu.maps.write_string(dst, &result);
    }
    log_red!(emu, "wincrt!strncat => '{}' n:{}", result, n);
    call.set_return(emu, dst);
}
