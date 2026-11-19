use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn EnumSystemGeoID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _GeoClass = api_try!(emu, call.arg_u32(emu, 0));
    let _ParentGeoId = api_try!(emu, call.arg_u32(emu, 1));
    let _lpGeoEnumProc = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!EnumSystemGeoID");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
