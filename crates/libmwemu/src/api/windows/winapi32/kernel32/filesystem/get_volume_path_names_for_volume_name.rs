use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetVolumePathNamesForVolumeNameA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpszVolumeName = api_try!(emu, call.arg_u32(emu, 0));
    let lpszVolumePathNames = api_try!(emu, call.arg_u32(emu, 1));
    let _cchBufferLength = api_try!(emu, call.arg_u32(emu, 2));
    let _lpcchReturnLength = api_try!(emu, call.arg_u32(emu, 3));

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);
    let lpszVolumePathNames = emu.maps.read_string(lpszVolumePathNames as u64);

    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameA {} {}",
        lpszVolumeName,
        lpszVolumePathNames
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn GetVolumePathNamesForVolumeNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameW '{}'  0x{:x}",
        s,
        arg2
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}
