use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let out_timeZoneInfo = WinApiAbi::arg(emu, 0) as u32;

    //TODO: new structure https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information

    log_red!(emu, "kernel32!GetTimeZoneInformation");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // TIME_ZONE_ID_STANDARD
}
