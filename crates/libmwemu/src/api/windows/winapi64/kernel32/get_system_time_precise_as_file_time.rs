use crate::emu;

pub fn GetSystemTimePreciseAsFileTime(emu: &mut emu::Emu) {
    let file_time_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemTimePreciseAsFileTime");

    let ft: u64 = 132500000000000000;
    emu.maps.write_qword(file_time_ptr, ft);
}
