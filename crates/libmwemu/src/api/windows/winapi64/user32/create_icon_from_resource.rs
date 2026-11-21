use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateIconFromResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let presbits = api_try!(emu, call.arg(emu, 0));
    let dw_res_size = api_try!(emu, call.arg_u32(emu, 1));
    let is_icon = api_try!(emu, call.arg_u32(emu, 2));
    let dw_ver = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "user32!CreateIconFromResource data:0x{:x} size:{} icon:{} ver:0x{:x}",
        presbits,
        dw_res_size,
        is_icon,
        dw_ver
    );
    let hndl = helper::handler_create("user32://CreateIconFromResource");
    call.set_return(emu, hndl);
}
