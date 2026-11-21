use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCharWidth32A(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32A");
}

fn get_char_width32(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let first = api_try!(emu, call.arg(emu, 1));
    let last = api_try!(emu, call.arg(emu, 2));
    let buf = api_try!(emu, call.arg(emu, 3));
    let count = last.saturating_sub(first).min(255) + 1;
    if buf != 0 {
        for i in 0..count {
            emu.maps.write_dword(buf + i * 4, 8);
        }
    }
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} first:{} last:{}",
        hdc,
        first,
        last
    );
    call.set_return(emu, 1);
}

pub fn GetCharWidth32W(emu: &mut emu::Emu) {
    get_char_width32(emu, "GetCharWidth32W");
}
