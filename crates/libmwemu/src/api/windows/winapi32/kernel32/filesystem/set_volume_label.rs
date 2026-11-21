use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetVolumeLabelA(emu: &mut emu::Emu) {
    set_volume_label(emu, "SetVolumeLabelA", false);
}

fn set_volume_label(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpRootPathName = api_try!(emu, call.arg_u32(emu, 0));
    let lpVolumeName = api_try!(emu, call.arg_u32(emu, 1));

    let lpRootPathName = strings::read_text(emu, lpRootPathName as u64, wide);
    let lpVolumeName = strings::read_text(emu, lpVolumeName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpRootPathName, lpVolumeName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn SetVolumeLabelW(emu: &mut emu::Emu) {
    set_volume_label(emu, "SetVolumeLabelW", true);
}
