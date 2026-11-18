use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BuildCommDCBA(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBA");
}

fn build_comm_dcb(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _lpDef = api_try!(emu, call.arg_u32(emu, 0));
    let _lpDCB = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn BuildCommDCBW(emu: &mut emu::Emu) {
    build_comm_dcb(emu, "BuildCommDCBW");
}
