use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerA", false);
}

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerW", true);
}

fn create_waitable_timer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _security = api_try!(emu, call.arg(emu, 0));
    let manual_reset = api_try!(emu, call.arg(emu, 1));
    let name = api_try!(emu, call.arg(emu, 2));
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!{} manual:{} name:'{}'",
        api,
        manual_reset,
        name_s
    );
    call.set_return(emu, helper::handler_create("waitable_timer"));
}
