use crate::emu;

pub fn GetTimeZoneInformationForYear(emu: &mut emu::Emu) {
    let year = emu.regs().rcx as u16;
    let dti_ptr = emu.regs().rdx;
    let tzi_ptr = emu.regs().r8;

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
}
