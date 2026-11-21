use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetVolumeMountPointA(emu: &mut emu::Emu) {
    set_volume_mount_point(emu, "SetVolumeMountPointA", false);
}

fn set_volume_mount_point(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _lpszVolumeMountPoint = api_try!(emu, call.arg_u32(emu, 0));
    let lpszVolumeName = api_try!(emu, call.arg_u32(emu, 1));

    let lpszVolumeName = strings::read_text(emu, lpszVolumeName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszVolumeName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn SetVolumeMountPointW(emu: &mut emu::Emu) {
    set_volume_mount_point(emu, "SetVolumeMountPointW", true);
}
