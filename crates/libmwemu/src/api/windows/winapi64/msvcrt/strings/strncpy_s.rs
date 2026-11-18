use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strncpy_s(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let _dst_size = api_try!(emu, call.arg(emu, 1));
    let src = api_try!(emu, call.arg(emu, 2));
    let n = api_try!(emu, call.arg(emu, 3)) as usize;
    let s = emu.maps.read_string(src);
    let truncated: String = s.chars().take(n).collect();
    if dst != 0 {
        emu.maps.write_string(dst, &truncated);
    }
    log_red!(
        emu,
        "msvcrt!strncpy_s dst:0x{:x} '{}' n:{}",
        dst,
        truncated,
        n
    );
    call.set_return(emu, 0);
}
