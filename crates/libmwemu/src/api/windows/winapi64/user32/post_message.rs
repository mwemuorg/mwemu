use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn PostMessageA(emu: &mut emu::Emu) {
    post_message(emu, "PostMessageA");
}

fn post_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let argument_1 = api_try!(emu, call.arg(emu, 1));
    let argument_2 = api_try!(emu, call.arg(emu, 2));
    let argument_3 = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "user32!{api} hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        argument_0,
        argument_1,
        argument_2,
        argument_3
    );
    call.set_return(emu, 1);
}

pub(super) fn PostMessageW(emu: &mut emu::Emu) {
    post_message(emu, "PostMessageW");
}
