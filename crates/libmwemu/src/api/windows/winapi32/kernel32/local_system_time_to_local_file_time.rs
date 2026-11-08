use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LocalSystemTimeToLocalFileTime(emu: &mut emu::Emu) {
    let cal_id = arg(emu, 0) as u64;
    let p_cal_date_time = arg(emu, 1) as u64;
    let p_local_file_time = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!LocalSystemTimeToLocalFileTime calId:{} pCal:0x{:x} pTime:0x{:x}",
        cal_id,
        p_cal_date_time,
        p_local_file_time
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
