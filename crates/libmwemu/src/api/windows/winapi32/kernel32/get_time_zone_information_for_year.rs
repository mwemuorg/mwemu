use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetTimeZoneInformationForYear(emu: &mut emu::Emu) {
    let year = arg(emu, 0) as u64 as u16;
    let dti_ptr = arg(emu, 1) as u64;
    let tzi_ptr = arg(emu, 2) as u64;

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

    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 3);
}
