use crate::emu;

pub fn ConvertSystemTimeToCalDateTime(emu: &mut emu::Emu) {
    let p_sys_time = emu.regs().rcx;
    let cal_id = emu.regs().rdx;
    let p_cal = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!ConvertSystemTimeToCalDateTime pSysTime:0x{:x} calId:{} pCal:0x{:x}",
        p_sys_time,
        cal_id,
        p_cal
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
