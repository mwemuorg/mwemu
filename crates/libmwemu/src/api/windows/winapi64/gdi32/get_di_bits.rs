use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetDIBits(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let hbmp = api_try!(emu, call.arg(emu, 1));
    let start = api_try!(emu, call.arg(emu, 2));
    let lines = api_try!(emu, call.arg(emu, 3));
    let lpvbits = api_try!(emu, call.arg(emu, 4));
    let lpbmi = api_try!(emu, call.arg(emu, 5));
    if lpbmi != 0 {
        emu.maps.write_dword(lpbmi, 40); // biSize = BITMAPINFOHEADER
        emu.maps.write_dword(lpbmi + 4, 1920); // biWidth
        emu.maps.write_dword(lpbmi + 8, 1080u32 as i32 as u32); // biHeight
        emu.maps.write_word(lpbmi + 12, 1); // biPlanes
        emu.maps.write_word(lpbmi + 14, 32); // biBitCount
    }
    log_red!(
        emu,
        "gdi32!GetDIBits hdc:0x{:x} hbmp:0x{:x} start:{} lines:{} bits:0x{:x} bmi:0x{:x}",
        hdc,
        hbmp,
        start,
        lines,
        lpvbits,
        lpbmi
    );
    emu.set_last_error(0);
    call.set_return(emu, lines);
}
