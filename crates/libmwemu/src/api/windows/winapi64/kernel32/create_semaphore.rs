use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    create_semaphore(emu, "CreateSemaphoreA", false);
}

pub fn CreateSemaphoreW(emu: &mut emu::Emu) {
    create_semaphore(emu, "CreateSemaphoreW", true);
}

fn create_semaphore(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _security = api_try!(emu, call.arg(emu, 0));
    let initial = api_try!(emu, call.arg(emu, 1));
    let maximum = api_try!(emu, call.arg(emu, 2));
    let name = api_try!(emu, call.arg(emu, 3));
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!{} init:{} max:{} name:'{}'",
        api,
        initial,
        maximum,
        name_s
    );
    call.set_return(emu, helper::handler_create(&name_s));
}
