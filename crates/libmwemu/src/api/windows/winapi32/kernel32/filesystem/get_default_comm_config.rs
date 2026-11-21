use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetDefaultCommConfigA(emu: &mut emu::Emu) {
    get_default_comm_config(emu, "GetDefaultCommConfigA", false);
}

fn get_default_comm_config(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpszName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpCC = api_try!(emu, call.arg_u32(emu, 1));
    let _lpdwSize = api_try!(emu, call.arg_u32(emu, 2));

    let lpszName = strings::read_text(emu, lpszName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetDefaultCommConfigW(emu: &mut emu::Emu) {
    get_default_comm_config(emu, "GetDefaultCommConfigW", true);
}
