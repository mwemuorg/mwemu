use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindNextVolumeMountPointA(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointA");
}

fn find_next_volume_mount_point(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _hFindVolumeMountPoint = api_try!(emu, call.arg_u32(emu, 0));
    let _lpszVolumeMountPoint = api_try!(emu, call.arg_u32(emu, 1));
    let _cchBufferLength = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn FindNextVolumeMountPointW(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointW");
}
