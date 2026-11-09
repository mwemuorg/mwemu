use crate::emu;

pub fn GetTimeZoneInformation(emu: &mut emu::Emu) {
    let tz_ptr = emu.regs().rcx;
    log_red!(emu, "kernelbase!GetTimeZoneInformation ptr:0x{:x}", tz_ptr);
    if tz_ptr != 0 {
        emu.maps.write_dword(tz_ptr, 0i32 as u32); // Bias = 0 (UTC)
        for i in 4..172u64 {
            emu.maps.write_byte(tz_ptr + i, 0);
        }
    }
    emu.regs_mut().rax = 0; // TIME_ZONE_ID_UNKNOWN
}
