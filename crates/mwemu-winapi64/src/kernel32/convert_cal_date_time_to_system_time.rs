use crate::emu;

pub fn ConvertCalDateTimeToSystemTime(emu: &mut emu::Emu) {
    let p_cal = emu.regs().rcx;
    let p_sys_time = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!ConvertCalDateTimeToSystemTime pCal:0x{:x} pSysTime:0x{:x}",
        p_cal,
        p_sys_time
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
