use crate::emu;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let _out_timeZoneInfo = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!GetTimeZoneInformation ptr:0x{:x}",
        _out_timeZoneInfo
    );

    //TODO: Write TIME_ZONE_INFORMATION structure if needed
    // Assuming out_timeZoneInfo points to valid primitive buffer.

    emu.regs_mut().rax = 1; // TIME_ZONE_ID_STANDARD
}
