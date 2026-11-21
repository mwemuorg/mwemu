use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeleteMenu(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    let u_position = api_try!(emu, call.arg_u32(emu, 1));
    let u_flags = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "user32!DeleteMenu menu:0x{:x} pos:{} flags:0x{:x}",
        hmenu,
        u_position,
        u_flags
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    call.set_return(emu, 1);
}
