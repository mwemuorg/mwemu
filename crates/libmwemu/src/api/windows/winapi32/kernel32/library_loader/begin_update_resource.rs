use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    begin_update_resource(emu, "BeginUpdateResourceA", false);
}

fn begin_update_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let pFileName = api_try!(emu, call.arg_u32(emu, 0));
    let _bDeleteExistingResources = api_try!(emu, call.arg_u32(emu, 1));

    let pFileName = strings::read_text(emu, pFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", pFileName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn BeginUpdateResourceW(emu: &mut emu::Emu) {
    begin_update_resource(emu, "BeginUpdateResourceW", true);
}
