use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCharWidth32A(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32A");
}

fn get_char_width32(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let first = api_try!(emu, call.arg_u32(emu, 1));
    let last = api_try!(emu, call.arg_u32(emu, 2));
    let buf = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + (i as u64) * 4, 8);
        }
    }
    log_red!(emu, "gdi32!{api} first:{} last:{}", first, last);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32W");
}
