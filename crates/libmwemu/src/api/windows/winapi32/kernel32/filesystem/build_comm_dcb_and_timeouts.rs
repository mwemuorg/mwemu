use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BuildCommDCBAndTimeoutsA(emu: &mut emu::Emu) {
    build_comm_dcb_and_timeouts(emu, "BuildCommDCBAndTimeoutsA");
}

fn build_comm_dcb_and_timeouts(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _lpDef = api_try!(emu, call.arg_u32(emu, 0));
    let _lpDCB = api_try!(emu, call.arg_u32(emu, 1));
    let _lpCommTimeouts = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn BuildCommDCBAndTimeoutsW(emu: &mut emu::Emu) {
    build_comm_dcb_and_timeouts(emu, "BuildCommDCBAndTimeoutsW");
}
