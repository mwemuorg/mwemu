use crate::emu;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let tz_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetTimeZoneInformation ptr:0x{:x}", tz_ptr);

    if tz_ptr == 0 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0xFFFFFFFF; // TIME_ZONE_ID_INVALID
        return;
    }

    // TIME_ZONE_INFORMATION is 172 bytes:
    //   LONG Bias (4 bytes)
    //   WCHAR StandardName[32] (64 bytes)
    //   SYSTEMTIME StandardDate (16 bytes)
    //   LONG StandardBias (4 bytes)
    //   WCHAR DaylightName[32] (64 bytes)
    //   SYSTEMTIME DaylightDate (16 bytes)
    //   LONG DaylightBias (4 bytes)

    // Zero the whole structure
    for off in (0u64..172).step_by(4) {
        emu.maps.write_dword(tz_ptr + off, 0);
    }

    // Bias = 0 (UTC)
    emu.maps.write_dword(tz_ptr, 0);

    // StandardName = "UTC" at offset 4
    for (i, ch) in "UTC".encode_utf16().enumerate() {
        emu.maps.write_word(tz_ptr + 4 + (i as u64) * 2, ch);
    }

    // StandardBias at offset 84 = 0
    emu.maps.write_dword(tz_ptr + 84, 0);

    // DaylightName = "UTC" at offset 88
    for (i, ch) in "UTC".encode_utf16().enumerate() {
        emu.maps.write_word(tz_ptr + 88 + (i as u64) * 2, ch);
    }

    // DaylightBias at offset 168 = 0
    emu.maps.write_dword(tz_ptr + 168, 0);

    // Return TIME_ZONE_ID_UNKNOWN (0) = no DST info
    emu.regs_mut().rax = 0;
}
