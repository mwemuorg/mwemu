use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FileTimeToLocalFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpFileTime = api_try!(emu, call.arg(emu, 0));
    let out_lpLocalFileTime = api_try!(emu, call.arg(emu, 1));

    let dwLowDateTime = emu.maps.read_dword(lpFileTime).unwrap_or(0);
    let dwHighDateTime = emu.maps.read_dword(lpFileTime + 4).unwrap_or(0);

    emu.maps.write_dword(out_lpLocalFileTime, dwLowDateTime);
    emu.maps
        .write_dword(out_lpLocalFileTime + 4, dwHighDateTime);

    log_red!(
        emu,
        "kernel32!FileTimeToLocalFileTime {} {}",
        dwLowDateTime,
        dwHighDateTime
    );

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
