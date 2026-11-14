use crate::emu;

pub fn SystemTimeToTzSpecificLocalTime(emu: &mut emu::Emu) {
    let p_tz_info = emu.regs().rcx;
    let p_universal = emu.regs().rdx;
    let p_local = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!SystemTimeToTzSpecificLocalTime tz:0x{:x} utc:0x{:x} local:0x{:x}",
        p_tz_info,
        p_universal,
        p_local
    );

    if p_universal == 0 || p_local == 0 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
        return;
    }

    // Copy the 16-byte SYSTEMTIME from universal to local (UTC=local for Bias=0)
    for off in (0..16).step_by(2) {
        let val = emu.maps.read_word(p_universal + off).unwrap_or(0);
        emu.maps.write_word(p_local + off, val);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
