use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    get_text_extent_point32(emu, "GetTextExtentPoint32A");
}

fn get_text_extent_point32(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let len = api_try!(emu, call.arg_u32(emu, 2));
    let size_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, len * 7); // cx
        emu.maps.write_dword(size_ptr + 4, 16); // cy
    }
    log_red!(emu, "gdi32!{api} len:{}", len);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    get_text_extent_point32(emu, "GetTextExtentPoint32W");
}
