use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_timeZoneInfo = api_try!(emu, call.arg_u32(emu, 0));

    //TODO: new structure https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information

    log_red!(emu, "kernel32!GetTimeZoneInformation");

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1); // TIME_ZONE_ID_STANDARD
}
