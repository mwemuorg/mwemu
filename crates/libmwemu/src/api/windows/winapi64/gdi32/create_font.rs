use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFontA(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontA", false);
}

fn create_font(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let height = api_try!(emu, call.arg_i32(emu, 0));
    let width = api_try!(emu, call.arg_i32(emu, 1));
    let escapement = api_try!(emu, call.arg_i32(emu, 2));
    let orientation = api_try!(emu, call.arg_i32(emu, 3));
    let weight = api_try!(emu, call.arg_i32(emu, 4));
    let italic = api_try!(emu, call.arg_u32(emu, 5));
    let underline = api_try!(emu, call.arg_u32(emu, 6));
    let strikeout = api_try!(emu, call.arg_u32(emu, 7));
    let charset = api_try!(emu, call.arg_u32(emu, 8));
    let out_precision = api_try!(emu, call.arg_u32(emu, 9));
    let clip_precision = api_try!(emu, call.arg_u32(emu, 10));
    let quality = api_try!(emu, call.arg_u32(emu, 11));
    let pitch_and_family = api_try!(emu, call.arg_u32(emu, 12));
    let face_ptr = api_try!(emu, call.arg(emu, 13));
    let face_name = if face_ptr != 0 {
        strings::read_text(emu, face_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} h:{} w:{} weight:{} italic:{} charset:{} quality:{} face:'{}'",
        height,
        width,
        weight,
        italic,
        charset,
        quality,
        face_name
    );
    let _ = (
        escapement,
        orientation,
        underline,
        strikeout,
        out_precision,
        clip_precision,
        pitch_and_family,
    );
    call.set_return(emu, helper::handler_create("gdi32://font"));
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontW", true);
}
