use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateBoundaryDescriptorA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let Name = api_try!(emu, call.arg_u32(emu, 0));
    let _Flags = api_try!(emu, call.arg_u32(emu, 1));

    let Name = emu.maps.read_string(Name as u64);

    log_red!(emu, "kernel32!CreateBoundaryDescriptorA {}", Name);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn CreateBoundaryDescriptorW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 2);
    log_red!(emu, "kernel32!CreateBoundaryDescriptorW");
    call.set_return(emu, helper::handler_create("CreateBoundaryDescriptorW"));
}
