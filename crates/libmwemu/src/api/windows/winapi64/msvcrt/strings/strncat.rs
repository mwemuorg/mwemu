use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn strncat(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let n = api_try!(emu, call.arg(emu, 2)) as usize;
    let dst_s = emu.maps.read_string(dst);
    let src_s = emu.maps.read_string(src);
    let copy_len = src_s.len().min(n);
    let offset = dst + dst_s.len() as u64;
    for (i, b) in src_s.as_bytes()[..copy_len].iter().enumerate() {
        emu.maps.write_byte(offset + i as u64, *b);
    }
    emu.maps.write_byte(offset + copy_len as u64, 0);
    log_red!(emu, "msvcrt!strncat 0x{:x} '{}' {}", dst, src_s, n);
    call.set_return(emu, dst);
}
