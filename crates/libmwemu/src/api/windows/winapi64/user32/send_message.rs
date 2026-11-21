use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SendMessageA(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageA", false);
}

pub(super) fn SendMessageW(emu: &mut emu::Emu) {
    send_message(emu, "SendMessageW", true);
}

fn send_message(emu: &mut emu::Emu, api: &str, wide: bool) {
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
    if wide {
    } else {
        emu.set_last_error(0);
    }

    call.set_return(emu, 0);
}
