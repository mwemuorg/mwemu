use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateMutexA(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexA", false);
}

fn create_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let attr = api_try!(emu, call.arg_u32(emu, 0));
    let owner = api_try!(emu, call.arg_u32(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = strings::read_text(emu, name_ptr, wide);

    log_red!(emu, "kernel32!{api} '{}'", name);

    call.cleanup_args(emu, 3);

    let uri = format!("mutex://{}", name);
    call.set_return(emu, helper::handler_create(&uri));
}

pub fn CreateMutexW(emu: &mut emu::Emu) {
    create_mutex(emu, "CreateMutexW", true);
}
