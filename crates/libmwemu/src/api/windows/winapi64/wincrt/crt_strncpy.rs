use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_strncpy(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2));
    let s = emu.maps.read_string(src);
    let copy_len = s.len().min(n as usize);
    for (i, b) in s.as_bytes()[..copy_len].iter().enumerate() {
        let _ = emu.maps.write_byte(dst + i as u64, *b);
    }
    for i in copy_len..n as usize {
        let _ = emu.maps.write_byte(dst + i as u64, 0);
    }
    log_red!(emu, "wincrt!strncpy 0x{:x} '{}' {}", dst, s, n);
    call.set_return(emu, dst);
}
