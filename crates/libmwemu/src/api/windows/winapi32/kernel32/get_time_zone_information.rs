use crate::emu;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let out_timeZoneInfo = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetTimeZoneInformation cannot read out_timeZoneInfo");

    //TODO: new structure https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information

    log_red!(emu, "kernel32!GetTimeZoneInformation");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // TIME_ZONE_ID_STANDARD
}
