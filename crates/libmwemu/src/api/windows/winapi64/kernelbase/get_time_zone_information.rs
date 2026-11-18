use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tz_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetTimeZoneInformation ptr:0x{:x}", tz_ptr);
    if tz_ptr != 0 {
        emu.maps.write_dword(tz_ptr, 0i32 as u32); // Bias = 0 (UTC)
        for i in 4..172u64 {
            emu.maps.write_byte(tz_ptr + i, 0);
        }
    }
    call.set_return(emu, 0); // TIME_ZONE_ID_UNKNOWN
}
