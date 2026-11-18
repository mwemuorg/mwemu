use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateIconFromResourceEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let presbits = api_try!(emu, call.arg(emu, 0));
    let dw_res_size = api_try!(emu, call.arg_u32(emu, 1));
    let is_icon = api_try!(emu, call.arg_u32(emu, 2));
    let dw_ver = api_try!(emu, call.arg_u32(emu, 3));
    let cx = api_try!(emu, call.arg_i32(emu, 4));
    let cy = api_try!(emu, call.arg_i32(emu, 5));
    let flags = api_try!(emu, call.arg_u32(emu, 6));
    log_red!(
        emu,
        "user32!CreateIconFromResourceEx data:0x{:x} size:{} icon:{} cx:{} cy:{} flags:0x{:x}",
        presbits,
        dw_res_size,
        is_icon,
        cx,
        cy,
        flags
    );
    let hndl = helper::handler_create("user32://CreateIconFromResourceEx");
    call.set_return(emu, hndl);
}
