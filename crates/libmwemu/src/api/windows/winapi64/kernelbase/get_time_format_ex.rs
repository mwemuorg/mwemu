use crate::emu;

pub fn GetTimeFormatEx(emu: &mut emu::Emu) {
    let _locale_ptr = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let _time_ptr = emu.regs().r8;
    let _format_ptr = emu.regs().r9;
    let buf_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let time = "12:00:00";
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, time);
    }
    log_red!(emu, "kernelbase!GetTimeFormatEx => '{}'", time);
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (time.len() + 1) as u64;
}
