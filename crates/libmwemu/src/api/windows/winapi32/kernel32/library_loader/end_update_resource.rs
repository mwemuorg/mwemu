use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn EndUpdateResourceA(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceA");
}

fn end_update_resource(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _hUpdate = api_try!(emu, call.arg_u32(emu, 0));
    let _fDiscard = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn EndUpdateResourceW(emu: &mut emu::Emu) {
    end_update_resource(emu, "EndUpdateResourceW");
}
