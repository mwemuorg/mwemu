use crate::emu;

pub fn GetDateFormatA(emu: &mut emu::Emu) {
    let _locale = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let _date_ptr = emu.regs().r8;
    let _format_ptr = emu.regs().r9;
    let buf_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let date = "11/01/2026";
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_string(buf_ptr, date);
    }
    log_red!(emu, "kernelbase!GetDateFormatA => '{}'", date);
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (date.len() + 1) as u64;
}

pub fn GetDateFormatW(emu: &mut emu::Emu) {
    let buf = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let date = "11/01/2026";
    log_red!(emu, "kernelbase!GetDateFormatW -> '{}'", date);
    if buf != 0 && buf_size > 0 {
        for (i, ch) in date.encode_utf16().enumerate() {
            emu.maps.write_word(buf + (i as u64) * 2, ch);
        }
        emu.maps.write_word(buf + (date.len() as u64) * 2, 0);
    }
    emu.regs_mut().rax = (date.len() + 1) as u64;
}
