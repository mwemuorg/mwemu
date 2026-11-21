use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindFirstVolumeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpszVolumeName = api_try!(emu, call.arg_u32(emu, 0));
    let _cchBufferLength = api_try!(emu, call.arg_u32(emu, 1));

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindFirstVolumeA {}", lpszVolumeName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn FindFirstVolumeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 2);
    log_red!(emu, "kernel32!FindFirstVolumeW");
    call.set_return(emu, helper::handler_create("FindFirstVolumeW"));
}
