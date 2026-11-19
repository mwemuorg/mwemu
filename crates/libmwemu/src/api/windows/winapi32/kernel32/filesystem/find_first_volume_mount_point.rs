use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindFirstVolumeMountPointA(emu: &mut emu::Emu) {
    find_first_volume_mount_point(emu, "FindFirstVolumeMountPointA", false);
}

fn find_first_volume_mount_point(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpszRootPathName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpszVolumeMountPoint = api_try!(emu, call.arg_u32(emu, 1));
    let _cchBufferLength = api_try!(emu, call.arg_u32(emu, 2));

    let lpszRootPathName = strings::read_text(emu, lpszRootPathName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszRootPathName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn FindFirstVolumeMountPointW(emu: &mut emu::Emu) {
    find_first_volume_mount_point(emu, "FindFirstVolumeMountPointW", true);
}
