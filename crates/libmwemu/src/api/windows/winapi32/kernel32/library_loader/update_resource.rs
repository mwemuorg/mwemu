use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn UpdateResourceA(emu: &mut emu::Emu) {
    update_resource(emu, "UpdateResourceA", false);
}

fn update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hUpdate = api_try!(emu, call.arg_u32(emu, 0));
    let _lpType = api_try!(emu, call.arg_u32(emu, 1));
    let lpName = api_try!(emu, call.arg_u32(emu, 2));
    let _wLanguage = api_try!(emu, call.arg_u32(emu, 3));
    let _lpData = api_try!(emu, call.arg_u32(emu, 4));
    let _cb = api_try!(emu, call.arg_u32(emu, 5));

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn UpdateResourceW(emu: &mut emu::Emu) {
    update_resource(emu, "UpdateResourceW", true);
}
