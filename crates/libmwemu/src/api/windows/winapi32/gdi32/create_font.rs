use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFontA(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontA");
}

fn create_font(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    let arg3 = api_try!(emu, call.arg_u32(emu, 2));
    let arg4 = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "gdi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    call.cleanup_args(emu, 14);
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontW");
}
