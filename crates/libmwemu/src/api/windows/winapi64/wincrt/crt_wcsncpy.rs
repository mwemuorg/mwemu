use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_wcsncpy(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2));
    let ws = emu.maps.read_wide_string(src);
    let chars: Vec<u16> = ws.encode_utf16().collect();
    let copy_len = chars.len().min(n as usize);
    for (i, c) in chars.iter().take(copy_len).enumerate() {
        emu.maps.write_word(dst + (i as u64) * 2, *c);
    }
    for i in copy_len..n as usize {
        emu.maps.write_word(dst + (i as u64) * 2, 0);
    }
    call.set_return(emu, dst);
}
