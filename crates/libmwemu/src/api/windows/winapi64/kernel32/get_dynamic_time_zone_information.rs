use crate::emu;

pub fn GetDynamicTimeZoneInformation(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!GetDynamicTimeZoneInformation ptr:0x{:x}",
        ptr
    );

    if ptr != 0 {
        // DYNAMIC_TIME_ZONE_INFORMATION
        // Bias (LONG) at offset 0
        emu.maps.write_dword(ptr, 0); // UTC bias = 0 (UTC)
        // StandardName (WCHAR[32]) at offset 4 - 68 bytes
        emu.maps.write_wide_string(ptr + 4, "UTC");
        // StandardDate (SYSTEMTIME) at offset 68 - 16 bytes
        for i in 0..8 {
            emu.maps.write_word(ptr + 68 + i * 2, 0);
        }
        // StandardBias (LONG) at offset 84
        emu.maps.write_dword(ptr + 84, 0);
        // DaylightName (WCHAR[32]) at offset 88 - 64 bytes
        emu.maps.write_wide_string(ptr + 88, "UTC");
        // DaylightDate (SYSTEMTIME) at offset 152 - 16 bytes
        for i in 0..8 {
            emu.maps.write_word(ptr + 152 + i * 2, 0);
        }
        // DaylightBias (LONG) at offset 168
        emu.maps.write_dword(ptr + 168, 0);
        // TimeZoneKeyName (WCHAR[128]) at offset 172
        emu.maps.write_wide_string(ptr + 172, "UTC");
        // DynamicDaylightTimeDisabled (BOOLEAN) at offset 428
        emu.maps.write_byte(ptr + 428, 1);
    }

    // Return TIME_ZONE_ID_UNKNOWN = 0
    emu.regs_mut().rax = 0;
}
