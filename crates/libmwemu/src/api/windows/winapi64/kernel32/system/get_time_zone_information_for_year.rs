use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTimeZoneInformationForYear(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let year = api_try!(emu, call.arg_u16(emu, 0));
    let dti_ptr = api_try!(emu, call.arg(emu, 1));
    let tzi_ptr = api_try!(emu, call.arg(emu, 2));

    log_red!(
        emu,
        "kernel32!GetTimeZoneInformationForYear year:{} dti:0x{:x} tzi:0x{:x}",
        year,
        dti_ptr,
        tzi_ptr
    );

    if tzi_ptr != 0 {
        // TIME_ZONE_INFORMATION: Bias=0 (UTC), rest zeroed (172 bytes total)
        for i in 0..22 {
            emu.maps.write_qword(tzi_ptr + i * 8, 0);
        }
    }

    call.set_return(emu, 1); // TRUE
}
