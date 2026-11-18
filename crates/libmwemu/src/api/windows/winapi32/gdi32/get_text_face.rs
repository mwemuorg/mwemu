use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetTextFaceA(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceA", false);
}

fn get_text_face(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let count = api_try!(emu, call.arg_u32(emu, 1));
    let buf = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        strings::write_string(emu, buf, face, wide);
    }
    log_red!(emu, "gdi32!{api} => '{}'", face);
    call.set_return(emu, face.len() as u64);
    call.cleanup_args(emu, 3);
}

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceW", true);
}
