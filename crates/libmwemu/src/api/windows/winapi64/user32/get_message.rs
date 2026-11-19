use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetMessageA(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageA");
}

fn get_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "user32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    call.set_return(emu, 0);
}

pub(super) fn GetMessageW(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageW");
}
