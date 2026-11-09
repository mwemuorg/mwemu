use crate::emu;

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
