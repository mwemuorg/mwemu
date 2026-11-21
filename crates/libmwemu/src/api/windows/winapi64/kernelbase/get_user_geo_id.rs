use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetUserGeoID(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let geo_class = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetUserGeoID class:{}", geo_class);
    let _ = emu.maps.is_mapped(geo_class);
    emu.set_last_error(0);
    call.set_return(emu, 244);
}
