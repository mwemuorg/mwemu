use crate::emu;

pub fn LocalSystemTimeToLocalFileTime(emu: &mut emu::Emu) {
    let cal_id = emu.regs().rcx;
    let p_cal_date_time = emu.regs().rdx;
    let p_local_file_time = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!LocalSystemTimeToLocalFileTime calId:{} pCal:0x{:x} pTime:0x{:x}",
        cal_id,
        p_cal_date_time,
        p_local_file_time
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
