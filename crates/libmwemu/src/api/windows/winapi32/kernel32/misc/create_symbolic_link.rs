use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpSymlinkFileName = api_try!(emu, call.arg_u32(emu, 0));
    let lpTargetFileName = api_try!(emu, call.arg_u32(emu, 1));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 2));

    let lpSymlinkFileName = emu.maps.read_string(lpSymlinkFileName as u64);
    let lpTargetFileName = emu.maps.read_string(lpTargetFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateSymbolicLinkA {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn CreateSymbolicLinkW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!CreateSymbolicLinkW '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}
