use crate::emu;

pub fn LocalFileTimeToLocalSystemTime(emu: &mut emu::Emu) {
    let p_local_file_time = emu.regs().rcx;
    let cal_id = emu.regs().rdx;
    let p_cal_date_time = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!LocalFileTimeToLocalSystemTime pTime:0x{:x} calId:{} pCal:0x{:x}",
        p_local_file_time,
        cal_id,
        p_cal_date_time
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
