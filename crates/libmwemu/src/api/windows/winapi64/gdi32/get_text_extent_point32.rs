use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    get_text_extent_point32(emu, "GetTextExtentPoint32A", false);
}

fn get_text_extent_point32(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let text_ptr = api_try!(emu, call.arg(emu, 1));
    let len = api_try!(emu, call.arg(emu, 2));
    let size = api_try!(emu, call.arg(emu, 3));
    let text = if text_ptr != 0 && len > 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    if size != 0 {
        emu.maps.write_dword(size, (len * 7) as u32); // cx
        emu.maps.write_dword(size + 4, 16); // cy
    }
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} text:'{}' len:{} => {}x16",
        hdc,
        text,
        len,
        len * 7
    );
    call.set_return(emu, 1);
}

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    get_text_extent_point32(emu, "GetTextExtentPoint32W", true);
}
