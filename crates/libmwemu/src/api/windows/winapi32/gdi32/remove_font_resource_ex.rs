use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExA");
}

fn remove_font_resource_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    call.cleanup_args(emu, 3);
    log_red!(
        emu,
        "gdi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExW");
}
