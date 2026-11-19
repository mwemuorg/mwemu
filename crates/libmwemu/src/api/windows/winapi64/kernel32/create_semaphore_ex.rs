use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateSemaphoreExA(emu: &mut Emu) {
    invoke(emu, false, "CreateSemaphoreExA");
}

pub fn CreateSemaphoreExW(emu: &mut Emu) {
    invoke(emu, true, "CreateSemaphoreExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let initial = api_try!(emu, call.arg_i32(emu, 1));
    let maximum = api_try!(emu, call.arg_i32(emu, 2));
    let name = strings::read_text(emu, api_try!(emu, call.arg(emu, 3)), wide);
    log_red!(
        emu,
        "kernel32!{} initial:{} maximum:{} name:'{}'",
        api,
        initial,
        maximum,
        name
    );
    if initial < 0 || maximum <= 0 || initial > maximum {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        call.set_return(emu, 0);
    } else {
        call.set_return(emu, helper::handler_create(&name));
    }
}
