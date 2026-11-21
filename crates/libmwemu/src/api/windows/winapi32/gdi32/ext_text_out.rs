use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ExtTextOutA(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutA");
}

fn ext_text_out(emu: &mut emu::Emu, api: &str) {
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
    call.cleanup_args(emu, 8);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutW");
}
