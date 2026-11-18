use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn DefWindowProcA(emu: &mut emu::Emu) {
    def_window_proc(emu, "DefWindowProcA");
}

fn def_window_proc(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let argument_1 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!{api} hwnd:0x{:x} msg:0x{:x}",
        argument_0,
        argument_1
    );
    call.set_return(emu, 0);
}

pub(super) fn DefWindowProcW(emu: &mut emu::Emu) {
    def_window_proc(emu, "DefWindowProcW");
}
