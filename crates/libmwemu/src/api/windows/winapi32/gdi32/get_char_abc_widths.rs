use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCharABCWidthsA(emu: &mut emu::Emu) {
    get_char_abc_widths(emu, "GetCharABCWidthsA");
}

fn get_char_abc_widths(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let first = api_try!(emu, call.arg_u32(emu, 1));
    let last = api_try!(emu, call.arg_u32(emu, 2));
    let buf = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let count = (last.saturating_sub(first) + 1).min(256);
    if buf != 0 {
        for i in 0..count {
            let off = (i as u64) * 12; // ABC struct: 3 ints
            emu.maps.write_dword(buf + off, 1); // abcA
            emu.maps.write_dword(buf + off + 4, 6); // abcB
            emu.maps.write_dword(buf + off + 8, 1); // abcC
        }
    }

    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} first:0x{:x} last:0x{:x} buf:0x{:x}",
        hdc,
        first,
        last,
        buf
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

pub fn GetCharABCWidthsW(emu: &mut emu::Emu) {
    get_char_abc_widths(emu, "GetCharABCWidthsW");
}
