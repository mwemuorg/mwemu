use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetGeoInfoA(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoA");
}

fn get_geo_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _Location = api_try!(emu, call.arg_u32(emu, 0));
    let _GeoType = api_try!(emu, call.arg_u32(emu, 1));
    let _lpGeoData = api_try!(emu, call.arg_u32(emu, 2));
    let _cchData = api_try!(emu, call.arg_u32(emu, 3));
    let _LangId = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn GetGeoInfoW(emu: &mut emu::Emu) {
    get_geo_info(emu, "GetGeoInfoW");
}
