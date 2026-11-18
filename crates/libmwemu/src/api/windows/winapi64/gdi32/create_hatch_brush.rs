use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateHatchBrush(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let style = api_try!(emu, call.arg_u32(emu, 0));
    let color = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    call.set_return(emu, helper::handler_create("gdi32://brush"));
}
