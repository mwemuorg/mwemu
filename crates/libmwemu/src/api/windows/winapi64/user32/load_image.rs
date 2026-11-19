use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadImageA(emu: &mut emu::Emu) {
    load_image(emu, "LoadImageA", false);
}

fn load_image(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let img_type = api_try!(emu, call.arg_u32(emu, 2));
    let cx = api_try!(emu, call.arg_i32(emu, 3));
    let cy = api_try!(emu, call.arg_i32(emu, 4));
    let fu_load = api_try!(emu, call.arg_u32(emu, 5));
    let name = if name_ptr > 0xffff {
        strings::read_text(emu, name_ptr, wide)
    } else {
        format!("#{}", name_ptr)
    };
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} name:'{}' type:{} cx:{} cy:{} flags:0x{:x}",
        hinstance,
        name,
        img_type,
        cx,
        cy,
        fu_load
    );
    let uri = format!("user32://{api}/{}", name);
    let hndl = helper::handler_create(&uri);
    call.set_return(emu, hndl);
}

pub fn LoadImageW(emu: &mut emu::Emu) {
    load_image(emu, "LoadImageW", true);
}
